use std::fs;
use std::process::{Command};
use std::time::SystemTime;
use libsql::{Builder};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<libsql::Error>> {
    let start = SystemTime::now();
    let url = std::env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");

    let mut rows;
    {
        let db = match std::env::var("DEV_MODE").unwrap().as_str() {
            "0" => {
                let token = std::env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");
                Builder::new_remote(url, token).build().await?
            }
            _ => Builder::new_local(url).build().await?
        };

        let conn = db.connect()?;

        let relative_id = select_random_id();

        rows = match conn
            .query("SELECT photos.photo_id, taxa.name, photos.extension FROM observations JOIN photos ON photos.observation_uuid == observations.observation_uuid JOIN taxa ON taxa.taxon_id == observations.taxon_id WHERE relative_id == ?1 LIMIT 1", [relative_id])
            .await {
            Ok(rows) => { rows },
            Err(err) => return Err(Box::new(err)),
        };
        conn.reset().await;
    }
    let row = rows.next().await?.unwrap();

    for i in 0..row.column_count() {
        println!("Row {}: {:?}", i, row.get_value(i).unwrap());
    }

    let extension = row.get_value(2).unwrap();
    let image = get_image(*row.get_value(0).unwrap().as_integer().unwrap() as u32, extension.as_text().unwrap().clone());
    let name = row.get_value(1).unwrap();

    generate_page(image, name.as_text().unwrap().clone());

    println!("Time Taken: {}ms", start.elapsed().unwrap().as_millis());
    Ok(())
}

fn select_random_id() -> u32 {
    rand::random_range(1..=414405) as u32
}

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

fn generate_page(file_name: String, taxon_name: String) {
    let path = Path::new("template.html");
    let path_new_file = Path::new("html/index.html");
    let mut file = File::create(path_new_file).unwrap();
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let image_src = Regex::new(r"#IMAGE#").unwrap();
    let name = Regex::new(r"#NAME#").unwrap();
    let x = image_src.replace(contents.as_str(), file_name);
    let y = name.replace(x.as_ref(), taxon_name);

    file.write_all(y.as_bytes()).expect("Should have been able to write file");
}
