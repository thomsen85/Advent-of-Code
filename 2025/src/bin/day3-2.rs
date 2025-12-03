use itertools::Itertools;
use std::time::Instant;

// Delta time: 9min 24sec
fn main() {
    let now = Instant::now();
    let result = solve(include_str!("../../inputs/day3.txt"));
    let elapsed = now.elapsed();
    dbg!(result);
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let l = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec();

            max_digit(&l, 0, 12).parse::<usize>().unwrap()
        })
        .sum::<usize>()
        .to_string()
}

fn max_digit(list: &[usize], start_i: usize, depth_left: usize) -> String {
    if depth_left == 0 {
        return String::new();
    }
    let mut max = 0;
    let mut max_i = 0;
    let len = list.len();
    for i in start_i..=(len - depth_left) {
        if list[i] > max {
            max_i = i;
            max = list[i];
        }
    }
    max.to_string() + &max_digit(list, max_i + 1, depth_left - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve(ti), "3121910778619".to_string());
    }
}
