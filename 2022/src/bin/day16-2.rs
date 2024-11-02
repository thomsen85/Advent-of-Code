use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use cgmath::num_traits::Pow;
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
    let timer = Instant::now();
    dbg!(solve(include_str!("../../inputs/day16.txt")));
    let elapsed = timer.elapsed();
    println!("Took: {}", elapsed.as_secs_f64());
}

fn solve(input: &str) -> String {
    let start = "AA";

    let p = parse_input(input).unwrap().1;

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

    // (Minute, current, current2, opened, score)
    let mut queue = VecDeque::new();
    let start_i = graph.translation_map.get(&start.to_owned()).unwrap();

    queue.push_back((1, start_i, start_i, HashSet::from([start_i]), 0));

    let mut best_score = 0;
    // let mut best_story = None;

    let mut cache = HashSet::new();

    while let Some((minute, current, current2, opened, score)) = queue.pop_front() {
        let state = (
            minute,
            current,
            opened.iter().map(|a| 2_usize.pow(*a as u32)).sum::<usize>(),
        );
        if cache.contains(&state) {
            continue;
        }
        cache.insert(state);

        if minute >= 26 {
            if score > best_score {
                best_score = score;
                // best_story = Some(story);
            }
            continue;
        }
        let pressure_per_minute = opened
            .iter()
            .map(|a| graph.nodes[*a].unwrap_or(0))
            .sum::<i32>();

        let score = score + pressure_per_minute;
        //         story.push_str(&format!(
        //             "\n== Minute {} ==
        // Valve {:?}, releasing {} pressure. ({})\n",
        //             minute, opened, pressure_per_minute, score
        //         ));

        if !opened.contains(&current) {
            let mut opened_clone = opened.clone();
            opened_clone.insert(current);
            // story.push_str(&format!("You open valve {}", current));
            queue.push_back((minute + 1, current, opened_clone, score));
        }

        for edge in &graph.edges[current] {
            let score = score + pressure_per_minute * (edge.weight - 1).min(30 - minute);

            // let mut story_c = story.clone();
            // story_c.push_str(&format!("You move to valve {}", edge.to));
            queue.push_back((
                minute + edge.weight,
                edge.to,
                opened.clone(),
                score,
                // story_c,
            ))
        }
    }

    // println!("{}", best_story.unwrap());

    best_score.to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, i32, Vec<&str>)>> {
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
