use std::collections::HashSet;

use common::{datastructs::vec2::Vec2, utils::string_to_char_grid};
fn main() {
    dbg!(solve(include_str!("../../inputs/day6.txt")));
}

fn solve(input: &str) -> String {
    let m = string_to_char_grid(input);

    let mut start_pos = None;
    for row in 0..m.len() {
        for col in 0..m[0].len() {
            if m[row][col] == '^' {
                start_pos = Some(Vec2::from_row_col(row, col));
                break;
            }
        }
    }

    let start_pos = start_pos.unwrap();

    let mut visited = HashSet::new();
    let mut pos = start_pos;
    let mut current_dir = Vec2::new(-1, 0);

    loop {
        visited.insert(pos);

        let mut next_pos = pos + current_dir;
        if !(0..m.len() as i32).contains(&next_pos.x)
            || !(0..m[0].len() as i32).contains(&next_pos.y)
        {
            break;
        } else if m[next_pos.row()][next_pos.col()] == '#' {
            current_dir = match current_dir {
                Vec2::ARR_UP => Vec2::ARR_RIGHT,
                Vec2::ARR_RIGHT => Vec2::ARR_DOWN,
                Vec2::ARR_DOWN => Vec2::ARR_LEFT,
                Vec2::ARR_LEFT => Vec2::ARR_UP,
                _ => panic!(),
            };

            next_pos = pos + current_dir;
        }

        pos = next_pos;
    }

    let vs = visited.clone();

    let mut sum = 0;
    for p in vs.iter() {
        if *p == start_pos {
            continue;
        }
        let mut visited = HashSet::new();
        let mut pos = start_pos;
        let mut current_dir = Vec2::new(-1, 0);

        loop {
            visited.insert((pos.x, pos.y, current_dir.x, current_dir.y));

            let next_pos = pos + current_dir;
            if !(0..m.len() as i32).contains(&next_pos.x)
                || !(0..m[0].len() as i32).contains(&next_pos.y)
            {
                break;
            }

            if m[next_pos.row()][next_pos.col()] == '#' || next_pos == *p {
                current_dir = match current_dir {
                    Vec2::ARR_UP => Vec2::ARR_RIGHT,
                    Vec2::ARR_RIGHT => Vec2::ARR_DOWN,
                    Vec2::ARR_DOWN => Vec2::ARR_LEFT,
                    Vec2::ARR_LEFT => Vec2::ARR_UP,
                    _ => panic!(),
                };
            } else {
                pos = next_pos;
            }

            if visited.contains(&(pos.x, pos.y, current_dir.x, current_dir.y)) {
                sum += 1;
                break;
            }

            visited.insert((pos.x, pos.y, current_dir.x, current_dir.y));
        }
    }
    sum.to_string()
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
        assert_eq!(solve(ti), "6".to_string());
    }
}
