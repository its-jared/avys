use std::{fs::File, io::{BufRead, BufReader}};
use rand::RngExt;

pub fn get_splash_text() -> std::io::Result<String> {
    let mut rng = rand::rng();
    let mut lines: Vec<String> = Vec::new();
    let file = File::open("assets/data/splash_text.txt")
        .expect("Unable to read `assets/data/splash_text.txt`, the file is not found.");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        lines.push(line?);
    }

    let splash_text = lines.get(rng.random_range(0..lines.len())).unwrap();
    Ok(splash_text.clone())
}