use std::collections::HashMap;

use common::utils;
use itertools::Itertools;

fn main() {
    dbg!(solve(include_str!("../../inputs/day14.txt")));
}

fn solve(input: &str) -> String {
    let mut g = utils::string_to_grid(input);
    let mut map_s: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    let cycles: usize = 1000000000;
    let mut cyc: (usize, usize) = (0, 0);
    for i in 0..cycles {
        map_s.insert(g.clone(), i);
        g = transpose_and_fall(&g, false);
        g = transpose_and_fall(&g, false);
        g = transpose_and_fall(&g, true);
        g = transpose_and_fall(&g, true);
        if let Some(cycle) = map_s.get(&g) {
            cyc = (*cycle, i);
            break;
        }
    }

    dbg!(&cyc);
    let g_i = dbg!((cycles - cyc.0) % (cyc.1 - cyc.0) + cyc.0);
    let g = map_s.into_iter().find(|(_, v)| *v == g_i).unwrap().0;
    let g = g
        .iter()
        .map(|line| {
            line.iter().fold(String::new(), |mut acc, n| {
                acc.push(*n);
                acc
            })
        })
        .collect_vec();
    dbg!(&g);
    let sum = g
        .into_iter()
        .rev()
        .enumerate()
        .map(|(m, c)| c.matches("O").count() * (m + 1))
        .sum::<usize>();

    sum.to_string()
}

fn transpose_and_fall(g: &Vec<Vec<char>>, rev: bool) -> Vec<Vec<char>> {
    let mut g = utils::transpose_2d_vec(g.clone())
        .iter()
        .map(|line| {
            line.iter().fold(String::new(), |mut acc, n| {
                acc.push(*n);
                acc
            })
        })
        .collect_vec();

    loop {
        let mut change = false;
        for l in g.iter_mut() {
            let nl = if rev {
                l.replace("O.", ".O")
            } else {
                l.replace(".O", "O.")
            };
            if nl != *l {
                change = true;
                *l = nl;
            }
        }

        if !change {
            break;
        }
    }
    let g = g
        .into_iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    g
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(solve(ti), "64".to_string());
    }
}
