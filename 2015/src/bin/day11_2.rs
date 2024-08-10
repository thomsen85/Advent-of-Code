use std::u16;

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
    // 1 hour 7 min 31 secds
    dbg!(solve("hepxxyzz"));
}

fn solve(input: &str) -> String {
    dbg!(input);
    let mut current = inc(input, None);
    loop {
        let (valid, skip) = is_valid_pass(&current);
        if valid {
            break;
        }

        current = inc(&current, skip);
    }
    current
}

fn inc(p: &str, skip: Option<usize>) -> String {
    let max = 'z' as u8;
    let pc = p.chars().collect_vec();
    let mut new = Vec::new();
    let mut carry = true;
    let to = if let Some(s) = skip { s + 1 } else { p.len() };
    for _ in (to..p.len()).rev() {
        new.push('a')
    }

    for i in (0..to).rev() {
        let add = if carry { 1 } else { 0 };
        carry = false;
        if pc[i] as u8 + add > max {
            carry = true;
            new.push('a');
        } else {
            new.push((pc[i] as u8 + add) as char);
        }
    }

    if carry {
        new = (0..p.len()).map(|_| 'a').collect_vec();
    }

    new.into_iter().rev().join("")
}

fn is_valid_pass(p: &str) -> (bool, Option<usize>) {
    if let Some(i) = p.chars().position(|c| matches!(c, 'i' | 'o' | 'l')) {
        return (false, Some(i));
    }

    let mut has_inc = false;

    let pc = p.chars().collect_vec();
    for i in 2..p.len() {
        if pc[i - 2] as u16 + 1 == pc[i - 1] as u16 && pc[i - 1] as u16 + 1 == pc[i] as u16 {
            has_inc = true;
            break;
        }
    }

    if !has_inc {
        return (false, None);
    }

    let pairs = p
        .chars()
        .tuple_windows()
        .enumerate()
        .filter(|(i, (a, b))| a == b)
        .collect_vec();

    if pairs.len() < 2 || (pairs.len() == 2 && pairs[0].0 + 1 == pairs[1].0) {
        return (false, None);
    }
    (true, None)
}

fn parse(input: &str) -> IResult<&str, ()> {
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("hijklmmn", (false, Some(1)))]
    #[case("abbceffg", (false, None))]
    #[case("abbcegjk", (false, None))]
    #[case("abcdffaa", (true, None))]
    #[case("ghijklmn", (false, Some(2)))]
    fn test_valid(#[case] a: &str, #[case] b: (bool, Option<usize>)) {
        assert_eq!(is_valid_pass(a), b);
    }

    #[rstest]
    #[case("abcdefgh", "abcdffaa")]
    #[case("ghijklmn", "ghjaabcc")]
    fn test_next(#[case] a: &str, #[case] b: &str) {
        assert_eq!(solve(a), b);
    }

    #[test]
    fn test_inc() {
        let ti = "abaca";
        assert_eq!(inc(ti, Some(1)), "acaaa".to_string());
    }
}
