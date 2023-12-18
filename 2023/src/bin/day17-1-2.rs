use std::collections::{BinaryHeap, HashSet};

use glam::IVec2;
use itertools::Itertools;

fn main() {
    dbg!(solve(include_str!("../../inputs/day17.txt")));
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn to_ivec_delta(&self) -> IVec2 {
        use Direction::*;
        IVec2::from_array(match self {
            Up => [-1, 0],
            Down => [1, 0],
            Left => [0, -1],
            Right => [0, 1],
        })
    }

    const fn rot90(&self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
    const fn rot90_cc(&self) -> Self {
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
struct Node {
    total_heat_loss: u32,
    position: IVec2,
    direction: Direction,
    path_followed: u8,
}

impl Eq for Node {}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_heat_loss.cmp(&other.total_heat_loss).reverse()
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.total_heat_loss.partial_cmp(&other.total_heat_loss) {
            Some(a) => Some(a.reverse()),
            None => None,
        }
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.total_heat_loss == other.total_heat_loss
    }
}

fn solve(input: &str) -> String {
    let map = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let cols = map[0].len();

    let mut pri_queue: BinaryHeap<Node> = BinaryHeap::new();
    let mut dists = vec![u32::MAX; map.len() * cols];
    let goal = IVec2::new(map.len() as i32 - 1, map[0].len() as i32 - 1);
    // let goal = IVec2::new(60, 60);

    pri_queue.push(Node {
        total_heat_loss: map[0][0],
        position: IVec2::splat(0),
        direction: Direction::Right,
        path_followed: 0,
    });

    while let Some(n) = pri_queue.pop() {
        if n.position == goal {
            dbg!(n);
            break;
        }
        let i = n.position.x as usize * cols + n.position.y as usize;
        if dists[i] <= n.total_heat_loss {
            continue;
        }
        dists[i] = n.total_heat_loss;
        // dbg!(seen.len());
        // Direction: Where we went to arrive
        // poision: Where we have arrive

        let mut possible_directions = vec![n.direction.rot90_cc(), n.direction.rot90()];

        if n.path_followed < 3 {
            possible_directions.push(n.direction);
        }

        for direction in possible_directions {
            let next_pos = n.position + direction.to_ivec_delta();
            if !is_valid_pos(next_pos, &map) {
                continue;
            }
            let total_heat_loss = map[next_pos.x as usize][next_pos.y as usize] + n.total_heat_loss;
            let path_followed = if n.direction == direction {
                n.path_followed + 1
            } else {
                1
            };
            let new_n = Node {
                total_heat_loss,
                direction,
                position: next_pos,
                path_followed,
            };

            pri_queue.push(new_n);
        }
    }

    "".to_string()
}

fn is_valid_pos(next_pos: IVec2, map: &[Vec<u32>]) -> bool {
    (0..map.len()).contains(&(next_pos.x as usize))
        && (0..map[0].len()).contains(&(next_pos.y as usize))
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
