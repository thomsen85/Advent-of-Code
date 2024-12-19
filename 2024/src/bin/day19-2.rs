use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{collections::HashMap, time::Instant};
// For number types

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day19.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let (avaliable, wanted) = input.split_once("\n\n").unwrap();
    let avaliable = avaliable.split(", ").collect_vec();
    let wanted = wanted.lines().collect_vec();

    let mut s = 0;
    let mut mem = HashMap::new();
    for want in wanted.into_iter().progress() {
        s += dfs_m(want, &avaliable, &mut mem);
    }

    s.to_string()
}

fn dfs_m(curr: &str, avaliable: &[&str], mem: &mut HashMap<String, usize>) -> usize {
    if let Some(&m) = mem.get(curr) {
        return m;
    }

    if curr.is_empty() {
        return 1;
    }

    let mut s = 0;

    for a in avaliable {
        if let Some(n_c) = curr.strip_prefix(a) {
            s += dfs_m(n_c, avaliable, mem);
        }
    }

    mem.insert(curr.to_owned(), s.to_owned());
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(solve(ti), "16".to_string());
    }
}
