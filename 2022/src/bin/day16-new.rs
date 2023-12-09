use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, multispace1, newline, space0, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
// For number types
use nom::character::complete as cnom;
type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Node {
    id: u16,
    flow_rate: u32,
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

    fn insert(&mut self, name: String, flow_rate: u32, to: Vec<String>) {
        let mut to_id = Vec::new();
        for t in to {
            if let Some(id) = self.n_lookup.get(&t) {
                to_id.push(*id)
            } else {
                let id = self.nodes.len() as u16;
                self.nodes.push(Node {
                    id,
                    flow_rate: 0,
                    to: Vec::new(),
                });
                to_id.push(id);
                self.n_lookup.insert(t, id);
            }
        }

        if let Some(id) = self.n_lookup.get(&name) {
            // Update inserted value
            self.nodes[*id as usize] = Node {
                id: *id,
                flow_rate,
                to: to_id,
            }
        } else {
            let id = self.nodes.len() as u16;
            self.nodes.push(Node {
                id,
                flow_rate,
                to: to_id,
            });
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

fn main() -> Res<()> {
    dbg!(solve(include_str!("../../inputs/day16.txt")));
    Ok(())
}

fn solve(input: &str) -> String {
    let mut graph = Graph::new();

    for line in input.lines() {
        let ((name, flow_rate), to) = parse(line).unwrap().1;
        dbg!(&name, flow_rate, &to);
        graph.insert(name, flow_rate, to);
    }

    let aa_id = *graph.n_lookup.get("AA").unwrap();
    let (a, b) = do_action(
        20,
        &graph,
        aa_id,
        0,
        HashSet::new(),
        HashSet::new(),
        String::new(),
    );

    println!("History: {}", b);
    dbg!(a);
    a.to_string()
}

fn do_action(
    minute: u16,
    graph: &Graph,
    current: u16,
    mut score: u32,
    mut opened: HashSet<u16>,
    mut visited: HashSet<(u16, u16)>,
    mut history: String,
) -> (u32, String) {
    if minute == 0 {
        return (score, history);
    }
    // println!(
    //     "Visiting {}, minute: {}",
    //     graph.get_name_from_id(current).unwrap(),
    //     minute
    // );

    history.push_str(&format!("== Minute {} ==\n", 31 - minute));
    let c_score = get_score(graph, &opened);
    history.push_str(&format!("Score {} added, opened {:?}\n", c_score, opened));
    score += c_score;

    let current_n = &graph.nodes[current as usize];

    let mut max_score = u32::MIN;
    let mut best_history = String::new();

    if current_n.flow_rate != 0 && !opened.contains(&current) {
        // Open
        opened.insert(current);
        //println!("Opening {}", graph.get_name_from_id(current).unwrap());

        let (a, b) = do_action(
            minute - 1,
            graph,
            current,
            score,
            opened.clone(),
            visited.clone(),
            history.clone(),
        );
        if max_score < a {
            max_score = a;
            best_history = b;
        }
    }

    // Move
    for to in &current_n.to {
        if !visited.contains(&(current, *to)) {
            visited.insert((current, *to));

            let (a, b) = do_action(
                minute - 1,
                graph,
                *to,
                score,
                opened.clone(),
                visited.clone(),
                history.clone(),
            );
            if max_score < a {
                max_score = a;
                best_history = b;
            }
        }
    }

    (max_score, best_history)
}

fn get_score(graph: &Graph, opened: &HashSet<u16>) -> u32 {
    opened
        .iter()
        .map(|v| graph.nodes[*v as usize].flow_rate)
        .sum()
}

fn parse(input: &str) -> IResult<&str, ((String, u32), Vec<String>)> {
    let (input, v) = separated_pair(
        separated_pair(
            preceded(tag("Valve "), map(alpha1, &str::to_string)),
            tag(" has flow rate="),
            cnom::u32,
        ),
        tag("; "),
        preceded(
            alt((
                tag("tunnels lead to valves "),
                tag("tunnel leads to valve "),
            )),
            separated_list1(tag(", "), map(alpha1, &str::to_string)),
        ),
    )(input)?;

    Ok((input, v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(solve(ti), "1651".to_string());
    }
}
