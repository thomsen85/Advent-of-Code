use std::collections::HashSet;

use common::{datastructs::vec2::Vec2, strings::string_to_char_grid};

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

        let next_pos = start_pos + current_dir;
        if !(0..m.len() as i32).contains(&next_pos.x)
            || !(0..m[0].len() as i32).contains(&next_pos.y)
        {
            break;
        }

        if m[next_pos.row()][next_pos.col()] == '#' {
            current_dir = current_dir.arr_rot_90_clockwise();
        } else {
            start_pos = next_pos;
        }
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
