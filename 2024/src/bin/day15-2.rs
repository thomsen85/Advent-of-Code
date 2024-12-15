use common::{datastructs::vec2::Vec2, strings::string_to_char_grid};
use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day15.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let (m, seq) = input.split_once("\n\n").unwrap();
    let old_m = string_to_char_grid(m);
    let mut m = old_m
        .into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|c| {
                    (match c {
                        '#' => "##",
                        'O' => "[]",
                        '.' => "..",
                        '@' => "@.",
                        _ => panic!(),
                    })
                    .chars()
                    .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    let seq = seq.replace("\n", "");
    let mut start = None;
    for row in 0..m.len() {
        for col in 0..m[0].len() {
            if m[row][col] == '@' {
                start = Some(Vec2::from_row_col(row, col));
            }
        }
    }

    let start = start.unwrap();
    let mut p = start.to_owned();

    for seq_c in seq.chars() {
        let dir = match seq_c {
            '<' => Vec2::ARR_LEFT,
            'v' => Vec2::ARR_DOWN,
            '^' => Vec2::ARR_UP,
            '>' => Vec2::ARR_RIGHT,
            _ => panic!(),
        };

        let mut movable = true;
        let mut stack = Vec::new();
        let mut pm = HashSet::new();
        stack.push(p.to_owned());

        while let Some(curr) = stack.pop() {
            if pm.contains(&curr) {
                continue;
            }

            let curr_c = *curr.i_arr(&m);
            if curr_c == '#' {
                movable = false;
                break;
            }

            if curr_c == '.' {
                continue;
            }

            pm.insert(curr);
            if curr_c == '[' {
                stack.push(curr + Vec2::ARR_RIGHT)
            }
            if curr_c == ']' {
                stack.push(curr + Vec2::ARR_LEFT)
            }

            stack.push(curr + dir)
        }

        if movable {
            let mut m_clone = m.clone();
            for pp in &pm {
                *pp.i_arr_mut(&mut m_clone) = '.';
            }

            for &pp in &pm {
                let c = pp.i_arr(&m);
                *(pp + dir).i_arr_mut(&mut m_clone) = *c;
            }

            *p.i_arr_mut(&mut m_clone) = '.';
            p = p + dir;
            m = m_clone
        }
    }

    let mut sum = 0;
    for row in 0..m.len() {
        for col in 0..m[0].len() {
            if m[row][col] == '[' {
                sum += row * 100 + col;
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(solve(ti), "9021".to_string());
    }
}
