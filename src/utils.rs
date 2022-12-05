use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn lines_from_file(path: &str) -> Vec<String> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .into_iter()
        .map(|a| a.unwrap().trim().to_owned())
        .collect::<Vec<String>>()
}
