use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

pub fn lines_from_file(path: &str) -> Vec<String> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|a| a.unwrap().trim().to_owned())
        .collect::<Vec<String>>()
}
pub fn lines_from_string(string: String) -> Vec<String> {
    string
        .lines()
        .map(|a| a.trim().to_owned())
        .collect::<Vec<String>>()
}

pub fn string_from_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn paragraph_from_file(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .map(|a| a.trim().to_owned())
        .filter(|a| !a.is_empty())
        .collect::<Vec<String>>()
}

pub fn transpose_2d_vec<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
