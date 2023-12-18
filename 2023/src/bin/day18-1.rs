use std::collections::{HashSet, VecDeque};

use geo::{
    coord, Area, BoundingRect, ConcaveHull, Contains, Coord, CoordsIter, Densify, GeodesicArea,
    LineString, MultiPolygon, Polygon, Rect,
};
use glam::IVec2;
use itertools::{Itertools, MinMaxResult};
use nom::{
    bytes::complete::{tag, take, take_while_m_n},
    character::{
        complete::{alpha1, alphanumeric1, multispace0, multispace1, newline, space0, space1},
        is_alphabetic, is_alphanumeric,
    },
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};
// For number types
use nom::character::complete as cnom;

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
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        use Direction::*;
        match value {
            'U' => Up,
            'D' => Down,
            'R' => Right,
            'L' => Left,
            c => panic!("Invalid char {c}"),
        }
    }
}

fn main() {
    dbg!(solve(include_str!("../../inputs/day18.txt")));
}

fn solve(input: &str) -> String {
    let lines = parse(input).unwrap().1;
    let mut current_pos = IVec2::new(0, 0);
    let mut points: HashSet<IVec2> = HashSet::new();

    for ((direction, amount), _color) in lines {
        for i in 0..amount {
            current_pos += direction.to_ivec_delta();
            points.insert(current_pos);
        }
    }

    let (min_x, max_x) = match points.iter().map(|a| a.x).minmax() {
        MinMaxResult::MinMax(a, b) => (a, b),
        _ => panic!(),
    };
    let (min_y, max_y) = match points.iter().map(|a| a.y).minmax() {
        MinMaxResult::MinMax(a, b) => (a, b),
        _ => panic!(),
    };

    let point_inside = IVec2::new(-1, -1);

    let mut queue = VecDeque::new();
    queue.push_back(point_inside);
    while let Some(n) = queue.pop_front() {
        assert!(n.x < max_x && n.x > min_x && n.y < max_y && n.y > min_y);
        points.insert(n);
        for sur in get_surrounding(n) {
            if points.contains(&sur) {
                continue;
            }
            queue.push_front(sur);
        }
    }

    for row in -20..12 {
        for col in -20..12 {
            if points.contains(&IVec2::new(row, col)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }

    (points.len()).to_string()
}

fn get_surrounding(p: IVec2) -> Vec<IVec2> {
    use Direction::*;
    [Up, Right, Down, Left]
        .iter()
        .map(|d| p + d.to_ivec_delta())
        .collect()
}

fn parse(input: &str) -> IResult<&str, Vec<((Direction, u8), &str)>> {
    let (input, a) = separated_list1(
        newline,
        separated_pair(
            separated_pair(map(cnom::anychar, |c| Direction::from(c)), space1, cnom::u8),
            space1,
            delimited(tag("("), preceded(tag("#"), alphanumeric1), tag(")")),
        ),
    )(input)?;

    Ok((input, a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(solve(ti), "Answer".to_string());
    }
}
