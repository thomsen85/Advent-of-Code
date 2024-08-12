use std::collections::{HashSet, VecDeque};

use glam::IVec3;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};

// For number types
use nom::character::complete as cnom;

fn main() {
    // 1131 low
    dbg!(solve(include_str!("../../inputs/day18.txt")));
}

fn outside_of_boundingbox(point: &IVec3, min: &IVec3, max: &IVec3) -> bool {
    if point.x < min.x
        || point.y < min.y
        || point.z < min.z
        || point.x > max.x
        || point.y > max.y
        || point.z > max.z
    {
        return true;
    }
    false
}

fn get_surrounding(p: IVec3) -> Vec<IVec3> {
    return vec![
        p + IVec3::X,
        p + IVec3::Y,
        p + IVec3::Z,
        p + IVec3::NEG_X,
        p + IVec3::NEG_Y,
        p + IVec3::NEG_Z,
    ];
}

fn solve(input: &str) -> String {
    let mut points = HashSet::new();

    let mut max_point = IVec3::MIN;
    let mut min_point = IVec3::MAX;

    for l in input.lines() {
        let split = l
            .split(",")
            .map(|a| a.parse::<i32>().unwrap())
            .collect_vec();
        let point = IVec3::new(split[0], split[1], split[2]);
        max_point = IVec3::new(
            point.x.max(max_point.x),
            point.y.max(max_point.y),
            point.z.max(max_point.z),
        );
        min_point = IVec3::new(
            point.x.min(min_point.x),
            point.y.min(min_point.y),
            point.z.min(min_point.z),
        );
        points.insert(point);
    }

    // Idea is flood fill from the outside,
    // check if collided, if yes, add to new set of reaced sides (IVec3, IVec3).
    //
    //
    min_point -= IVec3::ONE * 5;
    max_point += IVec3::ONE * 5;
    let mut reached = HashSet::new();
    let mut visited = HashSet::new();
    dbg!(min_point, max_point);

    let mut queue = VecDeque::new();
    queue.push_back(max_point);
    let mut visited2 = 0;

    visited.insert(max_point);
    while let Some(current) = queue.pop_front() {
        visited2 += 1;

        for next in get_surrounding(current)
            .into_iter()
            .filter(|p| !outside_of_boundingbox(p, &min_point, &max_point))
        {
            if points.contains(&next) {
                reached.insert((current, next));
                continue;
            }
            if visited.contains(&next) {
                continue;
            }
            visited.insert(next);
            queue.push_back(next)
        }
    }

    dbg!(visited2);

    reached.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert_eq!(solve(ti), "57".to_string());
    }
}
