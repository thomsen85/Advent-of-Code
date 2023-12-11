#![allow(dead_code)]

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

fn main() {
    solve(include_str!("../../inputs/day3.txt"));
}

fn solve(input: &str) -> i32 {
    let (nums, symbols) = parse(input).unwrap().1;

    let mut sum = 0;
    for d in nums {
        let mut breaked = false;
        for s in &symbols {
            if ((d.pos.0 - 1)..(d.pos.0 + d.width + 1)).contains(&s.pos.0) {
                if ((d.pos.1 - 1)..(d.pos.1 + 2)).contains(&s.pos.1) {
                    sum += d.num;
                    breaked = true;
                    break;
                }
            }
        }
        if !breaked {
            dbg!(d);
        }
    }
    dbg!(sum);
    sum
}

fn parse(input: &str) -> IResult<&str, (Vec<Num>, Vec<Symbol>)> {
    let mut nums = Vec::new();
    let mut symbols = Vec::new();
    for (height, line) in input.lines().enumerate() {
        let mut i: i32 = 0;
        loop {
            if i as usize >= line.len() {
                break;
            }
            if !line.chars().nth(i as usize).unwrap().is_numeric()
                && line.chars().nth(i as usize).unwrap() != '.'
            {
                symbols.push(Symbol {
                    cont: line.chars().nth(i as usize).unwrap(),
                    width: 1,
                    pos: (i, height as i32),
                });
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
    Ok(("", (nums, symbols)))
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
        assert_eq!(solve(ti), 4361);
    }

    #[test]
    fn tets2() {
        let ti = "#..
.1.
...";
        assert_eq!(solve(ti), 1);
    }
    #[test]
    fn tets25() {
        let ti = "..#
.1.
...";
        assert_eq!(solve(ti), 1);
    }
    #[test]
    fn tets21() {
        let ti = "...
.1.
..#";
        assert_eq!(solve(ti), 1);
    }
    #[test]
    fn tets22() {
        let ti = "#..
.1.
#..";
        assert_eq!(solve(ti), 1);
    }
    #[test]
    fn tets13() {
        let ti = "#..
#1.
...";
        assert_eq!(solve(ti), 1);
    }
    #[test]
    fn tets23() {
        let ti = "#..
.1.
.#.";
        assert_eq!(solve(ti), 1);
    }
}
