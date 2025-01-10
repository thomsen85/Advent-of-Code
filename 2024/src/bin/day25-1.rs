use common::strings::string_to_char_grid;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
use std::time::Instant;
// For number types
use nom::character::complete as cnom;

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day25.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for p in input.split("\n\n") {
        let arr = string_to_char_grid(p);
        if arr.first().unwrap().iter().all(|a| *a == '#') {
            // is lock
            let mut layout = Vec::new();
            for col in 0..arr[0].len() {
                let mut i = 0;

                for row in 1..arr.len() {
                    if arr[row][col] == '.' {
                        layout.push(i);
                        break;
                    }
                    i += 1;
                }
            }

            locks.push(layout);
        } else {
            // is key
            let mut layout = Vec::new();
            for col in 0..arr[0].len() {
                let mut i = 0;

                for row in (0..arr.len() - 1).rev() {
                    if arr[row][col] == '.' {
                        layout.push(i);
                        break;
                    }
                    i += 1;
                }
            }

            keys.push(layout);
        }
    }

    dbg!(&keys);
    dbg!(&locks);

    let mut s = 0;
    for lock in &locks {
        for key in &keys {
            if lock.iter().zip(key).all(|(a, b)| a + b < 6) {
                s += 1
            }
        }
    }

    s.to_string()
}

fn parse(input: &str) -> IResult<&str, ()> {
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!(solve(ti), "3".to_string());
    }
}
