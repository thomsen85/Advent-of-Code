use std::{
    collections::{HashMap, HashSet},
    usize,
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, multispace1, newline, space0, space1},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};
// For number types
use nom::character::complete as cnom;

#[derive(Debug)]
struct WeightedNamedGraph {
    translation: HashMap<String, usize>,
    r_translation: Vec<String>,
    edge_matrix: Vec<Vec<usize>>,
}

impl WeightedNamedGraph {
    fn new() -> Self {
        Self {
            translation: HashMap::new(),
            r_translation: Vec::new(),
            edge_matrix: Vec::new(),
        }
    }

    fn insert_undirected_edge(&mut self, from: &str, to: &str, weight: usize) {
        let from_i = {
            if let Some(i) = self.translation.get(from) {
                i.to_owned()
            } else {
                self.insert_node(from)
            }
        };
        let to_i = {
            if let Some(i) = self.translation.get(to) {
                i.to_owned()
            } else {
                self.insert_node(to)
            }
        };

        self.edge_matrix[from_i][to_i] = weight;
        self.edge_matrix[to_i][from_i] = weight;
    }

    fn insert_node(&mut self, name: &str) -> usize {
        let i = self.r_translation.len();
        self.translation.insert(name.to_owned(), i);
        self.r_translation.push(name.to_owned());

        // Expantion of edge_matrix is not optimal
        for ei in 0..self.edge_matrix.len() {
            self.edge_matrix[ei].push(usize::MAX);
        }
        self.edge_matrix.push(vec![usize::MAX; i + 1]);
        i
    }
}

fn main() {
    // Took 54 minutes and 3 seconds.
    dbg!(solve(include_str!("../../inputs/day9.txt")));
}

fn solve(input: &str) -> String {
    let mut g = WeightedNamedGraph::new();
    for l in parse(input).unwrap().1 {
        g.insert_undirected_edge(l.0, l.1, l.2 as usize)
    }

    dbg!(&g);

    let mut min = usize::MAX;
    for (i, n) in g.r_translation.iter().enumerate() {
        min = min.min(find_shortest(i, &g, vec![false; g.r_translation.len()], 0));
    }

    min.to_string()
}

fn find_shortest(
    current: usize,
    g: &WeightedNamedGraph,
    mut visited: Vec<bool>,
    sum: usize,
) -> usize {
    visited[current] = true;

    let unvisited = g.edge_matrix[current]
        .iter()
        .enumerate()
        .filter(|(i, w)| **w != usize::MAX && !visited[*i])
        .collect_vec();

    if unvisited.is_empty() {
        if !visited.iter().all(|a| *a) {
            return usize::MAX;
        } else {
            return sum;
        }
    }

    let mut min = usize::MAX;

    for (node_i, node) in unvisited {
        // Seeing now that adjecency list would be better
        min = min.min(find_shortest(
            node_i,
            g,
            visited.clone(),
            sum + g.edge_matrix[current][node_i],
        ));
    }
    min
}

// Tristram to AlphaCentauri = 34
fn parse(input: &str) -> IResult<&str, Vec<(&str, &str, u32)>> {
    separated_list1(
        newline,
        tuple((
            alpha1,
            preceded(tag(" to "), alpha1),
            preceded(tag(" = "), cnom::u32),
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        assert_eq!(solve(ti), "605".to_string());
    }
}
