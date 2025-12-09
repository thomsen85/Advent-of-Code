use itertools::Itertools;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let res = solve(include_str!("../../inputs/day9.txt"));
    let elapsed = now.elapsed();
    dbg!(res);
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let coords = input
        .trim()
        .split("\n")
        .map(|l| l.split_once(",").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .collect_vec();

    let mut max = 0;
    for c1 in &coords {
        for c2 in &coords {
            let area = (c2.0.abs_diff(c1.0) + 1) * (c2.1.abs_diff(c1.1) + 1);
            max = max.max(area);
        }
    }
    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(solve(ti), "50".to_string());
    }
}
