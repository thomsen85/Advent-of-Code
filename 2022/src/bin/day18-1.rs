use std::collections::HashSet;

use glam::IVec3;
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
    dbg!(solve(include_str!("../../inputs/day18.txt")));
}

fn get_surrounding(p: IVec3) -> [IVec3; 6] {
    return [
        p + IVec3::X,
        p + IVec3::Y,
        p + IVec3::Z,
        p + IVec3::NEG_X,
        p + IVec3::NEG_Y,
        p + IVec3::NEG_Z,
    ];
}

fn solve(input: &str) -> String {
    let mut points = HashSet::new();
    let mut sides = 0;

    for l in input.lines() {
        let split = l
            .split(",")
            .map(|a| a.parse::<i32>().unwrap())
            .collect_vec();
        let point = IVec3::new(split[0], split[1], split[2]);

        let collitions = get_surrounding(point)
            .iter()
            .map(|p| if points.contains(p) { 1 } else { 0 })
            .sum::<i32>();

        sides -= collitions * 2;
        sides += 6;
        points.insert(point);
    }

    sides.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert_eq!(solve(ti), "64".to_string());
    }
}
