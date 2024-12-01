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
    dbg!(solve(include_str!("../../inputs/day1.txt")));
}

fn solve(input: &str) -> String {
    let mut first = Vec::new();
    let mut second = Vec::new();
    input
        .lines()
        .map(|l| dbg!(l.split_whitespace().collect_vec()))
        .for_each(|v| {
            let a = v[0];
            let b = v[1];
            first.push(a);
            second.push(b);
        });
    first.sort();
    second.sort();
    first
        .into_iter()
        .zip(second.into_iter())
        .map(|(a, b)| (a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap()).abs())
        .sum::<i32>()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, ()> {
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "***input***";
        assert_eq!(solve(ti), "***output***".to_string());
    }
}
