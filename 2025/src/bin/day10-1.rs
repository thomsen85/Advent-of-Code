use std::{collections::VecDeque, time::Instant};

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
                .map(|x| x.parse::<i64>().unwrap())
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
    for (target_schema, buttons, joltage) in lines {
        let mut ans_found = false;
        let mut queue = VecDeque::from(vec![(0, vec!['.'; target_schema.len()])]);

        while let Some((depth, schema)) = queue.pop_front() {
            for button in &buttons {
                let mut new_schema = schema.clone();

                for &light_i in button {
                    new_schema[light_i] = if new_schema[light_i] == '.' { '#' } else { '.' };
                }

                if new_schema == target_schema {
                    presses += depth + 1;
                    ans_found = true;
                    break;
                }

                queue.push_back((depth + 1, new_schema));
            }
            if ans_found {
                break;
            }
        }
    }
    // .sum::<usize>()
    presses.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve(ti), "7".to_string());
    }
}
