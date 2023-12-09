use std::{cmp::Ordering, error::Error};

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{multispace0, multispace1, newline, space0, space1},
    combinator::map,
    multi::count,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
// For number types
use nom::character::complete as cnom;

fn main() {
    dbg!(solve(include_str!("../../inputs/day7.txt")));
}

#[derive(Debug, Eq)]
struct Hand {
    bid: u32,
    hand: Vec<char>,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand.iter().eq(other.hand.iter())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eq(other) {
            return Some(std::cmp::Ordering::Equal);
        }

        let s_kinds = self
            .hand
            .iter()
            .counts()
            .into_iter()
            .sorted_by(|a, b| b.1.cmp(&a.1))
            .collect_vec();

        let o_kinds = other
            .hand
            .iter()
            .counts()
            .into_iter()
            .sorted_by(|a, b| b.1.cmp(&a.1))
            .collect_vec();

        //dbg!(&s_kinds, &o_kinds);

        if s_kinds[0].1 > 3 || o_kinds[0].1 > 3 {
            if s_kinds[0].1 == o_kinds[0].1 {
                return Some(tight_eq(self, other));
            } else {
                return Some(s_kinds[0].1.cmp(&o_kinds[0].1));
            }
        }

        let s_full_house = s_kinds[0].1 == 3 && s_kinds[1].1 == 2;
        let o_full_house = o_kinds[0].1 == 3 && o_kinds[1].1 == 2;

        if s_full_house && !o_full_house {
            return Some(Ordering::Greater);
        }
        if !s_full_house && o_full_house {
            return Some(Ordering::Less);
        }
        if s_full_house && o_full_house {
            return Some(tight_eq(self, other));
        }

        if s_kinds[0].1 > 2 || o_kinds[0].1 > 2 {
            if s_kinds[0].1 == 3 && o_kinds[0].1 == 3 {
                return Some(tight_eq(self, other));
            }

            return Some(s_kinds[0].1.cmp(&o_kinds[0].1));
        }

        // Two Pair =
        let s_two_p = s_kinds[0].1 == 2 && s_kinds[1].1 == 2;
        let o_two_p = o_kinds[0].1 == 2 && o_kinds[1].1 == 2;

        if s_two_p && !o_two_p {
            return Some(Ordering::Greater);
        }
        if !s_two_p && o_two_p {
            return Some(Ordering::Less);
        }
        if s_two_p && o_two_p {
            return Some(tight_eq(self, other));
        }

        // One Pair =
        let s_one_p = s_kinds[0].1 == 2;
        let o_one_p = o_kinds[0].1 == 2;

        if s_one_p && !o_one_p {
            return Some(Ordering::Greater);
        }
        if !s_one_p && o_one_p {
            return Some(Ordering::Less);
        }
        if s_one_p && o_one_p {
            return Some(tight_eq(self, other));
        }

        return Some(tight_eq(self, other));
    }
}

fn highest_card(hand: &Vec<char>) -> u32 {
    hand.into_iter()
        .map(|b| match *b {
            'A' => 15,
            'K' => 14,
            'Q' => 13,
            'J' => 12,
            'T' => 11,
            _ => b.to_digit(10).unwrap(),
        })
        .max()
        .unwrap()
}

fn tight_eq(selfh: &Hand, other: &Hand) -> std::cmp::Ordering {
    for (a, b) in selfh.hand.iter().zip(other.hand.iter()) {
        if a == b {
            continue;
        }
        if a.is_digit(10) && !b.is_digit(10) {
            return Ordering::Less;
        }

        if !a.is_digit(10) && b.is_digit(10) {
            return Ordering::Greater;
        }

        if a.is_digit(10) && b.is_digit(10) {
            return a.cmp(b);
        }

        let a = match a {
            'A' => 10,
            'K' => 9,
            'Q' => 8,
            'J' => 7,
            'T' => 6,
            _ => panic!(),
        };

        let b = match b {
            'A' => 10,
            'K' => 9,
            'Q' => 8,
            'J' => 7,
            'T' => 6,
            _ => panic!(),
        };

        return a.cmp(&b);
    }
    panic!();
    Ordering::Equal
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn solve(input: &str) -> String {
    let mut hands = Vec::new();
    for line in input.lines() {
        let (hand, bid) = parse(line).unwrap().1;
        let hand = Hand { hand, bid };
        //dbg!(&hand);
        hands.push(hand);
    }

    hands.sort();
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i as u32 + 1) * hand.bid;
        dbg!(i, hand);
    }

    sum.to_string()
}

fn parse(input: &str) -> IResult<&str, (Vec<char>, u32)> {
    let (input, a) = separated_pair(
        count(map(take(1_u8), |a: &str| a.chars().nth(0).unwrap()), 5),
        space1,
        cnom::u32,
    )(input)?;
    Ok((input, a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(solve(ti), "6440".to_string());
    }

    #[test]
    fn test_2() {
        let ti = "33333 100
TTTTT 50
KKKKK 20
QQQJA 10";
        assert_eq!(solve(ti), "440".to_string());
    }
}
