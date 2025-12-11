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

    let mut memo = HashMap::new();
    dfs(&g, "svr", false, false, &mut memo).to_string()
}

fn dfs(
    g: &HashMap<String, Vec<String>>,
    curr: &str,
    mut visited_dac: bool,
    mut visited_fft: bool,
    memo: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    if let Some(p) = memo.get(&(curr.to_string(), visited_dac, visited_fft)) {
        return *p;
    }

    match curr {
        "out" => {
            if visited_fft && visited_dac {
                return 1;
            }
            return 0;
        }
        "dac" => visited_dac = true,
        "fft" => visited_fft = true,
        _ => (),
    }

    let res = g
        .get(curr)
        .unwrap()
        .iter()
        .map(|next| dfs(g, next, visited_dac, visited_fft, memo))
        .sum();

    memo.insert((curr.to_string(), visited_dac, visited_fft), res);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!(solve(ti), "2".to_string());
    }
}
