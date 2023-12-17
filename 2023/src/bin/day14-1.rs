use common::utils;
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
    dbg!(solve(include_str!("../../inputs/day14.txt")));
}

fn solve(input: &str) -> String {
    let g = utils::transpose_2d_vec(utils::string_to_grid(input));
    let mut g = g
        .iter()
        .map(|line| {
            line.iter().fold(String::new(), |mut acc, n| {
                acc.push(*n);
                acc
            })
        })
        .collect_vec();

    loop {
        let mut change = false;
        for l in g.iter_mut() {
            let nl = l.replace(".O", "O.");
            if nl != *l {
                change = true;
                *l = nl;
            }
        }

        if !change {
            break;
        }
    }

    let g = g
        .into_iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let g = utils::transpose_2d_vec(g);
    let mut g = g
        .iter()
        .map(|line| {
            line.iter().fold(String::new(), |mut acc, n| {
                acc.push(*n);
                acc
            })
        })
        .collect_vec();

    dbg!(&g);
    let sum = g
        .into_iter()
        .rev()
        .enumerate()
        .map(|(m, c)| c.matches("O").count() * (m + 1))
        .sum::<usize>();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(solve(ti), "136".to_string());
    }
}
