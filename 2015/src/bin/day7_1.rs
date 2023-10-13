#![allow(dead_code)]

use nom::{branch::alt, combinator::map, complete::tag, IResult};

enum Gate {
    NOT,
    AND,
    OR,
    RSHIFT,
    LSHIFT,
}

enum Lexicon {
    SignalStrength(u16),
    Wire(String),
    Operator(String),
    To,
}

impl Lexicon {
    fn parse(input: &str) -> IResult<&str, Self> {}
}

fn main() {
    let input = common::utils::lines_from_file("inputs/day7.txt");
    dbg!(input);
}

fn parse_line(input: &str) -> IResult<&str, Vec<Lexicon>> {}
