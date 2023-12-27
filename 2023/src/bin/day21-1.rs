use std::collections::HashSet;

use common::utils;
use glam::IVec2;

fn main() {
    dbg!(solve(include_str!("../../inputs/day21.txt"), 64));
}

fn solve(input: &str, steps: usize) -> String {
    let map = utils::string_to_grid(input);

    let start = map
        .iter()
        .enumerate()
        .flat_map(|(r, line)| {
            line.iter()
                .enumerate()
                .map(move |(col, c)| (IVec2::new(r as i32, col as i32), c))
        })
        .find(|(_, c)| **c == 'S')
        .unwrap()
        .0;

    let mut visited: HashSet<IVec2> = HashSet::from([start]);
    for _step in 0..steps {
        let visited_c = visited.clone();
        visited.clear();

        const MOVES: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];
        for p in visited_c {
            for mov in MOVES {
                let new_p = p + mov;
                if map[new_p.x as usize][new_p.y as usize] == '#' {
                    continue;
                }

                visited.insert(new_p);
            }
        }
    }

    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(solve(ti, 6), "16".to_string());
    }
}
