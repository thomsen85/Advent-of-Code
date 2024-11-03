use std::collections::VecDeque;

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
    dbg!(solve(include_str!("../../inputs/day20.txt")));
}

fn solve(input: &str) -> String {
    let mut inp = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<VecDeque<_>>();

    let mut index = (0..inp.len() as i32).collect::<VecDeque<_>>();

    let len = inp.len();
    let mut i = 0;
    loop {
        let index_item = index.iter().position(|&a| a as usize == i).unwrap() as i32;
        println!("{}", inp.iter().map(|a| a.to_string()).join(", "));
        // dbg!(&index.);

        let c = inp.remove(index_item as usize).unwrap();
        println!("Moving {}", c);
        let d = index.remove(index_item as usize).unwrap();

        let mut x = index_item + c;
        if x == 0 {
            x = len as i32 - 1;
        }

        let new_index = x.rem_euclid(len as i32 - 1) as usize;

        inp.insert(new_index, c);
        index.insert(new_index, d);

        i += 1;

        if i >= index.len() {
            break;
        }
    }

    println!("{}", inp.iter().map(|a| a.to_string()).join(", "));
    let zero_index = dbg!(inp.iter().position(|&a| a == 0).unwrap());
    let a = dbg!(*inp.iter().cycle().nth(zero_index + 1000).unwrap());
    let b = dbg!(*inp.iter().cycle().nth(zero_index + 2000).unwrap());
    let c = dbg!(*inp.iter().cycle().nth(zero_index + 3000).unwrap());

    dbg!(a + b + c).to_string()
}

fn parse(input: &str) -> IResult<&str, ()> {
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "1
2
-3
3
-2
0
4";
        assert_eq!(solve(ti), "3".to_string());
    }
}
