use std::{collections::VecDeque, error::Error};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};
// For number types
use nom::character::complete as cnom;

fn main() {
    dbg!(solve(include_str!("../../inputs/day9.txt")));
}

fn solve(input: &str) -> String {
    let (_, lines) = parse(input).unwrap();

    let mut sum = 0;
    for line in lines {
        let mut i = 0;
        let mut diff_lines: Vec<VecDeque<i32>> = Vec::new();
        diff_lines.push(line);

        loop {
            diff_lines.push(
                diff_lines[i]
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect(),
            );

            i += 1;

            if diff_lines[i].iter().all(|a| *a == 0) {
                break;
            }
        }

        diff_lines.last_mut().unwrap().push_front(0);

        let mut c = diff_lines.len() - 1;
        loop {
            if c <= 0 {
                break;
            }
            let l1 = *diff_lines[c].iter().next().unwrap();
            let l2 = *diff_lines[c - 1].iter().next().unwrap();

            diff_lines[c - 1].push_front(l2 - l1);

            c -= 1;
        }

        sum += diff_lines[0].iter().next().unwrap();
    }

    sum.to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<VecDeque<i32>>> {
    let (input, lines) = separated_list1(
        newline,
        map(separated_list1(space1, cnom::i32), VecDeque::from),
    )(input)?;

    Ok((input, lines))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(solve(ti), "2".to_string());
    }
}
