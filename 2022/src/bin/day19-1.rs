#![feature(duration_millis_float)]
use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use glam::IVec4;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take, take_till, take_while},
    character::{
        complete::{digit1, multispace0, multispace1, newline, space0, space1},
        is_alphabetic, is_space,
    },
    combinator::{map, not},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    AsChar, IResult, Parser,
};
// For number types
use nom::character::complete as cnom;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    robots: IVec4,
    minute: u16,
    resources: IVec4, // ore, clay, obsidian, geode
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    ore_robot: i32,             // ore
    clay_robot: i32,            // ore
    obsidian_robot: (i32, i32), // ore, clay
    geode_robot: (i32, i32),    // ore, obsidian
}

fn buy_possiblities(blueprint: &Blueprint, state: &State) -> Vec<(IVec4, IVec4)> {
    let ore_robot = IVec4::new(blueprint.ore_robot, 0, 0, 0);
    let clay_robot = IVec4::new(blueprint.clay_robot, 0, 0, 0);
    let obsidian_robot = IVec4::new(blueprint.obsidian_robot.0, blueprint.obsidian_robot.1, 0, 0);
    let geode_robot = IVec4::new(blueprint.geode_robot.0, 0, blueprint.geode_robot.1, 0);
    let collection = [ore_robot, clay_robot, obsidian_robot, geode_robot];

    let max_necessity = collection.iter().fold(IVec4::ZERO, |acc, x| acc.max(*x));

    // [4, 2, 1, 0] - [5, 0, 0, 0] = [-1, 2, 1, 0]
    let ignore = (max_necessity - state.robots).to_array();

    collection
        .into_iter()
        .enumerate()
        .filter(|(i, r)| (state.resources - *r).min_element() >= 0 && (*i == 3 || ignore[*i] > 0))
        .map(|(i, r)| {
            let mut v = [0; 4];
            v[i] = 1;
            (IVec4::from(v), r)
        })
        .collect_vec()
}

const MINUTES: u16 = 24;

fn main() {
    let timer = Instant::now();
    dbg!(solve(include_str!("../../inputs/day19.txt")));
    let elapsed = timer.elapsed();
    println!("Took {:.2} ms", elapsed.as_millis_f64())
}

fn solve(input: &str) -> String {
    let p = parse(input).unwrap().1;
    dbg!(&p);

    p.par_iter()
        .map(|blueprint| {
            let mut cache = HashSet::new();
            let mut queue = VecDeque::new();

            queue.push_back(State {
                robots: IVec4::new(1, 0, 0, 0),
                minute: MINUTES,
                resources: IVec4::ZERO,
            });

            let mut best_case = 0;

            while let Some(current) = queue.pop_front() {
                if current.minute == 0 {
                    if best_case < current.resources.w {
                        best_case = current.resources.w
                    }
                    continue;
                }
                for (robot, cost) in buy_possiblities(blueprint, &current) {
                    let state = State {
                        resources: current.resources + current.robots - cost,
                        robots: current.robots + robot,
                        minute: current.minute - 1,
                    };
                    if cache.contains(&state) {
                        continue;
                    }
                    cache.insert(state.clone());
                    queue.push_back(state);
                }

                let state = State {
                    resources: current.robots + current.resources,
                    minute: current.minute - 1,
                    ..current
                };

                if cache.contains(&state) {
                    continue;
                }
                cache.insert(state.clone());

                queue.push_back(state);
            }

            best_case * blueprint.id
        })
        .progress()
        .sum::<i32>()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(
        newline,
        map(
            tuple((
                preceded(tag("Blueprint "), cnom::i32),
                preceded(tag(": Each ore robot costs "), cnom::i32),
                preceded(tag(" ore. Each clay robot costs "), cnom::i32),
                preceded(tag(" ore. Each obsidian robot costs "), cnom::i32),
                preceded(tag(" ore and "), cnom::i32),
                preceded(tag(" clay. Each geode robot costs "), cnom::i32),
                delimited(tag(" ore and "), cnom::i32, tag(" obsidian.")),
            )),
            |l| {
                dbg!(Blueprint {
                    id: l.0,
                    ore_robot: l.1,
                    clay_robot: l.2,
                    obsidian_robot: (l.3, l.4),
                    geode_robot: (l.5, l.6),
                })
            },
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        assert_eq!(solve(ti), "33".to_string());
    }
}
