use std::collections::{HashSet, VecDeque};

use glam::IVec2;
use itertools::Itertools;

fn main() {
    dbg!(solve(include_str!("../../inputs/day16.txt")));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_ivec_delta(&self) -> IVec2 {
        use Direction::*;
        match self {
            Up => IVec2::new(0, -1),
            Down => IVec2::new(0, 1),
            Left => IVec2::NEG_X,
            Right => IVec2::X,
        }
    }
}

fn solve(input: &str) -> String {
    let map = input.lines().map(|a| a.chars().collect_vec()).collect_vec();

    let height = map.len();
    let width = map[0].len();
    let mut max = 0;

    use Direction::*;
    for x in 0..width {
        max = max.max(fun_name(&map, x as i32, 0, Down));
        max = max.max(fun_name(&map, x as i32, height as i32 - 1, Up));
    }

    for y in 0..height {
        max = max.max(fun_name(&map, 0, y as i32, Right));
        max = max.max(fun_name(&map, width as i32 - 1, y as i32, Left));
    }
    max.to_string()
}

fn fun_name(map: &Vec<Vec<char>>, x: i32, y: i32, direc: Direction) -> usize {
    use Direction::*;
    let height = map.len();
    let width = map[0].len();
    let mut active_beams = VecDeque::new();
    let mut visited = HashSet::new();
    let mut activated = HashSet::new();

    active_beams.push_back((IVec2::new(x, y), direc));

    while let Some(beam) = active_beams.pop_front() {
        if beam.0.x >= width as i32
            || beam.0.x < 0
            || beam.0.y >= height as i32
            || beam.0.y < 0
            || visited.contains(&beam)
        {
            continue;
        }
        visited.insert(beam);
        activated.insert(beam.0);

        match map[beam.0.y as usize][beam.0.x as usize] {
            '/' => {
                let n_direction = match beam.1 {
                    Up => Right,
                    Down => Left,
                    Left => Down,
                    Right => Up,
                };
                active_beams.push_back((beam.0 + n_direction.to_ivec_delta(), n_direction));
            }
            '\\' => {
                let n_direction = match beam.1 {
                    Up => Left,
                    Down => Right,
                    Left => Up,
                    Right => Down,
                };
                active_beams.push_back((beam.0 + n_direction.to_ivec_delta(), n_direction));
            }
            '-' => match beam.1 {
                Up | Down => {
                    active_beams.push_back((beam.0 + Left.to_ivec_delta(), Left));
                    active_beams.push_back((beam.0 + Right.to_ivec_delta(), Right));
                }
                _ => active_beams.push_back((beam.0 + beam.1.to_ivec_delta(), beam.1)),
            },
            '|' => match beam.1 {
                Left | Right => {
                    active_beams.push_back((beam.0 + Up.to_ivec_delta(), Up));
                    active_beams.push_back((beam.0 + Down.to_ivec_delta(), Down));
                }
                _ => active_beams.push_back((beam.0 + beam.1.to_ivec_delta(), beam.1)),
            },
            '.' => active_beams.push_back((beam.0 + beam.1.to_ivec_delta(), beam.1)),

            c => panic!("{} not expected", c),
        }
        // display(&map, &visited);
    }

    activated.len()
}

fn display(map: &[Vec<char>], activated: &HashSet<(IVec2, Direction)>) {
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if activated
                .iter()
                .find(|a| a.0 == IVec2::new(x as i32, y as i32))
                .is_some()
            {
                print!("#")
            } else {
                print!("{}", c)
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(solve(ti), "51".to_string());
    }
}
