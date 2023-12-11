use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone)]
struct Node {
    name: String,
    to: (String, String),
}

fn main() {
    dbg!(solve(include_str!("../../inputs/day8.txt")));
}

fn solve(input: &str) -> String {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let (pattern, lines) = parse(input).unwrap().1;
    let mut current = Node {
        name: lines[0].clone().0,
        to: lines[0].clone().1,
    };

    for (key, val) in lines {
        let n = Node {
            name: key.clone(),
            to: val,
        };
        nodes.insert(key, n);
    }
    let mut pattern = pattern.chars().into_iter().cycle();

    let mut i = 0;
    loop {
        let mov = pattern.next().unwrap();
        match mov {
            'L' => current = nodes.get(&current.to.0).unwrap().clone(),
            'R' => current = nodes.get(&current.to.1).unwrap().clone(),
            a => panic!("Got {}", a),
        }

        i += 1;
        if current.name == "ZZZ" {
            break;
        }
    }

    i.to_string()
}

fn parse(input: &str) -> IResult<&str, (String, Vec<(String, (String, String))>)> {
    let (input, p) = separated_pair(
        map(alpha1, |a: &str| a.to_owned()),
        multispace1,
        separated_list1(
            newline,
            separated_pair(
                map(alpha1, |a: &str| a.to_owned()),
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(
                        map(alpha1, |a: &str| a.to_owned()),
                        tag(", "),
                        map(alpha1, |a: &str| a.to_owned()),
                    ),
                    tag(")"),
                ),
            ),
        ),
    )(input)?;

    Ok((input, p))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve(ti), "2".to_string());
    }

    #[test]
    fn test_2() {
        let ti = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve(ti), "6".to_string());
    }
}
