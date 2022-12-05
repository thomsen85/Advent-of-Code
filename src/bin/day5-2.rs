use std::collections::VecDeque;

fn main() {
    let input = aoc2022_rust::utils::lines_from_file("inputs/day5.txt");

    let mut table: Vec<Vec<char>> = Vec::new();

    //     [G] [R]                 [P]
    //     [H] [W]     [T] [P]     [H]
    //     [F] [T] [P] [B] [D]     [N]
    // [L] [T] [M] [Q] [L] [C]     [Z]
    // [C] [C] [N] [V] [S] [H]     [V] [G]
    // [G] [L] [F] [D] [M] [V] [T] [J] [H]
    // [M] [D] [J] [F] [F] [N] [C] [S] [F]
    // [Q] [R] [V] [J] [N] [R] [H] [G] [Z]
    //  1   2   3   4   5   6   7   8   9
    table.push(vec!['Q', 'M', 'G', 'C', 'L']);
    table.push(vec!['R', 'D', 'L', 'C', 'T', 'F', 'H', 'G']);
    table.push(vec!['V', 'J', 'F', 'N', 'M', 'T', 'W', 'R']);
    table.push(vec!['J', 'F', 'D', 'V', 'Q', 'P']);
    table.push(vec!['N', 'F', 'M', 'S', 'L', 'B', 'T']);
    table.push(vec!['R', 'N', 'V', 'H', 'C', 'D', 'P']);
    table.push(vec!['H', 'C', 'T']);
    table.push(vec!['G', 'S', 'J', 'V', 'Z', 'N', 'H', 'P']);
    table.push(vec!['Z', 'F', 'H', 'G']);

    let mut popped = Vec::new();
    for instruction in input {
        let parsed: Vec<&str> = instruction.trim().split_ascii_whitespace().collect();
        let mov: usize = parsed[1].parse().unwrap();
        let from: usize = parsed[3].parse().unwrap();
        let to: usize = parsed[5].parse().unwrap();
        for _ in 0..mov {
            popped.push(table[from - 1].pop().unwrap());
        }
        for _ in 0..mov {
            table[to - 1].push(popped.pop().unwrap());
        }
    }

    println!(
        "{:?}",
        table.iter().fold("".to_string(), |mut acc, x| {
            acc.push(x.last().unwrap().to_owned());
            acc
        })
    )
}
