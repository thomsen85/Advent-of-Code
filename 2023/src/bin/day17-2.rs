use std::collections::{BinaryHeap, HashMap, HashSet};

use common::utils;
use glam::IVec2;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    fn all() -> [Self; 4] {
        use Direction::*;
        [Up, Down, Left, Right]
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

fn main() {
    dbg!(solve(include_str!("../../inputs/day17.txt")));
}

fn solve(input: &str) -> String {
    let map = utils::string_to_grid(input)
        .into_iter()
        .map(|l| {
            l.into_iter()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec();

    let end = IVec2::new(map.len() as i32 - 1, map[0].len() as i32 - 1);
    dbg!(end);
    dijkstra_ish(&map, IVec2::new(0, 0), end).to_string()
}

fn dijkstra_ish(map: &Vec<Vec<i32>>, start: IVec2, end: IVec2) -> usize {
    let rows = map.len();
    let cols = map[0].len();
    // Using vertexes *4 for each direction.
    let mut seen: HashSet<(IVec2, Direction, u8)> = HashSet::new();

    let mut queue: BinaryHeap<Priority<usize, (IVec2, Direction, u8)>> = BinaryHeap::new();
    queue.push(Priority {
        value: 0,
        data: (start, Direction::Right, 0),
    });
    queue.push(Priority {
        value: 0,
        data: (start, Direction::Down, 0),
    });

    while let Some(Priority {
        value: cost,
        data: this,
    }) = queue.pop()
    {
        if this.0 == end {
            return cost;
        }

        if seen.contains(&this) {
            continue;
        }

        seen.insert(this);

        let nexts = get_edges(this, rows, cols);

        for next in nexts {
            let next_price = map[next.0.x as usize][next.0.y as usize] as usize + cost;

            queue.push(Priority {
                value: next_price,
                data: next,
            });
        }
    }
    0
}

fn get_edges(
    data: (IVec2, Direction, u8),
    rows: usize,
    cols: usize,
) -> Vec<(IVec2, Direction, u8)> {
    let mut v = Vec::new();
    let (pos, direction, suc_moves) = data;

    let s_pos = pos + direction.ivec_delta();
    if suc_moves < 10 && !is_invalid_pos(s_pos, rows, cols) {
        v.push((s_pos, direction, suc_moves + 1));
    }

    if suc_moves < 4 {
        return v;
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
        assert_eq!(solve(ti), "94".to_string());
    }

    #[test]
    fn test_2() {
        let ti = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(solve(ti), "71")
    }
}
