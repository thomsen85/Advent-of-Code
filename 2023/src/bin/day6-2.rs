#![allow(dead_code)]

use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    dbg!(solve(include_str!("../../inputs/day6.txt")));
}

fn solve(input: &str) -> usize {
    let (_, (times, dists)) = parse(input).unwrap();
    let time = times
        .into_iter()
        .fold("".to_string(), |mut acc, x| {
            acc.push_str(&x.to_string());
            acc
        })
        .parse::<i64>()
        .unwrap();

    let dist = dists
        .into_iter()
        .fold("".to_string(), |mut acc, x| {
            acc.push_str(&x.to_string());
            acc
        })
        .parse::<i64>()
        .unwrap();
    dbg!(time, dist);

    let mut distances = Vec::new();

    for hold in 1..time {
        let left = time - hold;
        distances.push(left * hold);
    }

    let amnt = distances.into_iter().filter(|d| *d > dist).count();

    amnt
}

fn parse(input: &str) -> IResult<&str, (Vec<i64>, Vec<i64>)> {
    let (input, (times, dist)) = separated_pair(
        preceded(
            tag("Time: "),
            preceded(
                space1,
                separated_list1(space1, nom::character::complete::i64),
            ),
        ),
        newline,
        preceded(
            tag("Distance:"),
            preceded(
                space0,
                separated_list1(space1, nom::character::complete::i64),
            ),
        ),
    )(input)?;

    Ok((input, (times, dist)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(solve(ti), 71503);
    }
}
