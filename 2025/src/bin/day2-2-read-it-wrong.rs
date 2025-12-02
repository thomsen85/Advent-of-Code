// I managed to think it was any repeating substring in any number was invalid, bit to fast on the
// trigger there. But here is that solution...
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
    let mut search_space = 0;
    for r in input.replace("\n", "").split(",") {
        let (from, to) = r.split_once("-").unwrap();
        dbg!(from, to);
        let from: usize = from.parse().unwrap();
        let to: usize = to.parse().unwrap();
        search_space += to - from;
        ranges.push(from..=to);

        max = max.max(to);
    }
    dbg!(max);
    dbg!(search_space);

    let max_n = max.to_string().len() + 1;

    let mut sum = 0;
    for r in ranges {
        for i in r {
            if is_invalid(i) {
                dbg!(i);
                sum += i
            }
        }
    }

    sum.to_string()
}

fn is_invalid(n: usize) -> bool {
    let len = n.to_string().len() / 2;
    let n_s = n.to_string();

    for spacing in 1..=len {
        let mut set = HashSet::new();
        let mut p1 = 0;
        let mut p2 = spacing;

        for _ in spacing..=n.to_string().len() {
            let s = &n_s[p1..p2];
            if set.contains(s) {
                return true;
            }
            set.insert(s);
            p1 += 1;
            p2 += 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        assert_eq!(solve(ti), "4174379265".to_string());
    }
}
