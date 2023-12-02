use core::panic;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, multispace1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    solve(include_str!("../../inputs/day2.txt"));
}

fn solve(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut r_cubes = 0;
        let mut g_cubes = 0;
        let mut b_cubes = 0;
        let (_, (_id, sets)) = parse_line(line).unwrap();

        for set in sets {
            for (num, color) in set {
                match color {
                    "blue" => b_cubes = num.max(b_cubes),
                    "red" => r_cubes = num.max(r_cubes),
                    "green" => g_cubes = num.max(g_cubes),
                    _ => panic!("Unnown color"),
                };
            }
        }
        sum += b_cubes * r_cubes * g_cubes
    }

    dbg!(sum);
    sum
}

fn parse_line(i: &str) -> IResult<&str, (i32, Vec<Vec<(i32, &str)>>)> {
    let (i, id) = preceded(tag("Game "), nom::character::complete::i32)(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, sets) = separated_list1(
        tag(";"),
        separated_list1(
            tag(","),
            preceded(
                multispace0,
                separated_pair(nom::character::complete::i32, multispace1, alpha1),
            ),
        ),
    )(i)?;

    Ok((i, (id, sets)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let ti = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve(ti), 2286);
    }
}
