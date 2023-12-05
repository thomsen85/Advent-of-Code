#![allow(dead_code)]

use std::usize;

use nom::{
    bytes::{
        complete::{is_not, tag},
        streaming::take_until,
    },
    character::complete::{alpha1, multispace1, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    dbg!(solve(include_str!("../../inputs/day5.txt")));
}

fn solve(input: &str) -> usize {
    let (_, (mut seeds, maps)) = parse(input).unwrap();

    let mut nseeds = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        nseeds.extend(seeds[i]..(seeds[i] + seeds[i + 1]));
    }

    for (i, mut s) in nseeds.iter_mut().enumerate() {
        for (_name, map) in &maps {
            for line in map {
                if (line[1]..(line[1] + line[2])).contains(&s) {
                    *s += line[0];
                    *s -= line[1];
                    break;
                }
            }
        }
    }

    nseeds.iter().min().unwrap().to_owned() as usize
}

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<(&str, Vec<Vec<u64>>)>)> {
    let (input, seeds) = preceded(
        tag("seeds: "),
        separated_list1(multispace1, nom::character::complete::u64),
    )(input)?;

    let (input, maps) = preceded(
        multispace1,
        separated_list1(
            tag("\n\n"),
            separated_pair(
                is_not(" "),
                tag(" map:\n"),
                separated_list1(
                    newline,
                    separated_list1(space1, nom::character::complete::u64),
                ),
            ),
        ),
    )(input)?;

    Ok((input, (seeds, maps)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(solve(ti), 46);
    }
}
