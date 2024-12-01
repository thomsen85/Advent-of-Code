use std::ops::RangeBounds;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
// For number types
use nom::character::complete as cnom;

fn main() {
    dbg!(solve(include_str!("../../inputs/day3.txt")));
}

fn solve(input: &str) -> String {
    let mut nums = Vec::new();
    let mut chars = Vec::new();
    for (line_i, line) in input.lines().enumerate() {
        let mut current_num = String::new();
        for (char_i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                current_num.push(c)
            } else {
                if !current_num.is_empty() {
                    nums.push((
                        char_i - current_num.len(),
                        char_i,
                        line_i,
                        current_num.parse::<i32>().unwrap(),
                    ));
                    current_num.clear()
                }

                if c != '.' {
                    chars.push((char_i, line_i))
                }
            }
        }
    }
    dbg!(&nums);

    let mut sum = 0;
    for num in &nums {
        for c in &chars {
            if ((num.0..=num.1).contains(&(c.0 + 1))
                || (num.0..num.1).contains(&(c.0))
                || (num.0..num.1).contains(&(c.0 - 1)))
                && ((c.1 - 1)..=(c.1 + 1)).contains(&num.2)
            {
                sum += num.3
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
        let ti = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(solve(ti), "4361".to_string());
    }
}
