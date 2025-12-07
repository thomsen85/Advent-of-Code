use common::strings::string_to_char_grid;
use std::{collections::HashSet, time::Instant};

fn main() {
    let now = Instant::now();
    let res = solve(include_str!("../../inputs/day7.txt"));
    let elapsed = now.elapsed();
    dbg!(res);
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let g = string_to_char_grid(input);

    let start_col = g[0].iter().position(|c| *c == 'S').unwrap();

    let mut stack = vec![(0, start_col)];
    let mut visited = HashSet::new();
    let mut split = 0;
    while let Some((row, col)) = stack.pop() {
        if visited.contains(&(row, col)) || row >= g.len() {
            continue;
        }
        visited.insert((row, col));

        if g[row][col] == '^' {
            split += 1;
            if col > 0 {
                stack.push((row, col - 1));
            }
            if col < g[0].len() {
                stack.push((row, col + 1));
            }
        } else {
            stack.push((row + 1, col))
        }
    }

    split.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(solve(ti), "21".to_string());
    }
}
