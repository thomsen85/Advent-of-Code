use common::strings::string_to_extracted_nums_t_vec;
use itertools::Itertools;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day14.txt"), false));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str, test: bool) -> String {
    let mut robots = input
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

    let mut i = 0;

    let mut ss = 0;
    loop {
        i += 1;
        robots.iter_mut().for_each(|r| {
            r.0 = (r.0 + r.2).rem_euclid(w);
            r.1 = (r.1 + r.3).rem_euclid(h);
        });

        let a = robots.iter().fold(vec![0; w as usize], |mut acc, p| {
            acc[p.0 as usize] += 1;
            acc
        });

        let max = *a.iter().max().unwrap();
        let min = *a.iter().min().unwrap();
        let threashold = 35;

        if max - min > threashold {
            ss += 1;
            println!("{}", i);
            for hi in 0..h {
                for wi in 0..w {
                    print!(
                        "{}",
                        if robots.iter().filter(|a| a.0 == wi && a.1 == hi).count() == 0 {
                            " ".to_string()
                        } else {
                            robots
                                .iter()
                                .filter(|a| a.0 == wi && a.1 == hi)
                                .count()
                                .to_string()
                        }
                    )
                }
                println!();
            }
        }

        if i > 30_000 {
            break;
        }
    }
    dbg!(ss);

    " ".to_string()
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
