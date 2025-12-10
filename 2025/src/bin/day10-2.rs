use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::Write,
    time::Instant,
};

use itertools::Itertools;

fn main() {
    let now = Instant::now();
    let res = solve(include_str!("../../inputs/day10.txt"));
    let elapsed = now.elapsed();
    dbg!(res);
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let lines = input
        .trim()
        .split("\n")
        .map(|line| {
            let (schema, rest) = line.split_once("]").unwrap();
            let schema = schema[1..].chars().collect_vec();
            let (rest, joltage) = rest.split_once("{").unwrap();
            let joltage = joltage[..(joltage.len() - 1)]
                .split(",")
                .map(|x| x.parse::<i16>().unwrap())
                .collect_vec();

            let buttons = rest
                .replace(" ", "")
                .split(")")
                .take_while(|x| !x.is_empty())
                .map(|btn| {
                    btn[1..]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            (schema, buttons, joltage)
        })
        .collect_vec();

    let mut presses = 0;
    println!("1");
    for (_target_schema, buttons, target_joltage) in lines {
        let mut memo = HashSet::new();
        dbg!(&buttons, &target_joltage);
        presses += dbg!(
            dfs(
                target_joltage.clone(),
                &buttons
                    .into_iter()
                    .sorted_by_key(|k| k.len())
                    .rev()
                    .collect_vec(),
                0,
                &mut memo
            )
            .unwrap()
        );
    }
    // .sum::<usize>()
    presses.to_string()
}

fn dfs(
    joltage: Vec<i16>,
    buttons: &Vec<Vec<usize>>,
    presses: usize,
    memo: &mut HashSet<Vec<i16>>,
) -> Option<usize> {
    if memo.contains(&joltage) {
        // println!("{:?}", &joltage);
        return None;
    }
    memo.insert(joltage.clone());

    let mut new_presses = Vec::new();
    for button in buttons {
        let mut new_joltage = joltage.clone();

        let mut invalid = false;
        for &joltage_i in button {
            let val = &mut new_joltage[joltage_i];
            if *val == 0 {
                invalid = true;
                break;
            }
            *val -= 1;
        }
        if invalid {
            continue;
        }

        if new_joltage.iter().all(|j| *j == 0) {
            return Some(presses + 1);
        }
        new_presses.push(new_joltage);

        // // Can never recover if any is > target
        // if new_joltage
        //     .iter()
        //     .zip(target_joltage)
        //     .any(|(new, target)| new > target)
        // {
        //     continue;
        // }
    }
    new_presses.sort_by_key(|a| a.iter().map(|i| i.pow(2)).sum::<i16>());
    for new_joltage in new_presses {
        let res = dfs(new_joltage, buttons, presses + 1, memo);
        if res.is_some() {
            return res;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve(ti), "33".to_string());
    }
}
