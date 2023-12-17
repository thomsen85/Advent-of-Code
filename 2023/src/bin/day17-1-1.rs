use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

use glam::{IVec2, UVec2};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
// For number types
use nom::character::complete as cnom;
use petgraph::{
    algo::{astar, dijkstra},
    dot::Dot,
    graph::DiGraph,
    stable_graph::NodeIndex,
    visit::EdgeRef,
    Directed, Graph,
};

fn main() {
    dbg!(solve(include_str!("../../inputs/day17.txt")));
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn ivec_delta(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(-1, 0),
            Direction::Down => IVec2::new(1, 0),
            Direction::Left => IVec2::new(0, -1),
            Direction::Right => IVec2::new(0, 1),
        }
    }

    fn rot90(&self) -> Direction {
        use Direction::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
    fn rot90_cc(&self) -> Direction {
        use Direction::*;
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }
}

fn solve(input: &str) -> String {
    // IVec is now Row, Columns
    let columns = input.lines().next().unwrap().len();
    let rows = input.lines().count();

    let map = input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, c)| c.to_digit(10).expect(&format!("Got {}", c)) as u8)
                .collect_vec()
        })
        .collect_vec();

    cracked_astar(
        &map,
        IVec2::new(0, 0),
        IVec2::new(rows as i32 - 1, columns as i32 - 1),
    )
    .to_string()
}

fn cracked_astar(map: &Vec<Vec<u8>>, start: IVec2, goal: IVec2) -> i32 {
    let rows = map.len();
    let cols = map[0].len();
    dbg!(rows, cols, rows * cols);
    // Value: Position_to_look_at,
    let mut nodes: BinaryHeap<(i32, (IVec2, Direction, u8, i32))> = VecDeque::new(); // k Heuristic
    nodes.insert(0, (0, (start, Direction::Right, 0, 0)));
    let mut dists = vec![i32::MAX; rows * cols];
    dists[0] = 0;

    while let Some((_, (pos, direction, same_direction_count, cost))) = nodes.pop_front() {
        let i = (pos.x * cols as i32 + pos.y) as usize;
        if dists[i] < cost {
            continue;
        }

        dists[i] = cost;

        if pos == goal {
            return cost;
        }
        // Left
        let l_direction = direction.rot90_cc();
        let l_pos = pos + l_direction.ivec_delta();
        if !is_invalid_pos(l_pos, rows, cols) {
            let cost = cost + map[l_pos.x as usize][l_pos.y as usize] as i32;
            let l_f = estimated_cost(l_pos, goal) + cost;

            let index = match nodes.binary_search_by(|x| x.0.cmp(&l_f)) {
                Ok(a) => a,
                Err(a) => a,
            };

            nodes.insert(index, (l_f, (l_pos, l_direction, 0, cost)));
        }
        // Right
        let r_direction = direction.rot90();
        let r_pos = pos + r_direction.ivec_delta();
        if !is_invalid_pos(r_pos, rows, cols) {
            let cost = cost + map[r_pos.x as usize][r_pos.y as usize] as i32;
            let r_f = estimated_cost(r_pos, goal) + cost;
            let index = match nodes.binary_search_by(|x| x.0.cmp(&r_f)) {
                Ok(a) => a,
                Err(a) => a,
            };

            nodes.insert(index, (r_f, (r_pos, r_direction, 0, cost)));
        }

        // Straigt
        let s_pos = pos + direction.ivec_delta();
        if same_direction_count <= 2 && !is_invalid_pos(s_pos, rows, cols) {
            let cost = cost + map[s_pos.x as usize][s_pos.y as usize] as i32;
            let s_f = estimated_cost(s_pos, goal) + cost;
            let index = match nodes.binary_search_by(|x| x.0.cmp(&s_f)) {
                Ok(a) => a,
                Err(a) => a,
            };

            nodes.insert(
                index,
                (s_f, (s_pos, direction, same_direction_count + 1, cost)),
            );
        }
    }
    0
}

fn is_invalid_pos(pos: IVec2, rows: usize, cols: usize) -> bool {
    pos.x < 0 || pos.x >= rows as i32 || pos.y < 0 || pos.y >= cols as i32
}

fn estimated_cost(start: IVec2, end: IVec2) -> i32 {
    0
    //(start.distance_squared(end) as f32).sqrt().round() as i32 * 4
}

fn parse(input: &str) -> IResult<&str, ()> {
    todo!("Add parser");

    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(solve(ti), "102".to_string());
    }
}
