use std::{collections::HashMap, error::Error};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, multispace1, newline, space0, space1},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
// For number types
use nom::character::complete as cnom;

#[derive(Debug, Clone)]
struct Node {
    id: u16,
    to: Vec<u16>,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    n_lookup: HashMap<String, u16>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            n_lookup: HashMap::new(),
        }
    }

    fn insert(&mut self, name: String, to: Vec<String>) {
        let mut to_id = Vec::new();
        for t in to {
            if let Some(id) = self.n_lookup.get(&t) {
                to_id.push(*id)
            } else {
                let id = self.nodes.len() as u16;
                self.nodes.push(Node { id, to: Vec::new() });
                to_id.push(id);
                self.n_lookup.insert(t, id);
            }
        }

        if let Some(id) = self.n_lookup.get(&name) {
            // Update inserted value
            self.nodes[*id as usize] = Node { id: *id, to: to_id }
        } else {
            let id = self.nodes.len() as u16;
            self.nodes.push(Node { id, to: to_id });
            self.n_lookup.insert(name, id);
        }
    }

    fn get_name_from_id(&self, id: u16) -> Option<&str> {
        self.n_lookup
            .iter()
            .filter(|(_, v)| **v == id)
            .map(|(k, _)| k.as_str())
            .next()
    }
}

fn main() {
    dbg!(solve(include_str!("../../inputs/day8.txt")));
}

fn solve(input: &str) -> String {
    let mut graph = Graph::new();
    let (pattern, lines) = parse(input).unwrap().1;
    let first = lines[0].0.clone();

    for (key, val) in lines {
        graph.insert(key, vec![val.0, val.1])
    }
    let mut pattern = pattern.chars().into_iter().cycle();

    let mut current = graph.nodes[*graph.n_lookup.get("AAA").unwrap() as usize].clone();

    let mut i: usize = 0;
    let target = *graph.n_lookup.get("ZZZ").unwrap();
    loop {
        let mov = pattern.next().unwrap();
        match mov {
            'L' => current = graph.nodes[current.to[0] as usize].clone(),
            'R' => current = graph.nodes[current.to[1] as usize].clone(),
            a => panic!("Got {}", a),
        }

        i += 1;
        if current.id == target {
            break;
        }

        // if i % 2_usize.pow(12) == 0 {
        //     println!("{}", i);
        // }
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
