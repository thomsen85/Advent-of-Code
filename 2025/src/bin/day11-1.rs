use std::{collections::HashMap, time::Instant};

use itertools::Itertools;

fn main() {
    let now = Instant::now();
    let res = solve(include_str!("../../inputs/day11.txt"));
    let elapsed = now.elapsed();
    dbg!(res);
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let g = input
        .trim()
        .split("\n")
        .map(|l| l.split_once(": ").unwrap())
        .map(|(a, l)| {
            (
                a.to_string(),
                l.split(" ").map(|s| s.to_string()).collect_vec(),
            )
        })
        .collect::<HashMap<String, Vec<String>>>();

    let mut counter = 0;

    dfs(&g, "you", &mut counter);

    counter.to_string()
}

fn dfs(g: &HashMap<String, Vec<String>>, curr: &str, counter: &mut usize) {
    if curr == "out" {
        *counter += 1;
        return;
    }

    for next in g.get(curr).unwrap() {
        dfs(g, next, counter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(solve(ti), "5".to_string());
    }
}
