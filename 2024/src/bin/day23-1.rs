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

    dbg!(&g);

    let mut marked = vec![false; g.edges.len()];

    let mut count = 0;
    let mut ccs = Vec::new();

    dbg!(g.edges.len());
    for i in 0..g.edges.len() {
        if g.translation_map
            .bi_map
            .val_key
            .get(&i)
            .unwrap()
            .starts_with("t")
        {
            dfs(&g, &mut marked, 2, i, i, &mut count, Vec::new(), &mut ccs)
        }
    }

    dbg!(ccs
        .iter()
        .map(|a| a
            .iter()
            .map(|i| g.translation_map.bi_map.val_key.get(i).unwrap())
            .sorted()
            .collect_vec())
        .collect::<HashSet<_>>())
    .len()
    .to_string()
}

fn dfs(
    g: &NamedNodesWeightedGraph<()>,
    marked: &mut Vec<bool>,
    n: usize,
    vert: usize,
    start: usize,
    count: &mut usize,
    mut hist: Vec<usize>,
    ccs: &mut Vec<Vec<usize>>,
) {
    marked[vert] = true;

    hist.push(vert);

    if n == 0 {
        marked[vert] = false;

        if g.is_edge_between(vert, start) && g.is_edge_between(start, vert) {
            *count += 1;
            ccs.push(hist);
        }
        return;
    }

    for i in &g.edges[vert] {
        if !marked[i.to] {
            dfs(g, marked, n - 1, i.to, start, count, hist.clone(), ccs);
        }
    }
    marked[vert] = false;
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
