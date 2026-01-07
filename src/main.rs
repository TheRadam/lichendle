use std::{fs, io};
use std::process::{Command};
use std::time::SystemTime;
use libsql::{Builder, Connection, Row};
use std::fs::{File};
use std::io::prelude::*;
use std::path::Path;
use lazy_regex::{Regex};

#[tokio::main]
async fn main() -> Result<(), Box<libsql::Error>> {
    let start = SystemTime::now();
    let env = Environment::new();

    let relative_id = select_random_id(env.max);
    println!("relative_id {:?}", relative_id.clone());
    let connection = build_connection(env).await?;
    let row = match get_row(relative_id.clone(), connection).await? {
        Some(row) => row,
        None => panic!("Got no row :( {}", relative_id),
    };

    for i in 0..row.column_count() {
        println!("Row {}: {:?}", i, row.get_value(i)?);
    }

    let constants = read_constants();
    let replacements = generate_replacements(row, constants.1, constants.2);
    let output = File::create("html/index.html").unwrap();

    Page::new(constants.0)
        .populate_page(replacements)
        .write_page_to_file(output)
        .expect("Should have been able to write file");

    copy_dir_all(Path::new("bundle"), Path::new("html")).expect("Couldn't copy bundle");

    println!("Time Taken: {}ms", start.elapsed().unwrap().as_millis());
    Ok(())
}

/// Builds connection based
/// If not dev_mode then use a remote, otherwise use a local DB
async fn build_connection(env: Environment) -> libsql::Result<Connection> {
    let db = match env.token {
        Some(t) => Builder::new_remote(env.url, t).build().await?,
        None => Builder::new_local(env.url).build().await?
    };

    db.connect()
}

/// Selects a random number between 1 and max 414,405
fn select_random_id(max: u32) -> u32 { rand::random_range(1..=max) }

/// Downloads a photo from the iNaturalist open data S3 bucket to 'image.<extension>'
fn get_image(id: u32, extension: String) -> String {
    Command::new("aws")
        .arg("s3")
        .arg("cp")
        .arg("--no-sign-request")
        .arg(format!("s3://inaturalist-open-data/photos/{}/original.{}", id, extension))
        .arg(format!("html/image.{}", extension))
        .status()
        .expect("Failed to download image");

    format!("image.{}", extension)
}

/// Copies the source directory recursively to a destination
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Reads files into memory, the HTML template, genus and species CSVs
fn read_constants() -> (String, String, String) {
    let template = fs::read_to_string("template.html").expect("Should have been able to read template file");
    let genus_list = fs::read_to_string("genus.csv").expect("Should have been able to read genus file");
    let species_list = fs::read_to_string("species.csv").expect("Should have been able to read species file");

    (template, genus_list, species_list)
}

/// Generates a vector of replacements (tuples of Regex, String)
/// So that the vector can be folded into a single Page
fn generate_replacements(row: Row, genus_list: String, species_list: String) -> Vec<(Regex, String)> {

    let taxon_id = *row.get_value(0).expect("Couldn't get image_id").as_integer().unwrap() as u32;
    let taxon_name = row.get_value(1).expect("Couldn't get taxon_name");
    let extension = row.get_value(2).expect("Couldn't get extension");
    let image_path = get_image(taxon_id.clone(), extension.as_text().unwrap().clone());
    let license = row.get_value(3).expect("Couldn't get license");

    let cite = match row.get_value(5).expect("Couldn't get name").is_null() {
        true => row.get_value(4).expect("Couldn't get username (this also logically can't happen)"),
        false => row.get_value(5).expect("Couldn't get name (this logically can't happen")
    };

    let citation = match license.as_text().unwrap().as_str() {
        "CC0" => format!("{}, no rights reserved (CC0)", cite.as_text().unwrap()),
        "PD" => String::from(""),
        _ => format!("© {}, some rights reserved ({})", cite.as_text().unwrap(), license.as_text().unwrap())
    };

    vec![
        (Regex::new(r"#IMAGE#").expect("Won't happen"), image_path),
        (Regex::new(r"#CITATION#").expect("Won't happen"), citation),
        (Regex::new(r"#NAME#").expect("Won't happen"), taxon_name.as_text().unwrap().clone()),
        (Regex::new(r"#GENUS#").expect("Won't happen"), genus_list),
        (Regex::new(r"#SPECIES#").expect("Won't happen"), species_list),
        (Regex::new(r"#TAXON_ID#").expect("Won't happen"), taxon_id.to_string())
    ]
}

async fn get_row(id: u32, connection: Connection) -> libsql::Result<Option<Row>> {
    let mut rows = match connection
        .query("SELECT photos.photo_id, taxa.name, photos.extension, photos.license, observers.login, observers.name FROM observations JOIN photos ON photos.observation_uuid == observations.observation_uuid JOIN taxa ON taxa.taxon_id == observations.taxon_id JOIN observers ON observers.observer_id == photos.observer_id WHERE relative_id == ?1 LIMIT 1", [id]).await {
        Ok(rows) => { rows },
        Err(err) => return Err(err),
    };
    connection.reset().await;
    rows.next().await
}

/// Environment variables passed to the program
/// url: path to a sqlite database
/// token: auth token for a Turso database
struct Environment {
    url: String,
    token: Option<String>,
    max: u32
}

impl Environment {
    fn new() -> Environment {
        let dev_mode = std::env::var("DEV_MODE")
            .or::<String>(Ok(String::from("0")))
            .unwrap();

        let max: u32 = std::env::var("MAX_ID")
            .or::<String>(Ok(String::from("414405")))
            .unwrap()
            .parse()
            .unwrap();

        let url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let token = match dev_mode.as_str() {
            "1" => None,
            _ => Some(std::env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set"))
        };

        println!("max: {}", max);
        println!("dev_mode: {}", dev_mode);

        Environment { url, token, max }
    }
}

/// Represents a webpage to be generated by replacing strings in a template
struct Page {
    contents: String
}

impl Page {
    fn new(contents: String) -> Page {
        Page{contents}
    }

    /// Replaces the first occurrence of a regex with a given replacement string
    ///
    /// Returns a new Page object with the updated contents.
    fn replace(self, regex: Regex, replacement: String) -> Self {
        Self::new(regex.replace(self.contents.as_ref(), replacement).parse().unwrap())
    }

    /// Uses the replace function to replace each regex defined in the replacements Vec with
    /// its paired string.
    fn populate_page(self, replacements: Vec<(Regex, String)>) -> Self {
        replacements
            .iter()
            .fold(self, |acc, replacement| acc.replace(replacement.0.clone(), replacement.1.clone()))
    }

    /// Writes a Page object to a given File.
    fn write_page_to_file(self, mut file: File) -> io::Result<()> {
        file.write_all(self.contents.as_bytes())
    }
}