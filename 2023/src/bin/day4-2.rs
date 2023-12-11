#![allow(dead_code)]

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    solve(include_str!("../../inputs/day4.txt"));
}

fn solve(input: &str) -> usize {
    let mut card_number = 0;
    let mut instances: Vec<usize> = (0..input.lines().count()).into_iter().map(|_| 1).collect();

    for line in input.lines() {
        dbg!(line);
        let (_, (a, b)) = parse_line(line).unwrap();
        let points: usize = b
            .into_iter()
            .map(|p| if a.contains(&p) { 1 } else { 0 })
            .sum::<usize>();

        dbg!(&instances);
        for i in 1..=points {
            if instances.len() > card_number + i {
                instances[card_number + i] += instances[card_number];
            }
        }

        dbg!(&instances);
        card_number += 1;
    }
    dbg!(&instances.iter().sum::<usize>());

    instances.iter().sum()
}

fn parse_line(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, (_id, cards)) = separated_pair(
        preceded(
            tag("Card"),
            preceded(multispace1, nom::character::complete::u32),
        ),
        tag(":"),
        separated_pair(
            preceded(
                multispace1,
                separated_list1(multispace1, nom::character::complete::u32),
            ),
            tag(" | "),
            preceded(
                multispace0,
                separated_list1(multispace1, nom::character::complete::u32),
            ),
        ),
    )(input)?;
    Ok((input, cards))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(solve(input), 30);
    }
}
