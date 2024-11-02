use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
// For number types
use nom::character::complete as cnom;

fn main() {
    dbg!(solve(include_str!("../../inputs/day20.txt")));
}

fn solve(input: &str) -> String {
    let p = parse(input).unwrap().1;

    " ".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "";
        assert_eq!(solve(ti), "Answer".to_string());
    }
}
