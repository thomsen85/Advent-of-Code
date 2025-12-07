use std::time::Instant;

use common::strings::string_to_char_grid;
use itertools::Itertools;

// Delta 46, didnt relize i needed to align the numbs
fn main() {
    let now = Instant::now();
    let res = solve(include_str!("../../inputs/day6.txt"));
    let elapsed = now.elapsed();
    dbg!(res);
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let grid = string_to_char_grid(input);
    let len = grid.len() - 1;
    let operators = input
        .trim()
        .split("\n")
        .last()
        .unwrap()
        .split_whitespace()
        .collect_vec();

    let mut columns = Vec::new();

    let mut p = 0;

    for i in 0..grid[0].len() {
        // new column

        if (0..len).all(|j| grid[j][i] == ' ') {
            // på ny kolnne
            // p : i
            let mut colum = Vec::new();
            for j in 0..len {
                colum.push(&grid[j][p..i])
            }

            columns.push(colum);
            p = i + 1;
        }
    }
    let mut colum = Vec::new();
    for j in 0..len {
        colum.push(&grid[j][p..])
    }

    columns.push(colum);

    let nums_t_nums = columns
        .iter()
        .map(|column| {
            let max_digit = column.iter().map(|num| num.len()).max().unwrap();

            (0..max_digit)
                .map(|i| {
                    (0..len)
                        .map(|j| column[j][i])
                        .collect::<String>()
                        .trim()
                        .parse::<usize>()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    operators
        .iter()
        .zip(nums_t_nums)
        .map(|(op, arr)| match *op {
            "*" => arr.into_iter().product::<usize>(),
            "+" => arr.into_iter().sum::<usize>(),
            _ => panic!("{} not operator", op),
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(solve(ti), "427755".to_string());
    }
}
