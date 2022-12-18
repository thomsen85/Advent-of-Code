use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
struct EdgeTo {
    to: u32,
    drive_time: u32,
}

impl EdgeTo {
    fn new(to: u32, drive_time: u32) -> Self {
        Self { to, drive_time }
    }
}

#[derive(Clone)]
struct Node {
    id: u32,
    latitude: f64,
    longitude: f64,
}

impl Node {
    fn new(id: u32, latitude: f64, longitude: f64) -> Self {
        Self {
            id,
            latitude,
            longitude,
        }
    }
}

#[derive(Clone)]
struct Map {
    nodes: Vec<Node>,
    edges: Vec<Vec<EdgeTo>>,
}

impl Map {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn from_nodes_edges_and_poi(nodes: Vec<Node>, edges: Vec<Vec<EdgeTo>>) -> Self {
        Self { nodes, edges }
    }

    fn get_coordinates_from_node(&self, node: usize) -> (f64, f64) {
        let n = &self.nodes[node];
        (n.latitude, n.longitude)
    }

    fn get_reverse_copy(&self) -> Self {
        let mut edges = vec![Vec::new(); self.edges.len()];
        for i in 0..self.edges.len() {
            for edge in &self.edges[i] {
                edges[edge.to as usize].push(EdgeTo::new(i as u32, edge.drive_time));
            }
        }
        Map::from_nodes_edges_and_poi(self.nodes.clone(), edges)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Priority<'a, T> {
    number: usize,
    edges: &'a Vec<EdgeTo>,
    cost: T,
}

impl<'a, T> Priority<'a, T> {
    fn new(number: usize, cost: T, edges: &'a Vec<EdgeTo>) -> Self {
        Self {
            number,
            edges,
            cost,
        }
    }
}

impl<T: Ord> Ord for Priority<'_, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl<T: Ord> PartialOrd for Priority<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn rc_to_id(row: usize, col: usize, cols: usize) -> u32 {
    (row * cols + col) as u32
}

/// Returns: (shotests distances, previous)
fn full_dijkstra(map: &Map, source: u32) -> (Vec<u32>, Vec<Option<usize>>) {
    // Init variables
    let length = map.edges.len();
    let mut shortest_distances = vec![u32::MAX / 2; length];
    let mut previous: Vec<Option<usize>> = vec![None; length];
    let mut priority_queue: BinaryHeap<Priority<u32>> = BinaryHeap::new();
    shortest_distances[source as usize] = 0;

    // Push source variable
    priority_queue.push(Priority::new(
        source as usize,
        0,
        &map.edges[source as usize],
    ));

    while let Some(priority) = priority_queue.pop() {
        for neighbour in priority.edges {
            let alt = shortest_distances[priority.number] + neighbour.drive_time;
            if alt < shortest_distances[neighbour.to as usize] {
                shortest_distances[neighbour.to as usize] = alt;
                previous[neighbour.to as usize] = Some(priority.number);
                priority_queue.push(Priority::new(
                    neighbour.to as usize,
                    alt,
                    &map.edges[neighbour.to as usize],
                ));
            }
        }
    }
    (shortest_distances, previous)
}

fn main() {
    let input: Vec<Vec<u8>> = aoc2022_rust::utils::lines_from_file("inputs/day12.txt")
        .into_iter()
        .map(|s| s.as_bytes().to_owned())
        .collect();

    let rows = input.len();
    let cols = input[0].len();

    let mut nodes = Vec::new();
    let mut edges = vec![Vec::new(); rows * cols];
    let mut all_a = HashSet::new();

    for row in 0..rows {
        for col in 0..cols {
            // Nodes
            let id = rc_to_id(row, col, cols);
            let char = input[row][col] as i16;
            if char == 97 {
                all_a.insert(id);
            }
            let node = Node::new(id as u32, row as f64, col as f64);
            nodes.push(node);

            // Edges
            // Up
            if row > 0 && (char - input[row - 1][col] as i16) >= -1 {
                edges[id as usize].push(EdgeTo::new(id - cols as u32, 1))
            }
            // Down
            if row < rows - 1 && (char - input[row + 1][col] as i16) >= -1 {
                edges[id as usize].push(EdgeTo::new(id + cols as u32, 1))
            }
            // Left
            if col > 0 && (char - input[row][col - 1] as i16) >= -1 {
                edges[id as usize].push(EdgeTo::new(id - 1 as u32, 1));
            }
            if col < cols - 1 && (char - input[row][col + 1] as i16) >= -1 {
                edges[id as usize].push(EdgeTo::new(id + 1 as u32, 1))
            }
        }
    }
    let map = Map::from_nodes_edges_and_poi(nodes, edges);
    // 21
    // 3485
    let result = full_dijkstra(&map.get_reverse_copy(), 3485);
    //dbg!(&map.nodes.len(), &map.edges.len());
    println!(
        "Shotest distance: {:?}",
        &result
            .0
            .into_iter()
            .enumerate()
            .filter(|(i, _)| all_a.contains(&(*i as u32)))
            .map(|(i, x)| ((i / cols, i % cols), x))
            .sorted_by(|a, b| a.1.cmp(&b.1))
            .nth(0)
            .unwrap()
    );
}
