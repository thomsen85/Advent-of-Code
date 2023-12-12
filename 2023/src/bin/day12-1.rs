use std::collections::VecDeque;

use itertools::Itertools;

fn main() {
    dbg!(solve(include_str!("../../inputs/day12.txt")));
}

fn solve(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let (pattern, arr) = line.split_once(" ").unwrap();
        // let pattern = pattern.chars().collect_vec();
        let arr = arr
            .split(",")
            .map(|a| a.parse::<i32>().unwrap())
            .collect_vec();
        let combs = get_all_combinations(pattern);

        for com in combs {
            let mut com_c = com.chars().collect::<VecDeque<_>>();

            let mut arr_c = arr.clone();
            let mut p = 0;

            let mut space = true;
            while let Some(c) = com_c.pop_front() {
                if c == '.' {
                    if !space {
                        p += 1;
                        if p >= arr_c.len() {
                            break;
                        }
                    }
                    space = true;
                    continue;
                }

                if c == '#' {
                    arr_c[p] -= 1;
                    space = false;
                }
            }

            if arr_c.iter().all(|a| *a == 0) && com_c.iter().all(|a| *a == '.') {
                sum += 1;
            }
        }
    }

    sum.to_string()
}

fn get_all_combinations(pattern: &str) -> Vec<String> {
    if !pattern.contains("?") {
        return vec![pattern.to_string()];
    }

    let mut ans = Vec::new();

    ans.append(&mut get_all_combinations(&pattern.replacen("?", ".", 1)));
    ans.append(&mut get_all_combinations(&pattern.replacen("?", "#", 1)));

    return ans;
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
        assert_eq!(solve(ti), "21".to_string());
    }
}
