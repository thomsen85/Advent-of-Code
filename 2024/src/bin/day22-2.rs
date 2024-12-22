use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};
// For number types

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day22.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

const N: i64 = 2000;
fn solve(input: &str) -> String {
    let sequences = input
        .lines()
        .map(|a| a.parse::<i64>().unwrap())
        .map(|mut i| {
            (0..N + 1)
                .map(move |_| {
                    let r = i * 64;
                    i ^= r;
                    i %= 16777216;

                    let r = i / 32;
                    i ^= r;
                    i %= 16777216;

                    let r = i * 2048;
                    i ^= r;
                    i %= 16777216;
                    i
                })
                .map(|a| a % 10)
                .collect_vec()
        })
        .collect_vec();

    let mut m = HashMap::new();
    sequences.into_iter().for_each(|seq| {
        let mut seen = HashSet::new();
        for (a, b, c, d, e) in seq.into_iter().tuple_windows() {
            let s = [b - a, c - b, d - c, e - d];
            if seen.contains(&s) {
                continue;
            }
            seen.insert(s);
            *m.entry(s).or_insert(0) += e;
        }
    });

    dbg!(m.iter().max_by(|a, b| a.1.cmp(b.1)))
        .unwrap()
        .1
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "1
2
3
2024";
        assert_eq!(solve(ti), "23".to_string());
    }
}
