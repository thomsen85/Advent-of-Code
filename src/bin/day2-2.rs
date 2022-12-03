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

    if opp == "A" { // Rock 1
        if me == "X" { // lose
            points += 0 + 3;
        } else if me == "Y" { // draw
            points += 3 + 1;
        } else if me == "Z" { // win
            points += 6 + 2;
        }
    } else if opp == "B" { // Paper 2
        if me == "X" {
            points += 0 + 1;
        } else if me == "Y" {
            points += 3 + 2;
        } else if me == "Z" {
            points += 6 + 3;
        }
    } else if opp == "C" { // Scissors 3
        if me == "X" {
            points += 0 + 2;
        } else if me == "Y" {
            points += 3 + 3;
        } else if me == "Z" {
            points += 6 + 1;
        }
    }

    points
}