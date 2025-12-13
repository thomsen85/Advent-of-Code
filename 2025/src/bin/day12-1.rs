use std::{collections::HashMap, error::Error, time::Instant};

use itertools::Itertools;

fn main() {
    let now = Instant::now();
    let res = solve(include_str!("../../inputs/day12.txt"));
    let elapsed = now.elapsed();
    dbg!(res);
    println!("Time used {elapsed:?}");
}

#[derive(Debug, Clone, Copy)]
enum Square {
    Empty,
    Full,
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    form: [[Square; 3]; 3],
    area: usize,
}

impl TryFrom<char> for Square {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Square::Full),
            '.' => Ok(Square::Empty),
            _ => Err(format!("char needs to be either # or ., got {:?}", value)),
        }
    }
}

impl TryFrom<String> for Piece {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let form: [[Square; 3]; 3] = value
            .split("\n")
            .map(|line| {
                line.chars()
                    .map(|c| Square::try_from(c).unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .collect_vec()
            .try_into()
            .unwrap();

        let mut area = 0;
        for line in form {
            for square in line {
                match square {
                    Square::Full => area += 1,
                    _ => (),
                };
            }
        }

        Ok(Piece { form, area })
    }
}

fn solve(input: &str) -> String {
    let pieces: Vec<Piece> = input
        .split("\n\n")
        .take(6)
        .map(|a| a.split("\n").skip(1).join("\n").try_into().unwrap())
        .collect_vec();
    dbg!(&pieces);

    let lines = input
        .trim()
        .split("\n\n")
        .last()
        .unwrap()
        .split("\n")
        .map(|line| line.split_once(": ").unwrap())
        .collect_vec();

    let mut invalid = 0;
    let mut valid = 0;
    let total = lines.len();
    for (size, packages) in lines {
        let (width, height) = size.split_once("x").unwrap();
        let width: usize = width.parse().unwrap();
        let height: usize = height.parse().unwrap();

        let requirements = packages
            .split(" ")
            .map(|c| c.parse::<usize>().unwrap())
            .collect_vec();

        let total_area = width * height;
        let minimum_package_area = requirements
            .iter()
            .enumerate()
            .map(|(i, amount)| &pieces[i].area * amount)
            .sum();

        let number_of_packages = requirements.iter().map(|amount| amount).sum();

        if total_area <= minimum_package_area {
            invalid += 1;
            continue;
        }
        // Maybe fits
        if (width / 3) * (height / 3) >= number_of_packages {
            valid += 1;
            continue;
        }
        // Oh, this is enoguh...
    }

    dbg!(total);
    dbg!(invalid);
    dbg!(valid);

    valid.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        assert_eq!(solve(ti), "2".to_string());
    }
}
