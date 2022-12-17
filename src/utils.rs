use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

pub fn lines_from_file(path: &str) -> Vec<String> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .into_iter()
        .map(|a| a.unwrap().trim().to_owned())
        .collect::<Vec<String>>()
}

pub fn string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn paragraph_from_file(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .into_iter()
        .map(|a| a.trim().to_owned())
        .filter(|a| !a.is_empty())
        .collect::<Vec<String>>()
}
