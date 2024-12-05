use std::collections::{HashMap, VecDeque};

use common::{graphs::bi_directional_map::BiDirectionalMap, strings::string_to_t_vec};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
// For number types
use nom::character::complete as cnom;

fn main() {
    dbg!(solve(include_str!("../../inputs/day5.txt")));
}

fn solve(input: &str) -> String {
    let (l, p) = input.split_once("\n\n").unwrap();

    let mut map = HashMap::new();
    let mut map_r = HashMap::new();

    for line in l.lines() {
        let (key, val) = line.split_once("|").unwrap();
        let key = key.parse::<i32>().unwrap();
        let val = val.parse::<i32>().unwrap();
        map.entry(key).or_insert(Vec::new()).push(val);
        map_r.entry(val).or_insert(Vec::new()).push(key);
    }

    let mut incorrect = Vec::new();
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
        if !valid {
            incorrect.push(nums);
        }
    }

    let mut safe = 0;
    for inc in incorrect {
        let mut new_order: VecDeque<i32> = VecDeque::new();
        let mut inc_c = inc.clone();
        let mut i = 0;
        loop {
            let c = inc_c[i];

            let must_come_before = map_r.get(&c).map(|a| a.to_owned()).unwrap_or(Vec::new());

            if must_come_before
                .iter()
                .all(|a| new_order.contains(a) || !inc_c.contains(a))
            {
                let b = inc_c.remove(i);
                new_order.push_front(b);
            }
            if inc_c.is_empty() {
                break;
            }
            i = (i + 1) % inc_c.len();
        }

        let mid = new_order.len() / 2;
        safe += new_order.get(mid).unwrap();
    }

    safe.to_string()
}

fn parse(input: &str) -> IResult<&str, ()> {
    Ok((input, ()))
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
        assert_eq!(solve(ti), "123".to_string());
    }
}
