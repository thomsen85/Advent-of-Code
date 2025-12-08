use std::time::Instant;

use common::strings::string_to_char_grid;

// Delta time: 4 min 34 sek
fn main() {
    let now = Instant::now();
    let res = solve(include_str!("../../inputs/day5.txt"));
    let elapsed = now.elapsed();
    dbg!(res);
    println!("Time used {elapsed:?}");
}
fn solve(input: &str) -> String {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .split("\n")
        .map(|l| l.split_once("-").unwrap())
        .map(|(a, b)| a.parse::<usize>().unwrap()..=b.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    dbg!(ids);
    ids.trim()
        .split("\n")
        .map(|l| {
            l.parse::<usize>()
                .unwrap_or_else(|_| panic!("{} not a number", l))
        })
        .map(|n| {
            if ranges.iter().any(|range| range.contains(&n)) {
                1
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(solve(ti), "3".to_string());
    }
}
