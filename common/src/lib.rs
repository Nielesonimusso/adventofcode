use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::env::var;

pub fn input_file_lines() -> Lines<BufReader<File>> {
    let package_name = var("CARGO_PKG_NAME").expect("invalid package");
    let file = File::open(format!("2022/{package_name}/input.txt")).expect("no read");
    let reader = io::BufReader::new(file);
    reader.lines()
}