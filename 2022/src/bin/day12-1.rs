use std::collections::BinaryHeap;

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

/// Returns (distance, path, visited)
fn closest_dijkstra(map: &Map, from: usize, to: usize) -> (usize, Vec<u32>, Vec<u32>) {
    // Init variables
    let length = map.edges.len();
    let mut shortest_distances = vec![usize::MAX / 2; length];
    let mut previous: Vec<Option<usize>> = vec![None; length];
    let mut priority_queue: BinaryHeap<Priority<usize>> = BinaryHeap::new();
    shortest_distances[from] = 0;

    // Push source variable
    priority_queue.push(Priority::new(from, 0, &map.edges[from]));
    let mut visited = Vec::new();
    while let Some(priority) = priority_queue.pop() {
        visited.push(priority.number as u32);
        if priority.number == to {
            break;
        }
        for neighbour in priority.edges {
            let alt = shortest_distances[priority.number] + neighbour.drive_time as usize;
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
    //dbg!(&shortest_distances);
    let mut path = Vec::new();
    let mut prev = to;
    loop {
        path.push(prev as u32);
        if let Some(p) = previous[prev] {
            prev = p;
        } else {
            panic!("Something went wrong with finding path");
        }
        if prev == from {
            break;
        }
    }
    path.reverse();

    (shortest_distances[to], path, visited)
}

fn rc_to_id(row: usize, col: usize, cols: usize) -> u32 {
    (row * cols + col) as u32
}

fn main() {
    let input: Vec<Vec<u8>> = common::utils::lines_from_file("inputs/day12.txt")
        .into_iter()
        .map(|s| s.as_bytes().to_owned())
        .collect();

    let rows = input.len();
    let cols = input[0].len();

    let mut nodes = Vec::new();
    let mut edges = vec![Vec::new(); rows * cols];

    for row in 0..rows {
        for col in 0..cols {
            // Nodes
            let id = rc_to_id(row, col, cols);
            let char = input[row][col] as i16;
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
    let result = closest_dijkstra(&map, 20 * cols, 3485);
    //dbg!(&map.nodes.len(), &map.edges.len());

    println!("Shotest distance: {:?}", &result.0);
}
