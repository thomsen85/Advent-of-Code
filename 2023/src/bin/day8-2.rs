use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

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
}

fn main() {
    dbg!(solve(include_str!("../../inputs/day8.txt")));
}

fn solve(input: &str) -> String {
    let mut graph = Graph::new();
    dbg!("started");
    let (pattern, lines) = parse(input).unwrap().1;
    dbg!("Parsed");

    for (key, val) in lines {
        graph.insert(key, vec![val.0, val.1])
    }
    let mut pattern = pattern.chars().into_iter().cycle();

    let starts = graph
        .n_lookup
        .iter()
        .filter(|(n, _)| n.ends_with('A'))
        .map(|(_, n)| *n)
        .collect_vec();

    let mut current = starts
        .iter()
        .map(|id| graph.nodes[*id as usize].clone())
        .collect_vec();

    let mut i: usize = 0;
    let targets = graph
        .n_lookup
        .iter()
        .filter(|(n, _)| n.ends_with('Z'))
        .map(|(_, n)| *n)
        .collect::<HashSet<_>>();

    dbg!(&current, &targets);

    let mut hits: HashMap<u16, Vec<usize>> = HashMap::new();
    let mut hit_spacing: HashMap<u16, usize> = HashMap::new();

    loop {
        let mov = pattern.next().unwrap();
        match mov {
            'L' => {
                current = current
                    .iter()
                    .map(|n| graph.nodes[n.to[0] as usize].clone())
                    .collect_vec()
            }
            'R' => {
                current = current
                    .iter()
                    .map(|n| graph.nodes[n.to[1] as usize].clone())
                    .collect_vec();
            }
            a => panic!("Got {}", a),
        }
        // dbg!(current
        //     .iter()
        //     .map(|id| graph.get_name_from_id(id.id))
        //     .collect_vec());

        i += 1;

        for c in current.iter().filter(|a| targets.contains(&a.id)) {
            if let Some(a) = hits.get_mut(&c.id) {
                if a.len() == 2 {
                    hit_spacing.insert(c.id, a[1] - a[0]);
                };
                a.push(i);
            } else {
                hits.insert(c.id, Vec::new());
            }
        }

        if current.iter().all(|n| targets.contains(&n.id)) || hit_spacing.len() == current.len() {
            break;
        }

        // if i % 2_usize.pow(12) == 0 {
        //     println!("{}", i);
        // }
    }

    dbg!(&hit_spacing);

    if hit_spacing.len() == current.len() {
        let mut i = 1;
        for s in hit_spacing {
            i = ::num::integer::lcm(i, s.1);
        }
    }
    dbg!(i);
    i.to_string()
}

fn parse(input: &str) -> IResult<&str, (String, Vec<(String, (String, String))>)> {
    let (input, p) = separated_pair(
        map(alpha1, |a: &str| a.to_owned()),
        multispace1,
        separated_list1(
            newline,
            separated_pair(
                map(alphanumeric1, |a: &str| a.to_owned()),
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(
                        map(alphanumeric1, |a: &str| a.to_owned()),
                        tag(", "),
                        map(alphanumeric1, |a: &str| a.to_owned()),
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
        let ti = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(solve(ti), "6".to_string());
    }
}
