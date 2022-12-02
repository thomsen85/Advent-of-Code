use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let file = File::open("inputs/day1.txt").unwrap();
    // let input = BufReader::new(file).lines().into_iter().fold("".to_string(),|acc, x| acc.to_string() + x.unwrap().to_string());

    let input = BufReader::new(file).lines().into_iter().map(|a| a.unwrap()).collect::<Vec<String>>();
    let mut acc = 0;
    let mut res = Vec::new();
    for entry in input {
        if !entry.is_empty() {
            acc += entry.parse::<i32>().unwrap();
        } else {
            res.push(acc);
            acc = 0;
        }
    }
    res.sort();
    let ans: i32 = res[res.len()-3..].iter().sum();
    println!("{:?}", ans);

}
