use std::{collections::HashSet, ops::RangeInclusive, time::Instant};

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day2.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let mut ranges: Vec<RangeInclusive<usize>> = vec![];

    let mut max = 0;
    for r in input.replace("\n", "").split(",") {
        let (from, to) = r.split_once("-").unwrap();
        dbg!(from, to);
        ranges.push(from.parse().unwrap()..=to.parse().unwrap());

        max = max.max(to.parse::<usize>().unwrap());
    }
    dbg!(max);

    let max_n = max.to_string().len() + 1;

    let invalid_ids = (0..10_usize.pow((max_n / 2) as u32))
        .map(|num| num.to_string())
        .map(|n| (n.clone() + &n).parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    dbg!(invalid_ids.len());

    let mut sum = 0;
    for i_i in &invalid_ids {
        for r in &ranges {
            if r.contains(i_i) {
                sum += i_i;
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        assert_eq!(solve(ti), "1227775554".to_string());
    }
}
