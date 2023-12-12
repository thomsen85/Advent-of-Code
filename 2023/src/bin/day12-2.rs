use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    dbg!(solve(include_str!("../../inputs/day12.txt")));
}

fn solve(input: &str) -> String {
    // let p = ProgressBar::new(input.lines().count() as u64);
    let lines = input.lines().collect_vec();
    let mut cache = HashMap::new();
    lines
        .into_iter()
        .map(|line| {
            let (pattern, arr) = line.split_once(" ").unwrap();

            let mut pattern = (format!("{}?", pattern)).repeat(5);
            let mut arr = (format!("{},", arr)).repeat(5);
            let l1 = pattern.len();
            let l2 = arr.len();
            pattern.remove(l1 - 1);
            arr.remove(l2 - 1);

            let arr = arr
                .split(",")
                .map(|a| a.parse::<i32>().unwrap())
                .collect_vec();

            // p.inc(1);
            backtrack(&pattern, arr, &mut cache)
        })
        .sum::<usize>()
        .to_string()
}

fn backtrack(
    pattern: &str,
    arr: Vec<i32>,
    cache: &mut HashMap<(String, Vec<i32>), usize>,
) -> usize {
    let key = (pattern.to_string(), arr.clone());

    if let Some(valid) = cache.get(&key) {
        return *valid;
    }

    if arr.is_empty() && pattern.is_empty() {
        return 1;
    }

    if !pattern.contains("#") && arr.is_empty() {
        return 1;
    }

    if arr.is_empty() || pattern.is_empty() {
        return 0;
    }

    let current = pattern.chars().next().unwrap();

    let mut valid = 0;
    if "?.".contains(current) {
        valid += backtrack(&pattern[1..], arr.clone(), cache);
    }
    if "?#".contains(current) {
        if arr[0] <= pattern.len() as i32
            && !&pattern[..(arr[0] as usize)].contains('.')
            && (arr[0] == pattern.len() as i32
                || pattern.chars().nth(arr[0] as usize).unwrap() != '#')
        {
            if arr[0] == pattern.len() as i32 {
                valid += backtrack("", arr[1..].to_vec(), cache)
            } else {
                valid += backtrack(&pattern[(arr[0] as usize + 1)..], arr[1..].to_vec(), cache);
            }
        }
    }

    cache.insert(key, valid);

    valid
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
