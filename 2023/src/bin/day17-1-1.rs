use std::collections::{BTreeMap, HashMap, HashSet};

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
    // Value: Position_to_look_at,
    let mut nodes: BTreeMap<i32, (IVec2, Direction, u8, i32)> = BTreeMap::new(); // k Heuristic
    nodes.insert(0, (start, Direction::Right, 0, 0));
    let mut dist = HashMap::new();
    dist.insert(start, 0);

    while let Some((_, (pos, direction, same_direction_count, cost))) = nodes.pop_first() {
        dist.insert(pos, cost);
        dbg!(&dist);
        if pos == goal {
            return cost;
        }
        // Left
        let l_direction = direction.rot90_cc();
        let l_pos = pos + l_direction.ivec_delta();
        if !is_invalid_pos(l_pos, rows, cols) {
            let cost = cost + map[l_pos.x as usize][l_pos.y as usize] as i32;
            let l_f = estimated_cost(l_pos, goal) + cost;
            nodes.insert(l_f, (l_pos, l_direction, 0, cost));
        }
        // Right
        let r_direction = direction.rot90();
        let r_pos = pos + r_direction.ivec_delta();
        if !is_invalid_pos(r_pos, rows, cols) {
            let cost = cost + map[r_pos.x as usize][r_pos.y as usize] as i32;
            let r_f = estimated_cost(r_pos, goal) + cost;
            nodes.insert(r_f, (r_pos, r_direction, 0, cost));
        }

        // Straigt
        let s_pos = pos + direction.ivec_delta();
        if same_direction_count <= 3 && !is_invalid_pos(s_pos, rows, cols) {
            let cost = cost + map[s_pos.x as usize][s_pos.y as usize] as i32;
            let s_f = estimated_cost(s_pos, goal) + cost;
            nodes.insert(s_f, (s_pos, direction, same_direction_count + 1, cost));
        }
    }
    0
}

fn is_invalid_pos(pos: IVec2, rows: usize, cols: usize) -> bool {
    pos.x < 0 || pos.x >= rows as i32 || pos.y < 0 || pos.y >= cols as i32
}

fn estimated_cost(start: IVec2, end: IVec2) -> i32 {
    (start.distance_squared(end) as f32).sqrt().round() as i32
}

fn get_edges(
    map: &Vec<Vec<(u32, u8)>>,
    last_pos: IVec2,
    direction: Direction,
    same_direction_count: u8,
    mut tot_cost: u32,
    costs: &mut HashMap<IVec2, u32>,
) {
    let rows = map.len();
    let cols = map[0].len();
    let new_pos = last_pos + direction.ivec_delta();

    if new_pos.x < 0 || new_pos.x >= rows as i32 || new_pos.y < 0 || new_pos.y >= cols as i32 {
        return;
    }

    let cost = map[new_pos.x as usize][new_pos.y as usize].1;

    tot_cost += cost as u32;

    if let Some(old_cost) = costs.get_mut(&new_pos) {
        if *old_cost < tot_cost {
            return;
        } else {
            *old_cost = tot_cost;
        }
    } else {
        costs.insert(new_pos, tot_cost);
    }

    if same_direction_count < 3 {
        get_edges(
            map,
            new_pos,
            direction,
            same_direction_count + 1,
            tot_cost,
            costs,
        );
    }

    get_edges(map, new_pos, direction.rot90(), 0, tot_cost, costs);
    get_edges(map, new_pos, direction.rot90_cc(), 0, tot_cost, costs);
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
