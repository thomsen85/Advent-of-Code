use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
// For number types
use nom::character::complete as cnom;

fn main() {
    dbg!(solve(include_str!("../../inputs/day*20.txt")));
}

fn solve(input: &str) -> String {
    let p = parse(input).unwrap().1;

    " ".to_string()
}

fn parse(input: &str) -> IResult<&str, ()> {

    Ok((input, ()))
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_1() {
        let ti = "1";
        assert_eq!(solve(ti), "**ans**".to_string());
    }

    #[test]
    fn test_1() {
        let ti = "-3";
        assert_eq!(solve(ti), "**ans**".to_string());
    }

    #[test]
    fn test_1() {
        let ti = "-2";
        assert_eq!(solve(ti), "**ans**".to_string());
    }

}
