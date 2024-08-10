use std::fs;

use common::utils::{lines_from_file, string_from_file};
use itertools::Itertools;

fn main() {
    dbg!(solve(include_str!("../../inputs/day8.txt"),));
}

fn solve(input: &str) -> String {
    let mut tot = 0;
    let mut p_tot = 0;

    for line in input.lines() {
        tot += line.len();
        let mut p = 1;
        let mut new_line = Vec::new();
        let c_a = line.chars().collect_vec();
        while p < line.len() - 1 {
            if c_a[p] == '\\' {
                if c_a[p + 1] == 'x' {
                    p += 2
                }
                new_line.push(c_a[p + 1]);
                p += 1;
            } else {
                new_line.push(c_a[p])
            }
            p += 1
        }

        p_tot += new_line.len();
    }

    (tot - p_tot).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
}
