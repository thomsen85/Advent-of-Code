use std::{collections::HashSet, process::exit};

use common::utils;
use glam::IVec2;
use indicatif::ProgressIterator;

fn main() {
    dbg!(solve(include_str!("../../inputs/day21.txt"), 26501365));
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

    const MOVES: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];
    let rows = map.len();
    let columns = map[0].len();
    dbg!(rows, columns, steps);
    let mut visited: HashSet<IVec2> = HashSet::from([start]);

    for _step in 0..steps {
        let visited_c = visited.clone();
        visited.clear();

        for p in visited_c {
            for mov in MOVES {
                let new_p = p + mov;
                if map[new_p.x.rem_euclid(rows as i32) as usize]
                    [new_p.y.rem_euclid(columns as i32) as usize]
                    == '#'
                {
                    continue;
                }

                visited.insert(new_p);
            }
        }

        if _step % 5 == 0 {
            println!(
                "{}\t{}",
                _step,
                (visited.len() as f64).sqrt() / _step as f64
            );
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
    #[test]
    fn test_2() {
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
        assert_eq!(solve(ti, 10), "50".to_string());
    }
    #[test]
    fn test_3() {
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
        assert_eq!(solve(ti, 500), "167004".to_string());
    }
}
