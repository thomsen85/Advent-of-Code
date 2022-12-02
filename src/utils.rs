use std::{fs::File, io::{BufReader, BufRead}};


pub fn lines_from_file(path: &str) -> Vec<String> {    
    let file = File::open(path).unwrap();
    BufReader::new(file).lines().into_iter().map(|a| a.unwrap()).collect::<Vec<String>>()
}