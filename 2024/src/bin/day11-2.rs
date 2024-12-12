use std::{collections::HashMap, time::Instant};

use common::strings::string_to_t_grid;

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day11.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let m = string_to_t_grid::<usize>(input, " ")
        .first()
        .unwrap()
        .clone();

    let mut vis = HashMap::new();
    let mut sum = 0;
    for n in m {
        sum += dfs(n, 75, &mut vis);
    }

    sum.to_string()
}

fn dfs(num: usize, left: usize, mem: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(ans) = mem.get(&(num, left)) {
        return *ans;
    }
    if left == 0 {
        return 1;
    }
    if num == 0 {
        let amt = dfs(1, left - 1, mem);
        mem.insert((num, left), amt);
        return amt;
    }
    let num_s = num.to_string();
    if num_s.len() % 2 == 0 {
        let a = dfs(
            num_s[..num_s.len() / 2].parse::<usize>().unwrap(),
            left - 1,
            mem,
        );
        let b = dfs(
            num_s[num_s.len() / 2..].parse::<usize>().unwrap(),
            left - 1,
            mem,
        );
        mem.insert((num, left), a + b);
        return a + b;
    }

    let last = dfs(num * 2024, left - 1, mem);
    mem.insert((num, left), last);
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "125 17";
        // Answer was found after task was solved
        assert_eq!(solve(ti), "65601038650482".to_string());
    }
}
