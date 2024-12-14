use common::strings::string_to_extracted_nums_t_vec;
use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day14.txt"), false));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str, test: bool) -> String {
    let robots = input
        .lines()
        .map(|line| string_to_extracted_nums_t_vec(line).try_into().unwrap())
        .map(|[x, y, vx, vy]: [i64; 4]| (x, y, vx, vy))
        .collect_vec();

    dbg!(&robots);

    let mut w: i64 = 101;
    let mut h: i64 = 103;
    if test {
        w = 11;
        h = 7;
    }

    for hi in 0..h {
        for wi in 0..w {
            print!(
                "{}",
                robots.iter().filter(|a| a.0 == wi && a.1 == hi).count()
            )
        }
        println!();
    }
    let mut poss = Vec::new();
    for r in robots {
        let mut last_pos = (r.0, r.1);
        for _i in 0..100 {
            last_pos = (
                (last_pos.0 + r.2).rem_euclid(w),
                (last_pos.1 + r.3).rem_euclid(h),
            )
        }
        poss.push(last_pos);
    }

    println!();
    for hi in 0..h {
        for wi in 0..w {
            print!("{}", poss.iter().filter(|a| a.0 == wi && a.1 == hi).count())
        }
        println!();
    }

    dbg!(poss
        .iter()
        .filter_map(|(x, y)| {
            let wh = w / 2;
            let hh = h / 2;
            dbg!(wh, hh);
            if *x == wh || *y == hh {
                return None;
            }

            dbg!(x, y);
            Some(dbg!(x / (wh + 1), y / (hh + 1)))
        })
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        }))
    .into_values()
    .product::<i64>()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(solve(ti, true), "12".to_string());
    }
}
