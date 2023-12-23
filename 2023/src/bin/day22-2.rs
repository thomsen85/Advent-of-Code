use std::{
    collections::HashSet,
    ops::{Add, Range, RangeInclusive},
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
// For number types
use nom::character::complete as cnom;

#[derive(Debug, Clone)]
struct Brick {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl Brick {
    fn intersects(&self, rhs: &Self) -> bool {
        self.x.clone().any(|x| rhs.x.contains(&x))
            && self.y.clone().any(|y| rhs.y.contains(&y))
            && self.z.clone().any(|z| rhs.z.contains(&z))
    }
}

impl Add<(i32, i32, i32)> for Brick {
    type Output = Brick;

    fn add(self, rhs: (i32, i32, i32)) -> Self::Output {
        let x = (self.x.start() + rhs.0)..=(self.x.end() + rhs.0);
        let y = (self.y.start() + rhs.1)..=(self.y.end() + rhs.1);
        let z = (self.z.start() + rhs.2)..=(self.z.end() + rhs.2);
        Brick { x, y, z }
    }
}

fn main() {
    // 102227: too high :(
    // 57770
    dbg!(solve(include_str!("../../inputs/day22.txt")));
}

fn solve(input: &str) -> String {
    let mut bricks = input
        .lines()
        .map(|l| {
            l.replace("~", ",")
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()
        })
        .map(|l| Brick {
            x: l[0]..=l[3],
            y: l[1]..=l[4],
            z: l[2]..=l[5],
        })
        .collect_vec();

    let mut all_intersects = Vec::new();
    loop {
        all_intersects.clear();
        let mut changed = false;

        // For self
        for s in 0..bricks.len() {
            let s_down = bricks[s].clone() + (0, 0, -1);

            // If at bottom
            if *s_down.z.start() < 1 {
                all_intersects.push(Vec::new());
                continue;
            }

            let mut intersecting = Vec::new();

            // For Other
            for o in 0..bricks.len() {
                if s == o {
                    continue;
                }

                if s_down.intersects(&bricks[o]) {
                    intersecting.push(o)
                }
            }

            if intersecting.is_empty() {
                bricks[s] = s_down;
                changed = true;
            }

            all_intersects.push(intersecting);
        }

        if !changed {
            break;
        }
    }

    let mut dont_remove = HashSet::new();

    // dbg!(&all_intersects);
    for inter in &all_intersects {
        if inter.len() <= 1 {
            for dependet_on in inter {
                dont_remove.insert(dependet_on);
            }
        }
    }
    // dbg!(&dont_remove);

    let mut rev_all_intersects = vec![vec![]; all_intersects.len()];
    for i in 0..all_intersects.len() {
        for k in &all_intersects[i] {
            rev_all_intersects[*k].push(i);
        }
    }
    // dbg!(&rev_all_intersects);

    dont_remove
        .into_iter()
        .map(|n| count_dep(&all_intersects, *n))
        .sum::<usize>()
        .to_string()
}

fn count_dep(intersects: &Vec<Vec<usize>>, n: usize) -> usize {
    let mut fallen = HashSet::new();
    fallen.insert(n);

    loop {
        let mut changed = false;
        for (i, inter) in intersects.iter().enumerate() {
            if inter.is_empty() || fallen.contains(&i) {
                continue;
            }
            if inter.iter().all(|a| fallen.contains(&a)) {
                fallen.insert(i);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    fallen.remove(&n);
    // println!("If {} is removed, {:?} falls", n, &fallen);

    fallen.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(solve(ti), "7-".to_string());
    }
}
