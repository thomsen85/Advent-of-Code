use std::collections::{HashSet, VecDeque};

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
    Spillway,
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
            Spillway => (false, false, false, false),
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
    let mut path = Vec::new();
    path.push(starting_point);
    let mut current = starting_point;

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
                path.push(current);
                moved = true;
                break;
            }
        }

        if !moved {
            break;
        }
    }

    // display(&map, &visited_pos);

    let mut visited_pos = visited_pos
        .into_iter()
        .map(|(y, x)| (2 * y + 1, 2 * x + 1))
        .collect::<HashSet<(usize, usize)>>();

    let mut queue = VecDeque::new();
    let mut flooded = HashSet::new();
    queue.push_back((0, 0));

    let mut map2 = Vec::new();

    let mut l = 0;
    for row in map {
        let mut row_n = vec![Spillway];
        for val in row {
            row_n.push(val);
            row_n.push(Spillway)
        }
        l = row_n.len();
        map2.push(row_n);
        map2.push(vec![Spillway; l]);
    }

    map2.insert(0, vec![Spillway; l]);
    let mut map = map2;
    let mut adders = HashSet::new();
    let height = map.len();
    let width = map[0].len();

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            if !matches!(map[y][x], Spillway) {
                continue;
            }

            if visited_pos.contains(&(y - 1, x)) && visited_pos.contains(&(y + 1, x)) {
                let n_c = map[y - 1][x].get_connections().2;
                let s_c = map[y + 1][x].get_connections().0;

                if n_c && s_c {
                    map[y][x] = VPipe;
                    adders.insert((y, x));
                    continue;
                }
            }

            if visited_pos.contains(&(y, x - 1)) && visited_pos.contains(&(y, x + 1)) {
                let e_c = map[y][x + 1].get_connections().3;
                let w_c = map[y][x - 1].get_connections().1;

                if e_c && w_c {
                    map[y][x] = HPipe;
                    adders.insert((y, x));
                }
            }
        }
    }

    visited_pos.extend(adders.into_iter());

    while let Some(n) = queue.pop_front() {
        flooded.insert(n);
        for d_pos in get_around(n, (height, width)) {
            let next_pos_y = n.0 as i32 + d_pos.0;
            let next_pos_x = n.1 as i32 + d_pos.1;
            let next_pos = (next_pos_y as usize, next_pos_x as usize);

            if !visited_pos.contains(&next_pos)
                && !flooded.contains(&next_pos)
                && !queue.contains(&next_pos)
            {
                queue.push_back(next_pos);
            }
        }
    }

    display(&map, &visited_pos, &flooded);
    let len = map
        .iter()
        .enumerate()
        .flat_map(|(y, c)| c.iter().enumerate().map(|(x, a)| (y, x, a)).collect_vec())
        .filter(|a| !matches!(a.2, Spillway) && !flooded.contains(&(a.0, a.1)))
        .count();
    dbg!(len, visited_pos.len());
    (len - visited_pos.len()).to_string()
}

fn get_around(pos: (usize, usize), max: (usize, usize)) -> Vec<(i32, i32)> {
    let mut v = Vec::new();
    if pos.0 > 0 {
        v.push((-1, 0));
    }
    if pos.0 < max.0 - 1 {
        v.push((1, 0));
    }
    if pos.1 > 0 {
        v.push((0, -1));
    }
    if pos.1 < max.1 - 1 {
        v.push((0, 1))
    }

    v
}

fn display(
    map: &Vec<Vec<Piece>>,
    visited_pos: &HashSet<(usize, usize)>,
    flooded: &HashSet<(usize, usize)>,
) {
    let height = map.len();
    let length = map[0].len();

    println!();
    for y in 0..height {
        for x in 0..length {
            let c = {
                if visited_pos.contains(&(y, x)) {
                    use Piece::*;
                    match map[y][x] {
                        VPipe => "│",
                        HPipe => "─",
                        NEBend => "└",
                        NWBend => "┘",
                        SWBend => "┐",
                        SEBend => "┌",
                        StartingPoint => "S",
                        Spillway => ".",
                        _ => " ",
                    }
                } else if flooded.contains(&(y, x)) {
                    "~"
                } else {
                    " "
                }
            };
            print!("{}", c);
        }
        println!();
    }
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
        let ti = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!(solve(ti), "4".to_string());
    }

    #[test]
    fn test_3() {
        let ti = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(solve(ti), "4".to_string());
    }
    #[test]
    fn test_2() {
        let ti = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(solve(ti), "8".to_string());
    }

    #[test]
    fn test_4() {
        let ti = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ.F7FJ-
L---JF-JLJ....FJLJJ7
|F|F-JF---7...L7L|7|
|FFJF7L7F-JF7..L---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(solve(ti), "10".to_string());
    }
}
