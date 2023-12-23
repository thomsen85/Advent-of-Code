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
    // 4838 Too Low,
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

    let mut junc_list: HashMap<IVec2, HashMap<IVec2, usize>> = HashMap::new();
    junc_list.insert(start, HashMap::new());
    junc_list.insert(end, HashMap::new());

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            let c = map[x][y];
            if c == '#' {
                continue;
            }
            let pos = IVec2::new(x as i32, y as i32);
            if get_valid_neigbours(&map, pos).len() >= 3 {
                junc_list.insert(pos, HashMap::new());
            }
        }
    }

    for p in junc_list.clone().keys() {
        let mut stack = Vec::new();
        stack.push((0, *p));
        let mut seen = HashSet::new();

        while let Some((dist, to)) = stack.pop() {
            seen.insert(to);
            if to != *p && junc_list.contains_key(&to) {
                if let Some(v) = junc_list.get_mut(&p) {
                    v.insert(to, dist);
                    continue;
                }
            }

            for n in get_valid_neigbours(&map, to) {
                if seen.contains(&n) {
                    continue;
                }
                stack.push((dist + 1, n))
            }
        }
    }

    // dbg!(junc_list);
    //
    dbg!(junc_list.len());

    let d = f(start, end, &junc_list, HashSet::new(), 0);

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if d.1
                .clone()
                .unwrap()
                .contains(&IVec2::new(x as i32, y as i32))
            {
                print!("O")
            } else {
                print!("{}", map[x][y])
            }
        }
        println!()
    }

    d.0.to_string()
}

fn f(
    start: IVec2,
    end: IVec2,
    graph: &HashMap<IVec2, HashMap<IVec2, usize>>,
    mut visited: HashSet<IVec2>,
    length_traveled: usize,
) -> (usize, Option<HashSet<IVec2>>) {
    let mut max = 0;
    let mut max_v = None;
    visited.insert(start);

    if start == end {
        return (length_traveled, Some(visited));
    }

    for (p, dist) in graph.get(&start).unwrap() {
        if visited.contains(p) {
            continue;
        }
        let res = f(*p, end, graph, visited.clone(), *dist + length_traveled);
        if res.0 > max {
            max = res.0;
            max_v = res.1;
        }
    }

    (max, max_v)
}

fn get_valid_neigbours(map: &Vec<Vec<char>>, pos: IVec2) -> Vec<IVec2> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .map(|(x, y)| IVec2::new(x, y) + pos)
        .filter(|p| {
            if !(p.x > 0
                && p.x <= map.len() as i32 - 1
                && p.y > 0
                && p.y <= map[0].len() as i32 - 1)
            {
                return false;
            }
            let c = map[p.x as usize][p.y as usize];
            c != '#'
        })
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
        assert_eq!(solve(ti), "154".to_string());
    }
}
