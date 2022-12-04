extern crate aoc2022_rust;
use aoc2022_rust::utils;

fn main() {
    let input = utils::lines_from_file("inputs/day2.txt");

    let total_points = input.iter().fold(0, |acc, x| acc + points_from_line(x));

    println!("Points is {}", total_points);
}

fn points_from_line(line: &str) -> usize {
    let split: Vec<&str> = line.trim().split(' ').collect();
    let opp = split[0];
    let me = split[1];

    let mut points = 0;

    if opp == "A" {
        if me == "X" {
            points += 1 + 3;
        } else if me == "Y" {
            points += 2 + 6;
        } else if me == "Z" {
            points += 3 + 0;
        }
    } else if opp == "B" {
        if me == "X" {
            points += 1 + 0;
        } else if me == "Y" {
            points += 2 + 3;
        } else if me == "Z" {
            points += 3 + 6;
        }
    } else if opp == "C" {
        if me == "X" {
            points += 1 + 6;
        } else if me == "Y" {
            points += 2 + 0;
        } else if me == "Z" {
            points += 3 + 3;
        }
    }

    points
}
