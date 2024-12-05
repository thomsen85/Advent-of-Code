use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    dbg!(solve(include_str!("../../inputs/day5.txt")));
}

fn solve(input: &str) -> String {
    let (l, p) = input.split_once("\n\n").unwrap();

    let mut map_r = HashMap::new();
    let mut safe = 0;

    for line in l.lines() {
        let (key, val) = line.split_once("|").unwrap();
        let key = key.parse::<i32>().unwrap();
        let val = val.parse::<i32>().unwrap();
        map_r.entry(val).or_insert(Vec::new()).push(key);
    }

    for line in p.lines() {
        let nums = line
            .split(",")
            .map(|a| a.parse::<i32>().unwrap())
            .collect_vec();

        let valid = nums.iter().enumerate().all(|(i, c)| {
            !map_r
                .get(c)
                .map(|r| r.iter().any(|n| nums[(i + 1)..].contains(n)))
                .unwrap_or(false)
        });
        if valid {
            let mid = nums.len() / 2;
            safe += nums[mid];
        }
    }

    safe.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(solve(ti), "143".to_string());
    }
}
