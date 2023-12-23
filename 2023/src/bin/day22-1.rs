use std::collections::{HashMap, HashSet};

use glam::{IVec2, IVec3};
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
    // 501 : To High :(
    // 635 : Too High :(
    // 788
    dbg!(solve(include_str!("../../inputs/day22.txt")));
}

fn solve(input: &str) -> String {
    let mut bricks = input
        .lines()
        .map(|line| line.split_once("~").unwrap())
        .map(|(l, r)| (split_and_parse(l), split_and_parse(r)))
        .map(|(l, r)| to_cubes(l, r))
        .collect_vec();

    //dbg!(&bricks);

    let mut supports = Vec::new();
    loop {
        println!("Loop Start");
        supports.clear();

        let mut changed = false;

        // Per Block in list
        for brick_i in 0..bricks.len() {
            let brick_name = (brick_i + 65) as u8 as char;

            // println!("Looking at brick {}", brick_name);

            let mut crash = false;

            // Per Other Block in list
            for other_i in 0..bricks.len() {
                let other_name = (other_i + 65) as u8 as char;
                if brick_i == other_i {
                    continue;
                }
                // println!("Comparing to brick: {}", other_name);

                // Per block
                for c in bricks[brick_i].iter() {
                    if c.z <= 1 {
                        // println!("Brick is at bottom");
                        crash = true;
                        break;
                    }

                    // Per other block
                    for b in &bricks[other_i] {
                        // println!("\tComparing {:?} to {:?}", *c + IVec3::new(0, 0, -1), *b);
                        if *c + IVec3::new(0, 0, -1) == *b {
                            //dbg!("HEllo", c, b);
                            // println!("Brick is crashing with {}", other_name);
                            supports.push((brick_i, other_i));
                            crash = true;
                            break;
                        }
                    }
                    // if crash {
                    //     break;
                    // }
                }

                // if crash {
                //     break;
                // }
            }
            if !crash {
                // println!("Moved {}", brick_i);
                bricks[brick_i] = bricks[brick_i]
                    .iter()
                    .map(|c| *c + IVec3::new(0, 0, -1))
                    .collect_vec();
                changed = true;
            }
            // if crash {
            //     break;
            // }
        }
        if !changed {
            break;
        }
    }
    //dbg!(&bricks);
    //dbg!(&supports);
    // for support in &supports {
    //     println!(
    //         "{} is supporting {}",
    //         (support.1 + 65) as u8 as char,
    //         (support.0 + 65) as u8 as char,
    //     );
    // }

    // let mut single_supporters = HashSet::new();
    // for i in 0..bricks.len() {
    //     // Alle som kun blir støttet av en
    //     let supported_by = supports
    //         .iter()
    //         .filter(|(top, _bottom)| *top == i)
    //         .collect_vec();
    //     if supported_by.len() == 1 {
    //         single_supporters.insert(supported_by[0].1);
    //     }
    //     dbg!(i, supported_by);
    // }

    let supporting = (0..bricks.len())
        .map(|brick_id| {
            supports
                .iter()
                .filter(|(_top, bottom)| *bottom == brick_id)
                .collect_vec()
        })
        .collect_vec();

    let supported_by = (0..bricks.len())
        .map(|brick_id| {
            supports
                .iter()
                .filter(|(top, _bottom)| *top == brick_id)
                .collect_vec()
        })
        .collect_vec();

    let mut sum = 0;

    for (i, sup) in supporting.iter().enumerate() {
        // let mut s = true;
        // if sup.is_empty() {
        //     continue;
        // }
        // dbg!(&sup);
        // for (top, _bottom) in sup {
        //     dbg!(&supported_by[*top]);
        //     if supported_by[*top].len() > 1 {
        //         s = false;
        //         break;
        //     }
        // }
        // if s {
        //     dbg!(i);
        //     sum += 1
        // }

        // All of the supported bricks must have another one
        let mut a = true;
        for (top, _bottom) in sup {
            a = !supported_by[*top].iter().all(|(top, bottom)| *bottom == i)
        }
        if a {
            dbg!(i);
            sum += 1;
        }
    }

    sum.to_string()
}

fn display_bricks_xz(bricks: &Vec<Vec<IVec3>>) {
    for z in (1..10).rev() {
        for x in 0..3 {
            if let Some(a) = bricks
                .iter()
                .enumerate()
                .flat_map(|(i, b)| b.iter().map(move |c| (i, IVec2::new(c.x, c.z))))
                .find(|(_i, b)| b == &IVec2::new(x, z))
            {
                print!("{}", char::from_u32(a.0 as u32 + 65).unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("---");
}

fn display_bricks_yz(bricks: &Vec<Vec<IVec3>>) {
    for z in (1..10).rev() {
        for y in 0..3 {
            if let Some(a) = bricks
                .iter()
                .enumerate()
                .flat_map(|(i, b)| b.iter().map(move |c| (i, IVec2::new(c.y, c.z))))
                .find(|(_i, b)| b == &IVec2::new(y, z))
            {
                print!("{}", char::from_u32(a.0 as u32 + 65).unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("---");
}
fn split_and_parse(line: &str) -> (i32, i32, i32) {
    line.split(",")
        .take(3)
        .map(|a| a.parse::<i32>().unwrap())
        .collect_tuple::<(i32, i32, i32)>()
        .unwrap()
}

fn to_cubes(from: (i32, i32, i32), to: (i32, i32, i32)) -> Vec<IVec3> {
    let mut res = Vec::new();
    for x in from.0..=to.0 {
        for y in from.1..=to.1 {
            for z in from.2..=to.2 {
                res.push(IVec3::new(x, y, z));
            }
        }
    }
    res
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
        assert_eq!(solve(ti), "5".to_string());
    }
}
