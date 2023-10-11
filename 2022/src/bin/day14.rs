use std::collections::HashSet;

use common::datastructs::Vec2::Vec2;
use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair, IResult,
};

struct SandSim {
    rocks: HashSet<Vec2>,
    sand: HashSet<Vec2>,
}

impl SandSim {
    fn new(rocks: HashSet<Vec2>) -> Self {
        Self {
            rocks,
            sand: HashSet::new(),
        }
    }

    fn spawn_new(&mut self, at: Vec2) -> bool {
        let mut sand = at.clone();
        let mut is_settled = false;

        while !is_settled {
            if !self.intersects(&(sand + Vec2::DOWN)) {
                sand = sand + Vec2::DOWN;
            } else if !self.intersects(&(sand + Vec2::DOWN_LEFT)) {
                sand = sand + Vec2::DOWN_LEFT;
            } else if !self.intersects(&(sand + Vec2::DOWN_RIGHT)) {
                sand = sand + Vec2::DOWN_RIGHT;
            } else {
                is_settled = true;
            }
            if sand.y > 1000 {
                return false;
            }
        }
        self.sand.insert(sand);
        true
    }

    fn intersects(&self, point: &Vec2) -> bool {
        if self.rocks.contains(point) {
            return true;
        } else if self.sand.contains(point) {
            return true;
        }
        false
    }
}

fn main() {
    let input: String = common::utils::string_from_file("inputs/day14.txt");
    //const INPUT: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let rocks = get_rock_formation(input);
    let mut sim = SandSim::new(rocks);
    let mut counter = 0;

    while sim.spawn_new(Vec2::new(500, 0)) {
        counter += 1;
    }

    println!("Amount = {}", counter);
}

fn part2(input: &str) {
    let mut rocks = get_rock_formation(input);
    let highest_y = rocks.iter().map(|r| r.y).max().unwrap();

    for x in -10000..10000 {
        rocks.insert(Vec2::new(x, highest_y + 2));
    }
    let mut sim = SandSim::new(rocks);
    let mut counter = 0;

    while !sim.sand.contains(&Vec2::new(500, 0)) {
        sim.spawn_new(Vec2::new(500, 0));
        counter += 1;
    }

    println!("Amount = {}", counter);
}

fn get_complete_formation(corners: &Vec<Vec2>) -> HashSet<Vec2> {
    let mut set = HashSet::new();
    for i in 0..(corners.len() - 1) {
        let root = corners[i];
        let diff = corners[i + 1] - root;

        let len = diff.len() as i32;
        let direction = diff / len;

        for i in 0..=len {
            set.insert(root + direction * i);
        }
    }
    set
}

fn get_rock_formation(input: &str) -> HashSet<Vec2> {
    let parsed_input = rock_parser(input).unwrap().1;
    let mut complete_rock_formations = HashSet::new();
    for corners in &parsed_input {
        complete_rock_formations.extend(get_complete_formation(corners));
    }
    complete_rock_formations
}

fn rock_parser(input: &str) -> IResult<&str, Vec<Vec<Vec2>>> {
    separated_list1(newline, rock_formation)(input)
}

fn rock_formation(input: &str) -> IResult<&str, Vec<Vec2>> {
    let (input, out) = separated_list1(
        tag(" -> "),
        separated_pair(
            nom::character::complete::i32,
            tag(","),
            nom::character::complete::i32,
        ),
    )(input)?;
    let vec = out.into_iter().map(|x| x.into()).collect();
    Ok((input, vec))
}

fn get_drawing(
    start_pos: Vec2,
    end_pos: Vec2,
    rocks: HashSet<Vec2>,
    sand: HashSet<Vec2>,
) -> String {
    let mut out = String::new();
    for row in start_pos.y..=end_pos.y {
        for col in start_pos.x..=end_pos.x {
            let pos = Vec2::new(col, row);
            if rocks.contains(&pos) {
                out.push('#');
            } else if sand.contains(&pos) {
                out.push('O')
            } else {
                out.push(' ');
            }
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashSet;

    use crate::{get_drawing, get_rock_formation, Vec2};

    #[test]
    fn test_rocks() {
        const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let rocks = get_rock_formation(INPUT);

        println!(
            "{}",
            get_drawing(Vec2::new(470, 0), Vec2::new(530, 50), rocks, HashSet::new())
        );
        assert!(false)
    }
}
