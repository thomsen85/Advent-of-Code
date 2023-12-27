use geo::{polygon, Area, LineString, Polygon};
use glam::IVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take, take_while_m_n},
    character::complete::{alpha1, alphanumeric1, anychar, newline, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
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
            Up => [0, 1],
            Down => [0, -1],
            Left => [-1, 0],
            Right => [1, 0],
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
    use Direction::*;
    dbg!(input);
    let mut instructions = parse(input).unwrap().1;
    instructions.push(instructions[0]);
    instructions.push(instructions[1]);

    let mut pos = IVec2::new(0, 0);
    let mut history = Vec::new();

    for (current, next) in instructions.iter().tuple_windows() {
        dbg!(current);
        let offset = IVec2::from_array(match &[current.2 .1, next.2 .1] {
            [Right, Down] | [Down, Right] => [1, 1],
            [Down, Left] | [Left, Down] => [1, 0],
            [Left, Up] | [Up, Left] => [0, 0],
            [Up, Right] | [Right, Up] => [0, 1],
            _ => panic!(),
        });
        pos += current.2 .1.to_ivec_delta() * current.2 .0;
        history.push(pos + offset);
    }

    for h in &history {
        println!("{}\t{}", h.x, h.y);
    }
    let ls = LineString::from(
        history
            .iter()
            .map(|a| (a.x as f64, a.y as f64))
            .collect_vec(),
    );

    let poly = Polygon::new(ls, vec![]);

    (poly.unsigned_area() as usize).to_string()
}
fn from_hex(input: &str) -> Result<i32, std::num::ParseIntError> {
    i32::from_str_radix(input, 16)
}

fn hex_color(input: &str) -> IResult<&str, (i32, Direction)> {
    let (input, _) = tag("#")(input)?;
    let (input, num) = map(take(5_usize), |a| from_hex(a).unwrap())(input)?;
    // 0 means R, 1 means D, 2 means L, and 3 means U
    use Direction::*;
    let (input, direction) = map(cnom::u8, |a| match a {
        0 => Right,
        1 => Down,
        2 => Left,
        3 => Up,
        _ => panic!(),
    })(input)?;

    Ok((input, (num, direction)))
}

fn parse(input: &str) -> IResult<&str, Vec<(Direction, i32, (i32, Direction))>> {
    separated_list1(
        newline,
        tuple((
            map(anychar, |c| Direction::from(c)),
            delimited(space1, cnom::i32, space1),
            delimited(cnom::char('('), hex_color, cnom::char(')')),
        )),
    )(input)
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
        assert_eq!(solve(ti), "952408144115".to_string());
    }
}
