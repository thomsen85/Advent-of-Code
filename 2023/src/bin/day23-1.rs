use std::collections::{HashMap, HashSet};

use glam::IVec2;
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
    dbg!(solve(include_str!("../../inputs/day23.txt")));
}

fn solve(input: &str) -> String {
    // let map = input
    //     .lines()
    //     .enumerate()
    //     .flat_map(|(row, line)| {
    //         line.chars()
    //             .enumerate()
    //             .filter(|(_col, c)| *c != '#')
    //             .map(move |(col, c)| (IVec2::new(row as i32, col as i32), c))
    //     })
    //     .collect::<HashMap<IVec2, char>>();
    //

    let map = common::utils::string_to_grid(input);

    let start = IVec2::new(
        0,
        input
            .lines()
            .nth(0)
            .unwrap()
            .chars()
            .position(|c| c == '.')
            .unwrap() as i32,
    );
    let end = IVec2::new(
        map.len() as i32 - 1,
        input
            .lines()
            .last()
            .unwrap()
            .chars()
            .position(|c| c == '.')
            .unwrap() as i32,
    );
    dbg!(&map);
    dbg!(&start);

    let d = f(start, end, &map, HashSet::new(), 0);

    d.to_string()
}

fn f(
    mut start: IVec2,
    end: IVec2,
    map: &Vec<Vec<char>>,
    mut visited: HashSet<IVec2>,
    mut length_traveled: usize,
) -> usize {
    let mut max = 0;

    loop {
        visited.insert(start);
        if start == end {
            return length_traveled;
        }
        let n = get_valid_neigbours(map, start, &visited);

        if n.len() == 0 {
            return 0;
        }

        if n.len() == 1 {
            let p = n[0];
            start = p;
            length_traveled += 1;

            continue;
        }

        for p in n {
            max = max.max(f(p, end, map, visited.clone(), length_traveled + 1));
        }
        break;
    }

    max
}

fn get_valid_neigbours(map: &Vec<Vec<char>>, pos: IVec2, visited: &HashSet<IVec2>) -> Vec<IVec2> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .map(|(x, y)| IVec2::new(x, y) + pos)
        .enumerate()
        .filter(|(d, p)| {
            if !(p.x > 0
                && p.x <= map.len() as i32 - 1
                && p.y > 0
                && p.y <= map[0].len() as i32 - 1)
            {
                return false;
            }
            let c = map[p.x as usize][p.y as usize];
            c != '#'
                && !(*d == 0 && c == '<')
                && !(*d == 1 && c == '>')
                && !(*d == 2 && c == '^')
                && !(*d == 3 && c == 'v')
                && !visited.contains(&p)
        })
        .map(|(_d, p)| p)
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(solve(ti), "Answer".to_string());
    }
}
