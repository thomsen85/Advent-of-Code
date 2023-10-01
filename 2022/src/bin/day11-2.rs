#![allow(dead_code)]

use core::panic;
use std::collections::VecDeque;

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::multispace1,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug)]
enum OperationConst {
    Old,
    Const(u128),
}
impl OperationConst {
    fn unwrap(&self, old: u128) -> u128 {
        if let Self::Const(num) = self {
            num.to_owned()
        } else {
            old
        }
    }
}
impl From<&str> for OperationConst {
    fn from(value: &str) -> Self {
        if value == "old" {
            Self::Old
        } else {
            Self::Const(value.parse().unwrap())
        }
    }
}

#[derive(Debug)]
struct Operation {
    operation: char,
    left_const: OperationConst,
    right_const: OperationConst,
}

impl Operation {
    fn calculate(&self, old: u128) -> u128 {
        match self.operation {
            '+' => self.left_const.unwrap(old) + self.right_const.unwrap(old),
            '-' => self.left_const.unwrap(old) - self.right_const.unwrap(old),
            '*' => self.left_const.unwrap(old) * self.right_const.unwrap(old),
            '/' => self.left_const.unwrap(old) / self.right_const.unwrap(old),
            _ => panic!(),
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split_ascii_whitespace().collect();
        assert_eq!(3, split.len());
        assert_eq!(1, split[1].len());

        Self {
            operation: split[1].chars().next().unwrap(),
            left_const: split[0].into(),
            right_const: split[2].into(),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: u32,
    items: VecDeque<u128>,
    operation: Operation,
    test_const: u128,
    t_true: u32,
    t_false: u32,
    inspected: usize,
}

impl Monkey {
    fn new(
        id: u32,
        items: VecDeque<u128>,
        operation: Operation,
        test_const: u128,
        t_true: u32,
        t_false: u32,
    ) -> Self {
        Self {
            id,
            items,
            operation,
            test_const,
            t_true,
            t_false,
            inspected: 0,
        }
    }

    fn inspect(&mut self) -> (usize, u128) {
        self.inspected += 1;
        let item = self.items.pop_front().unwrap();
        let mut new_item = self.operation.calculate(item);

        new_item %= 9699690;

        if new_item % self.test_const == 0 {
            (self.t_true as usize, new_item)
        } else {
            (self.t_false as usize, new_item)
        }
    }
}

// Monkey 2:
//   Starting items: 54, 96, 82, 78, 69
//   Operation: new = old * old
//   Test: divisible by 7
//     If true: throw to monkey 0
//     If false: throw to monkey 7
fn parse_paragraph(input: &str) -> IResult<&str, Monkey> {
    let (input, id) = delimited(tag("Monkey "), nom::character::complete::u32, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), nom::character::complete::u128),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, operation_str) =
        delimited(tag("Operation: new = "), take_until("\n"), tag("\n"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test_const) =
        preceded(tag("Test: divisible by "), nom::character::complete::u128)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, true_c) = preceded(
        tag("If true: throw to monkey "),
        nom::character::complete::u32,
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, false_c) = preceded(
        tag("If false: throw to monkey "),
        nom::character::complete::u32,
    )(input)?;

    let monkey = Monkey::new(
        id,
        items.into(),
        operation_str.into(),
        test_const,
        true_c,
        false_c,
    );

    Ok((input, monkey))
}

fn main() {
    let mut monkeys: Vec<Monkey> = aoc2022_rust::utils::paragraph_from_file("inputs/day11.txt")
        .into_iter()
        .map(|a| parse_paragraph(&a).unwrap().1)
        .collect();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let instruction = monkeys[i].inspect();
                monkeys[instruction.0].items.push_back(instruction.1)
            }
        }
    }

    println!(
        "Monkey business: {}",
        monkeys
            .iter()
            .map(|m| m.inspected)
            .sorted()
            .rev()
            .take(2)
            .product::<usize>()
    );
}
