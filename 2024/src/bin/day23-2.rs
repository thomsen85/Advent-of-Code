use common::graphs::graph::NamedNodesWeightedGraph;
use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day23.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let mut g = NamedNodesWeightedGraph::<()>::new();
    for (from, to) in input.lines().map(|line| line.split_once("-").unwrap()) {
        g.add_edge_by_name(from.to_owned(), to.to_owned(), 1, true)
    }

    let mut sets = HashSet::new();

    for i in 0..g.edges.len() {
        s(&g, i, HashSet::new(), &mut sets)
    }

    sets.into_iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .iter()
        .map(|i| g.translation_map.bi_map.val_key.get(i).unwrap())
        .sorted()
        .join(",")
}

fn s(
    g: &NamedNodesWeightedGraph<()>,
    current: usize,
    must_be_connected: HashSet<usize>,
    sets: &mut HashSet<Vec<usize>>,
) {
    let k = must_be_connected.iter().copied().sorted().collect_vec();
    if sets.contains(&k) {
        return;
    }
    sets.insert(k);
    for n in &g.edges[current] {
        if must_be_connected.contains(&n.to) {
            continue;
        }

        if !must_be_connected
            .iter()
            .all(|&mn| g.edges[n.to].iter().any(|nn| mn == nn.to))
        {
            continue;
        }
        let mut c_must_be_connected = must_be_connected.clone();
        c_must_be_connected.insert(n.to);

        s(g, n.to, c_must_be_connected, sets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!(solve(ti), "7".to_string());
    }
}
