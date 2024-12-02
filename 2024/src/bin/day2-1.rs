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
    let mut reports = 0;
    for line in input.lines() {
        let nums = line
            .split(" ")
            .map(|a| dbg!(a).parse::<i32>().unwrap())
            .collect_vec();
        if nums.iter().tuple_windows().map(|(a, b)| a < b).all(|a| a)
            || nums.iter().tuple_windows().map(|(a, b)| a < b).all(|a| !a)
        {
            if nums
                .iter()
                .tuple_windows()
                .map(|(a, b)| (a - b).abs())
                .all(|a| a >= 1 && a <= 3)
            {
                reports += 1;
            }
        }
    }
    reports.to_string()
}

fn parse(input: &str) -> IResult<&str, ()> {
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(solve(ti), "2".to_string());
    }
}
