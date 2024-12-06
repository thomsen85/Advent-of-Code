use std::collections::HashSet;

use common::{datastructs::vec2::Vec2, utils::string_to_char_grid};
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
    dbg!(solve(include_str!("../../inputs/day6.txt")));
}

fn solve(input: &str) -> String {
    let m = string_to_char_grid(input);
    let mut visited = HashSet::new();

    let mut start_pos = None;
    for row in 0..m.len() {
        for col in 0..m[0].len() {
            if m[row][col] == '^' {
                start_pos = Some(Vec2::from_row_col(row, col));
                break;
            }
        }
    }

    let mut start_pos = start_pos.unwrap();
    let mut current_dir = Vec2::new(-1, 0);

    loop {
        visited.insert(start_pos);
        let mut next_pos = dbg!(start_pos + current_dir);
        if !(0..m.len() as i32).contains(&next_pos.x)
            || !(0..m[0].len() as i32).contains(&next_pos.y)
        {
            break;
        } else if m[next_pos.row()][next_pos.col()] == '#' {
            current_dir = match current_dir {
                Vec2::UP => Vec2::LEFT,
                Vec2::RIGHT => Vec2::UP,
                Vec2::DOWN => Vec2::RIGHT,
                Vec2::LEFT => Vec2::DOWN,
                _ => panic!(),
            };

            next_pos = start_pos + current_dir;
        }

        start_pos = next_pos;
    }

    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(solve(ti), "41".to_string());
    }
}
