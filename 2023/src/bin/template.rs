use std::error::Error;

use nom::{
    character::complete::{multispace0, multispace1, newline, space0, space1},
    complete::tag,
    sequence::{delimited, preceded},
};

// For number types
use nom::character::complete as cnom;

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    dbg!(solve(include_str!("../../inputs/dayX.txt"))?);
    Ok(())
}

fn solve(input: &str) -> Res<String> {
    todo!("");

    Ok(" ".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "";
        assert_eq!(solve(ti).unwrap(), "".to_string());
    }
}
