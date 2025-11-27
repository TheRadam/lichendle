use std::process::{Command, ExitStatus};
use std::time::SystemTime;
use libsql::{Builder};
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<libsql::Error>> {
    let start = SystemTime::now();
    let url = std::env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let token = std::env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");

    let mut rows;
    {
        let db = Builder::new_remote(url, token)
            .build()
            .await?;
        let conn = db.connect()?;

        let lichenised_id = select_random_id();

        rows = match conn
            .query("SELECT photos.photo_id, taxa.name FROM observations_l JOIN photos ON photos.observation_uuid == observations_l.observation_uuid JOIN taxa ON taxa.taxon_id == observations_l.taxon_id WHERE lichenised_id == ?1 LIMIT 1", [lichenised_id])
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

    get_image(*row.get_value(0).unwrap().as_integer().unwrap() as u32);


    println!("Time Taken: {}ms", start.elapsed().unwrap().as_millis());
    Ok(())
}

fn select_random_id() -> u32 {
    rand::random_range(1..=2372981) as u32
}

fn get_image(id: u32) -> ExitStatus {
    let jpeg_regex = Regex::new(r".jpeg").unwrap();
    let list = Command::new("aws")
        .arg("s3")
        .arg("ls")
        .arg("--no-sign-request")
        .arg(format!("s3://inaturalist-open-data/photos/{}/", id))
        .output()
        .expect("Failed to list files in dir");


    //some are jps, some are jpegs
    let extension = match jpeg_regex.is_match(String::from_utf8(list.stdout).unwrap().as_str()) {
        true => { "jpeg" }
        false => { "jpg" }
    };

    Command::new("aws")
        .arg("s3")
        .arg("cp")
        .arg("--no-sign-request")
        .arg(format!("s3://inaturalist-open-data/photos/{}/original.{}", id, extension))
        .arg(".")
        .status()
        .expect("Failed to download image")
}