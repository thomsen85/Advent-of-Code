use std::{collections::HashSet, ops::RangeInclusive, time::Instant};

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
    let mut ranges = ranges
        .split("\n")
        .map(|l| l.split_once("-").unwrap())
        .map(|(a, b)| a.parse::<usize>().unwrap()..=b.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut i = 0;
    let mut res = Vec::new();
    while i < ranges.len() {
        let mut current = ranges[i].clone();
        let mut next_index = i + 1;
        while next_index < ranges.len() && ranges[next_index].start() <= current.end() {
            if ranges[next_index].end() > current.end() {
                current = *current.start()..=*ranges[next_index].end();
            }
            next_index += 1;
        }
        res.push(current);
        i = next_index;
    }
    res.iter()
        .map(|r| r.end() - r.start() + 1)
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
        assert_eq!(solve(ti), 14.to_string());
    }

    #[test]
    fn test_2() {
        let ti = "5-8
8-10
4-5
10-12
14-24
3-25
4-20
5-19

1
1
1";
        assert_eq!(solve(ti), 23.to_string())
    }

    #[test]
    fn test_3() {
        let ti = "1-3
4-6
7-9

1
1
1";
        assert_eq!(solve(ti), 9.to_string())
    }
}
