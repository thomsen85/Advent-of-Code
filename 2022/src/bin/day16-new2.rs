use std::{
    collections::{HashSet, VecDeque},
    thread,
    time::Duration,
};

use common::graphs::graph::NamedNodesWeightedGraph;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, multispace1, newline, space0, space1},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};
// For number types
use nom::character::complete as cnom;

fn main() {
    dbg!(solve(include_str!("../../inputs/day16.txt")));
}

fn solve(input: &str) -> String {
    let start = "AA";

    let p = parse(input).unwrap().1;
    let null_nodes = p
        .iter()
        .filter(|a| a.1 == 0 && a.0 != start)
        .map(|a| a.0)
        .collect::<Vec<_>>();

    let mut graph = NamedNodesWeightedGraph::new();

    for (name, flow_rate, connections) in p {
        graph.add_or_override_node(name.to_owned(), flow_rate);
        for connection in connections {
            graph.add_edge_by_name(name.to_owned(), connection.to_owned(), 1, false);
        }
    }

    for null_node in null_nodes {
        graph.truncate_node(&null_node.to_owned())
    }

    //dbg!(&graph);

    // (Minute, current, opened, score)
    let mut queue = VecDeque::new();
    let start_i = graph.translation_map.get(&start.to_owned()).unwrap();

    queue.push_back((0, start_i, HashSet::from([start_i]), 0));

    let mut best_score = 0;

    let mut cache = HashSet::new();

    while let Some((minute, current, opened, score)) = queue.pop_front() {
        if cache.contains(&(
            minute,
            current,
            opened.clone().into_iter().collect_vec(),
            score,
        )) {
            continue;
        }
        cache.insert((
            minute,
            current,
            opened.clone().into_iter().collect_vec(),
            score,
        ));

        let score = score
            + opened
                .iter()
                .map(|a| graph.nodes[*a].unwrap_or(0))
                .sum::<i32>();

        if minute >= 30 {
            best_score = best_score.max(score);
            continue;
        }

        if !opened.contains(&current) {
            let mut opened_clone = opened.clone();
            opened_clone.insert(current);
            queue.push_back((minute + 1, current, opened_clone, score));
        }

        for edge in &graph.edges[current] {
            let score = score
                + opened
                    .iter()
                    .map(|a| graph.nodes[*a].unwrap_or(0))
                    .sum::<i32>()
                    * (edge.weight - 1).min(30 - minute);
            queue.push_back((minute + edge.weight, edge.to, opened.clone(), score))
        }
    }

    best_score.to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<(&str, i32, Vec<&str>)>> {
    separated_list1(
        newline,
        tuple((
            preceded(tag("Valve "), alpha1),
            preceded(tag(" has flow rate="), cnom::i32),
            preceded(
                alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                separated_list0(tag(", "), alpha1),
            ),
        )),
    )(input)
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
