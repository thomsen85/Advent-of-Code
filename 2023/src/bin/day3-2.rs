#![allow(dead_code)]

use std::usize;

use nom::{combinator::opt, IResult};

#[derive(Debug, Clone, Copy)]
struct Num {
    num: i32,
    width: i32,
    pos: (i32, i32),
}

#[derive(Debug)]
struct Symbol {
    cont: char,
    width: i32,
    pos: (i32, i32),
}

#[derive(Debug, Clone, Copy)]
struct Gear {
    pos: (i32, i32),
}

fn main() {
    solve(include_str!("../../inputs/day3.txt"));
}

fn solve(input: &str) -> i32 {
    let (nums, gears) = parse(input).unwrap().1;

    let mut sum = 0;
    for g in gears {
        let mut adj = Vec::new();

        for d in &nums {
            if ((d.pos.0 - 1)..(d.pos.0 + d.width + 1)).contains(&g.pos.0) {
                if ((d.pos.1 - 1)..(d.pos.1 + 2)).contains(&g.pos.1) {
                    dbg!(g, d);
                    adj.push(d.num);
                }
            }
        }
        if adj.len() == 2 {
            sum += adj[0] * adj[1];
        }
    }
    dbg!(sum);
    sum
}

fn parse(input: &str) -> IResult<&str, (Vec<Num>, Vec<Gear>)> {
    let mut nums = Vec::new();
    let mut gears = Vec::new();
    for (height, line) in input.lines().enumerate() {
        let mut i: i32 = 0;
        loop {
            if i as usize >= line.len() {
                break;
            }

            if line.chars().nth(i as usize).unwrap() == '*' {
                gears.push(Gear {
                    pos: (i, height as i32),
                });
                i += 1;
                continue;
            }

            if !line.chars().nth(i as usize).unwrap().is_numeric()
                && line.chars().nth(i as usize).unwrap() != '.'
            {
                i += 1;
                continue;
            }

            let (_, num) = opt(nom::character::complete::i32)(&line[(i as usize)..line.len()])?;

            if let Some(res) = num {
                let width = res.to_string().len() as i32;
                nums.push(Num {
                    num: res,
                    width,
                    pos: (i, height as i32),
                });
                i += width as i32;
                continue;
            }

            i += 1;
        }
    }
    Ok(("", (nums, gears)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(solve(ti), 467835);
    }
}
