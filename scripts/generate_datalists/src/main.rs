use std::{io};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Write};
use std::path::Path;

fn main() {
    generate_datalists("species.csv", "species.html");
}

fn generate_datalists(input_file: &str, output_file: &str) {
    let path_new_file = Path::new(output_file);
    let mut file = File::create(path_new_file).unwrap();

    if let Ok(lines) = read_lines(input_file) {
        for line in lines.map_while(Result::ok) {
            let markup = format!("<option value=\"{}\"></option>\n",line);
            file.write(markup.as_bytes()).expect("Should have been able to write file");
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}