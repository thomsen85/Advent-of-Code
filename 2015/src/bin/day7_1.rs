#![allow(dead_code)]

use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace1},
    combinator::map,
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Hash, Clone, Copy)]
enum Operation {
    NOT,
    AND,
    OR,
    RSHIFT,
    LSHIFT,
}

fn perform_operation(operation: Operation, val_1: u16, val_2: u16) -> u16 {
    match operation {
        Operation::AND => val_1 & val_2,
        Operation::OR => val_1 | val_2,
        Operation::RSHIFT => val_1 >> val_2,
        Operation::LSHIFT => val_1 << val_2,
        _ => unimplemented!(),
    }
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        map(tag("NOT"), |_| Operation::NOT),
        map(tag("AND"), |_| Operation::AND),
        map(tag("OR"), |_| Operation::OR),
        map(tag("RSHIFT"), |_| Operation::RSHIFT),
        map(tag("LSHIFT"), |_| Operation::LSHIFT),
    ))(input)
}

#[derive(Debug, Hash, Clone)]
enum Lexicon {
    SignalStrength(u16),
    Wire(String),
    Operator(Operation),
    To,
}

impl Lexicon {
    fn to_val(&self, wires: &mut HashMap<String, u16>) -> Option<u16> {
        match self {
            Lexicon::SignalStrength(v) => Some(v.to_owned()),
            Lexicon::Wire(w) => wires.get(w).map(|w| w.to_owned()),
            _ => panic!(),
        }
    }
}

fn main() {
    let input = include_str!("../../inputs/day7.txt");
    solve(input);
}

fn solve(input: &str) {
    let mut wires: HashMap<String, u16> = HashMap::new();
    for line in input.lines() {
        let lex = parse_line(line).unwrap().1;

        use Lexicon::*;

        match &lex[..] {
            [v1, To, Wire(w)] => {
                if let Some(val_1) = v1.to_val(&mut wires) {
                    wires.insert(w.to_owned(), val_1);
                }
            }
            [v1, Operator(o), v2, To, Wire(w3)] => {
                let val_1 = v1.to_val(&mut wires);
                let val_2 = v2.to_val(&mut wires);

                if val_1.is_none() || val_2.is_none() {
                    continue;
                }

                wires.insert(
                    w3.to_owned(),
                    perform_operation(o.to_owned(), val_1.unwrap(), val_2.unwrap()),
                );
                if w3 == "a" {
                    dbg!(val_1);
                }
            }
            [Operator(Operation::NOT), v1, To, Wire(w)] => {
                if let Some(val_1) = v1.to_val(&mut wires) {
                    wires.insert(w.to_owned(), !val_1);
                }
            }
            _ => unimplemented!("{:?}", lex),
        };
    }

    dbg!(wires);
}

fn parse_line(input: &str) -> IResult<&str, Vec<Lexicon>> {
    let (input, line) = separated_list1(
        multispace1,
        alt((
            map(parse_operation, Lexicon::Operator),
            map(tag("->"), |_| Lexicon::To),
            map(nom::character::complete::u16, Lexicon::SignalStrength),
            map(alpha1, |r: &str| Lexicon::Wire(r.to_owned())),
        )),
    )(input)?;
    Ok((input, line))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

        solve(input);

        assert!(1 == 2);
    }
}
