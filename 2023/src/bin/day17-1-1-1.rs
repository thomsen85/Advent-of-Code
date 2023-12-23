use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

use glam::{IVec2, UVec2};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
};

fn main() {
    dbg!(solve(include_str!("../../inputs/day17.txt")));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug)]
struct Priority<T, P>
where
    T: PartialEq + PartialOrd + Ord + Eq,
{
    value: T,
    data: P,
}

impl<T, P> Eq for Priority<T, P> where T: PartialEq + PartialOrd + Ord + Eq {}

impl<T, P> PartialEq for Priority<T, P>
where
    T: PartialEq + PartialOrd + Ord + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T, P> Ord for Priority<T, P>
where
    T: PartialEq + PartialOrd + Ord + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value).reverse()
    }
}

impl<T, P> PartialOrd for Priority<T, P>
where
    T: PartialEq + PartialOrd + Ord + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value).reverse())
    }
}
fn solve(input: &str) -> String {
    // IVec is now Row, Columns

    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect(&format!("Got {}", c)))
                .collect_vec()
        })
        .collect_vec();
    let cols = map[0].len();
    let rows = map.len();
    let goal = IVec2::new(rows as i32 - 1, cols as i32 - 1);

    let (ans, path) = shortest_path(&map, IVec2::new(0, 0), goal).unwrap();

    let mut visited = HashSet::new();
    use Direction::*;
    dbg!(&path, ans);
    let mut last = [Down, Up, Left, Right]
        .into_iter()
        .map(|d| path[ivec_2_index(goal, rows, cols, d)])
        .inspect(|i| println!("Goal: {}", i))
        .min()
        .unwrap();

    visited.insert(last);
    dbg!(rows, cols);
    loop {
        println!("Last: {}", last);
        println!(
            "Row, Column, {}, {}",
            last % (rows * cols) / cols,
            last % (rows * cols) % cols
        );
        last = path[last];
        visited.insert(last);
        if last % (rows * cols) == 0 {
            break;
        }
    }

    for (x, row) in map.iter().enumerate() {
        for (y, v) in row.iter().enumerate() {
            use Direction::*;
            if visited.contains(&ivec_2_index(
                IVec2::new(x as i32, y as i32),
                rows,
                cols,
                Right,
            )) {
                print!(">");
            } else if visited.contains(&ivec_2_index(
                IVec2::new(x as i32, y as i32),
                rows,
                cols,
                Left,
            )) {
                print!("<");
            } else if visited.contains(&ivec_2_index(
                IVec2::new(x as i32, y as i32),
                rows,
                cols,
                Up,
            )) {
                print!("^");
            } else if visited.contains(&ivec_2_index(
                IVec2::new(x as i32, y as i32),
                rows,
                cols,
                Down,
            )) {
                print!("v");
            } else {
                print!("{}", v);
            }
        }
        println!();
    }

    ans.to_string()
}

fn shortest_path(map: &Vec<Vec<u32>>, start: IVec2, goal: IVec2) -> Option<(u32, Vec<usize>)> {
    use Direction::*;
    // dist[node] = current shortest distance from `start` to `node`
    let rows = map.len();
    let cols = map[0].len();

    let mut dist: Vec<_> = (0..(rows * cols * 4)).map(|_| u32::MAX).collect();
    let mut prev: Vec<_> = (0..(rows * cols * 4)).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[ivec_2_index(start, rows, cols, Right)] = 0;
    heap.push(Priority {
        value: 0,
        data: (start, Right, 0),
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(Priority { value, data }) = heap.pop() {
        //println!("Popped: {:?}. from queue, rest: {:?}", (value, data), &heap);
        // Alternatively, we could have continued to find all shortest paths
        let current_index = ivec_2_index(data.0, rows, cols, data.1);

        if data.0 == goal {
            return Some((value, prev));
        }

        // Important as we may have already found a better way
        if value > dist[current_index] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for n_data in get_edges(data, rows, cols) {
            let next_index = ivec_2_index(n_data.0, rows, cols, n_data.1);
            let next = Priority {
                value: value + map[n_data.0.x as usize][n_data.0.y as usize],
                data: n_data,
            };
            // If so, add it to the frontier and continue
            if next.value < dist[next_index] {
                // Relaxation, we have now found a better way
                dist[next_index] = next.value;
                prev[next_index] = current_index;
                heap.push(next);
            }
        }
    }

    // Goal not reachable
    None
}

fn get_edges(
    data: (IVec2, Direction, u8),
    rows: usize,
    cols: usize,
) -> Vec<(IVec2, Direction, u8)> {
    let mut v = Vec::new();
    let (pos, direction, suc_moves) = data;

    let s_pos = pos + direction.ivec_delta();
    if suc_moves < 3 && !is_invalid_pos(s_pos, rows, cols) {
        v.push((s_pos, direction, suc_moves + 1));
    }

    let r_direction = direction.rot90();
    let r_pos = pos + r_direction.ivec_delta();
    if !is_invalid_pos(r_pos, rows, cols) {
        v.push((r_pos, r_direction, 1));
    }

    let l_direction = direction.rot90_cc();
    let l_pos = pos + l_direction.ivec_delta();
    if !is_invalid_pos(l_pos, rows, cols) {
        v.push((l_pos, l_direction, 1));
    }

    v
}

fn ivec_2_index(vec: IVec2, rows: usize, cols: usize, direction: Direction) -> usize {
    (vec.x * cols as i32 + vec.y) as usize + (rows * cols * direction as usize)
}

fn is_invalid_pos(pos: IVec2, rows: usize, cols: usize) -> bool {
    pos.x < 0 || pos.x >= rows as i32 || pos.y < 0 || pos.y >= cols as i32
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

    #[test]
    fn test_ivec2index() {
        assert_eq!(ivec_2_index(IVec2::new(5, 8), 10, 10, Direction::Up), 58);
        assert_eq!(
            ivec_2_index(IVec2::new(5, 8), 10, 10, Direction::Right),
            358
        );
    }

    #[test]
    fn smol_test() {
        let ti = "11199
99119
99199
91199
91999
91999
91119
99911";

        assert_eq!(solve(ti), "13".to_string());
    }
    #[test]
    fn smol_test2() {
        let ti = "11111
99991
99221
92291
92991
92991
92221";

        assert_eq!(solve(ti), "13".to_string());
    }
}
