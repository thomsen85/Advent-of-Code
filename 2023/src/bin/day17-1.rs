use std::collections::{HashMap, HashSet};

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
    dbg!(solve(include_str!("../../inputs/day12.txt")));
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
                .map(|(col, c)| ((row * columns + col) as u32, c.to_digit(10).unwrap() as u8))
                .collect_vec()
        })
        .collect_vec();

    let goal = map[rows - 1][columns - 1].0;

    let mut edges = HashMap::new();
    get_edges(&map, IVec2::new(0, -1), Direction::Right, 0, &mut edges);

    let g = DiGraph::<(), u32>::from_edges(edges.iter().map(|(k, v)| (k.0, k.1, *v as u32)));
    let res = astar(
        &g,
        NodeIndex::new(0),
        |fin| fin.index() as u32 == goal,
        |edge| *edge.weight(),
        |edge| estimated_cost(0, edge.index() as u32, columns as u32),
    )
    .unwrap();

    dbg!(res.1);
    res.0.to_string()
}

fn estimated_cost(start: u32, end: u32, cols: u32) -> u32 {
    let s = UVec2::new(start / cols, start % cols).as_ivec2();
    let e = UVec2::new(end / cols, end % cols).as_ivec2();
    s.distance_squared(e) as u32
}

fn get_edges(
    map: &Vec<Vec<(u32, u8)>>,
    current_pos: IVec2,
    direction: Direction,
    same_direction_count: u8,
    edges: &mut HashMap<(u32, u32), u8>,
) {
    let rows = map.len();
    let cols = map[0].len();
    let new_pos = current_pos + direction.ivec_delta();

    if new_pos.x < 0 || new_pos.x >= rows as i32 || new_pos.y < 0 || new_pos.y >= cols as i32 {
        return;
    }

    let edge = (
        map[current_pos.x.try_into().unwrap_or(0)][current_pos.y.try_into().unwrap_or(0)].0,
        map[new_pos.x as usize][new_pos.y as usize].0,
    );

    if edges.contains_key(&edge) {
        return;
    }

    let cost = map[new_pos.x as usize][new_pos.y as usize].1;

    edges.insert(edge, cost);

    dbg!(edge, cost, same_direction_count);
    if same_direction_count < 3 {
        get_edges(map, new_pos, direction, same_direction_count + 1, edges);
    }

    get_edges(map, new_pos, direction.rot90(), 0, edges);
    get_edges(map, new_pos, direction.rot90_cc(), 0, edges);
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
