use std::ops::RangeBounds;

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
    dbg!(solve(include_str!("../../inputs/day2.txt")));
}

fn solve(input: &str) -> String {
    let mut safe = 0;
    for line in input.lines() {
        let mut all_nums = Vec::new();
        let nums = line
            .split(" ")
            .map(|a| a.parse::<i32>().unwrap())
            .collect_vec();
        all_nums.push(nums.clone());
        for i in 0..nums.len() {
            let mut n = nums.clone();
            n.remove(i);
            all_nums.push(n);
        }

        for nums in all_nums {
            let increasing = nums.iter().tuple_windows().map(|(a, b)| a < b).all(|a| a);
            let decreasing = nums.iter().tuple_windows().map(|(a, b)| a > b).all(|a| a);
            let within_nums = nums
                .iter()
                .tuple_windows()
                .map(|(a, b)| a.abs_diff(*b))
                .all(|a| (0..=3).contains(&a));

            if (increasing || decreasing) && within_nums {
                safe += 1;
                break;
            }
        }
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
    fn test_2() {
        let ti = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(solve(ti), "4".to_string());
    }
}
