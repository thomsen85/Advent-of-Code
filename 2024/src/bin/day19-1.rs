use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

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
    for want in wanted.into_iter().progress() {
        let mut mem = HashSet::new();
        let mut p = false;
        let mut stack = vec![want];

        while let Some(curr) = stack.pop() {
            mem.insert(curr);
            for a in &avaliable {
                if let Some(n_c) = curr.strip_prefix(a) {
                    if n_c.is_empty() {
                        p = true;
                        break;
                    }

                    if mem.contains(&n_c) {
                        continue;
                    }
                    stack.push(n_c);
                }
            }
            if p {
                break;
            }
        }

        if p {
            s += 1;
        }
    }

    s.to_string()
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
        assert_eq!(solve(ti), "6".to_string());
    }
}
