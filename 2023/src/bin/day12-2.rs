use std::collections::VecDeque;

use indicatif::{ProgressBar, ProgressIterator};
use itertools::Itertools;

use rayon::prelude::*;

fn main() {
    dbg!(solve(include_str!("../../inputs/day12.txt")));
}

fn solve(input: &str) -> String {
    let mut sum = 0;
    let p = ProgressBar::new(input.lines().count() as u64);
    let lines = input.lines().collect_vec();
    lines
        .par_iter()
        .map(|line| {
            let (pattern, arr) = line.split_once(" ").unwrap();

            let mut pattern = (format!("{}?", pattern)).repeat(5);
            let mut arr = (format!("{},", arr)).repeat(5);
            let l1 = pattern.len();
            let l2 = arr.len();
            pattern.remove(l1 - 1);
            arr.remove(l2 - 1);

            let mut arr = arr
                .split(",")
                .map(|a| a.parse::<i32>().unwrap())
                .collect::<VecDeque<_>>();
            arr.push_front(0);

            let mut valids = Vec::new();
            backtrack(&pattern, &pattern, arr, true, &mut valids);
            p.inc(1);
            valids.len()
        })
        .sum::<usize>()
        .to_string()
}

fn backtrack(
    pattern: &str,
    original_pattern: &str,
    mut arr: VecDeque<i32>,
    last_space: bool,
    valid: &mut Vec<String>,
) {
    // dbg!(&pattern, &original_pattern, &arr, &last_space, &valid);
    if arr.iter().sum::<i32>() == 0 && pattern.chars().all(|a| a == '.') {
        valid.push(original_pattern.to_string());
        return;
    }

    if pattern.len() == 0 {
        return;
    }
    if arr.len() == 0 {
        return;
    }
    if arr.iter().skip(2).fold(0, |acc, x| acc + x + 1) > pattern.len() as i32 {
        return;
    }

    let current = pattern.chars().next().unwrap();
    let new_pat = &pattern[1..];

    match current {
        '?' => {
            backtrack(
                &pattern.replacen('?', ".", 1),
                &original_pattern.replacen('?', ".", 1),
                arr.clone(),
                last_space,
                valid,
            );
            backtrack(
                &pattern.replacen('?', "#", 1),
                &original_pattern.replacen('?', "#", 1),
                arr,
                last_space,
                valid,
            );
        }
        '.' => backtrack(new_pat, original_pattern, arr, true, valid),
        '#' => {
            if last_space {
                if arr[0] != 0 || arr.len() <= 1 {
                    return;
                }
                arr.pop_front();

                arr[0] -= 1;
                backtrack(new_pat, original_pattern, arr, false, valid);
            } else {
                arr[0] -= 1;
                if arr[0] < 0 {
                    return;
                }
                backtrack(new_pat, original_pattern, arr, false, valid);
            }
        }
        oof => panic!("{} was not expected", oof),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(solve(ti), "525152".to_string());
    }
}
