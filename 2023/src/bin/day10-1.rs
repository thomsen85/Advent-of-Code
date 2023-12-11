use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    dbg!(solve(include_str!("../../inputs/day10.txt")));
}

#[derive(Debug, Clone, Copy)]
enum Piece {
    VPipe,
    HPipe,
    NEBend,
    NWBend,
    SWBend,
    SEBend,
    StartingPoint,
    Ground,
}

impl Piece {
    fn get_connections(&self) -> (bool, bool, bool, bool) {
        /// Starting at top rotating around gives N = 0, E = 1, S = 2, W = 3
        use Piece::*;
        match self {
            VPipe => (true, false, true, false),
            HPipe => (false, true, false, true),
            NEBend => (true, true, false, false),
            NWBend => (true, false, false, true),
            SWBend => (false, false, true, true),
            SEBend => (false, true, true, false),
            StartingPoint => (true, true, true, true),
            Ground => (false, false, false, false),
        }
    }
}

impl From<char> for Piece {
    fn from(value: char) -> Self {
        use Piece::*;
        match value {
            '|' => VPipe,
            '-' => HPipe,
            'L' => NEBend,
            'J' => NWBend,
            '7' => SWBend,
            'F' => SEBend,
            '.' => Ground,
            'S' => StartingPoint,
            v => panic!("{} not valid", v),
        }
    }
}

fn solve(input: &str) -> String {
    use Piece::*;
    let map = input
        .lines()
        .map(|l| l.chars().map(Piece::from).collect_vec())
        .collect_vec();

    let starting_point = map
        .iter()
        .enumerate()
        .flat_map(|(y, a)| a.iter().enumerate().map(move |(x, b)| ((y, x), *b)))
        .find(|(_, c)| matches!(c, StartingPoint))
        .unwrap();

    let mut visited_pos = HashSet::new();
    visited_pos.insert(starting_point.0);
    let mut current = starting_point;
    let mut false_moves = 0;

    loop {
        let mut moved = false;
        for (pos, con) in get_surrounding(&map, current) {
            let new_pos_y = current.0 .0 as i32 + pos.0;
            let new_pos_x = current.0 .1 as i32 + pos.1;
            let new_pos = (new_pos_y as usize, new_pos_x as usize);
            let piece = map[new_pos.0][new_pos.1];

            if !valid_con(piece, con) {
                continue;
            }

            if !visited_pos.contains(&new_pos) {
                visited_pos.insert(new_pos);
                current = (new_pos, piece);
                moved = true;
                break;
            }
        }

        if !moved && !matches!(current.1, StartingPoint) {
            false_moves += visited_pos.len() - false_moves;
            current = starting_point;
        } else if !moved {
            break;
        }
    }

    (visited_pos.len() / 2).to_string()
}

fn valid_con(piece: Piece, b: (bool, bool, bool, bool)) -> bool {
    let a = piece.get_connections();
    (a.0 && b.0) || (a.1 && b.1) || (a.2 && b.2) || (a.3 && b.3)
}

/// (delta_y, delta_x, pipe_connection)
fn get_surrounding(
    map: &[Vec<Piece>],
    current: ((usize, usize), Piece),
) -> Vec<((i32, i32), (bool, bool, bool, bool))> {
    let mut sur = Vec::new();
    let cons = current.1.get_connections();
    if current.0 .0 > 0 && cons.0 {
        sur.push(((-1, 0), (false, false, true, false)));
    }
    if current.0 .0 < map.len() - 1 && cons.2 {
        sur.push(((1, 0), (true, false, false, false)));
    }
    if current.0 .1 > 0 && cons.3 {
        sur.push(((0, -1), (false, true, false, false)));
    }

    if current.0 .1 < map[0].len() - 1 && cons.1 {
        sur.push(((0, 1), (false, false, false, true)));
    }

    sur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(solve(ti), "4".to_string());
    }
}
