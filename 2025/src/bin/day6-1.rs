use std::time::Instant;

use itertools::Itertools;

fn main() {
    let now = Instant::now();
    let res = solve(include_str!("../../inputs/day6.txt"));
    let elapsed = now.elapsed();
    dbg!(res);
    println!("Time used {elapsed:?}");
}
//16 43
fn solve(input: &str) -> String {
    let l = input
        .trim()
        .split("\n")
        .map(|line| line.trim().split_whitespace().collect_vec())
        .collect_vec();

    let nums = l
        .iter()
        .take(4)
        .map(|line| {
            line.iter()
                .map(|n| {
                    n.parse::<usize>()
                        .unwrap_or_else(|_| panic!("{} not a num", n))
                })
                .collect_vec()
        })
        .collect_vec();

    let operators = l[4].clone();

    operators
        .iter()
        .enumerate()
        .map(|(i, op)| match *op {
            "*" => (0..4).map(|j| nums[j][i]).product::<usize>(),
            "+" => (0..4).map(|j| nums[j][i]).sum::<usize>(),
            _ => panic!("{} not operator", op),
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(solve(ti), "4277556".to_string());
    }
}
