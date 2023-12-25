use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
// For number types
use nom::character::complete as cnom;
use petgraph::{adj::NodeIndex, graph::UnGraph, graphmap::UnGraphMap};
use rustworkx_core::{connectivity::stoer_wagner_min_cut, Result};

fn main() {
    dbg!(solve(include_str!("../../inputs/day25.txt")));
}

fn solve(input: &str) -> String {
    let edges = input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .flat_map(|(l, r)| r.split_whitespace().map(move |a| (l, a)));
    let g: UnGraphMap<_, ()> = edges.collect();

    let min_cut: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&g, |_| Ok(1));

    dbg!(&min_cut);

    let c_count = min_cut.unwrap().unwrap().1.len();
    ((g.node_count() - c_count) * c_count).to_string()
}

fn parse(input: &str) -> IResult<&str, ()> {
    todo!("Add parser");

    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        assert_eq!(solve(ti), "54".to_string());
    }
}
