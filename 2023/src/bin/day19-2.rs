use std::{collections::HashMap, ops::Range, usize};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::take,
    character::complete::{alpha1, anychar, multispace1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};
// For number types
use nom::character::complete as cnom;

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Clone)]
struct SuperPart {
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>,
}

impl SuperPart {
    fn new(x: Range<u32>, m: Range<u32>, a: Range<u32>, s: Range<u32>) -> Self {
        SuperPart { x, m, a, s }
    }
}

#[derive(Debug)]
enum Rule<'a> {
    Condition(char, char, u32, &'a str),
    End(&'a str),
}

fn main() {
    dbg!(solve(include_str!("../../inputs/day19.txt")));
}

fn solve(input: &str) -> String {
    let (workflows, _parts) = parse(input).unwrap().1;
    let locs = workflows.iter().map(|a| a.0).collect_vec();
    let workflows: HashMap<&str, Vec<Rule>> = HashMap::from_iter(workflows.into_iter());
    //dbg!(&workflows);

    let mut parts = HashMap::new();
    locs.iter().for_each(|loc| {
        parts.insert(*loc, vec![]);
    });
    parts.insert("A", vec![]);
    parts.insert("R", vec![]);
    parts.insert(
        "in",
        vec![SuperPart {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }],
    );

    loop {
        let mut new_parts = HashMap::new();
        locs.iter().for_each(|loc| {
            new_parts.insert(*loc, vec![]);
        });
        new_parts.insert("A", parts.get("A").unwrap().clone());
        new_parts.insert("R", parts.get("R").unwrap().clone());
        for (w, parts_in) in parts {
            if w == "A" || w == "R" {
                continue;
            }
            let wf = workflows.get(w).unwrap();
            for original_part in parts_in {
                let mut part = original_part.clone();

                for rule in wf {
                    match rule {
                        Rule::Condition(arg, op, amnt, new_loc) => {
                            let p_val = match arg {
                                'x' => part.x.clone(),
                                'm' => part.m.clone(),
                                'a' => part.a.clone(),
                                's' => part.s.clone(),
                                c => panic!("Not expectecd {}", c),
                            };

                            let left = if p_val.start < *amnt {
                                let add = if *op == '>' { 1 } else { 0 };
                                let split = (p_val.start)..(*amnt + add).min(p_val.end);

                                Some(match arg {
                                    'x' => SuperPart::new(
                                        split,
                                        part.m.clone(),
                                        part.a.clone(),
                                        part.s.clone(),
                                    ),
                                    'm' => SuperPart::new(
                                        part.x.clone(),
                                        split,
                                        part.a.clone(),
                                        part.s.clone(),
                                    ),
                                    'a' => SuperPart::new(
                                        part.x.clone(),
                                        part.m.clone(),
                                        split,
                                        part.s.clone(),
                                    ),
                                    's' => SuperPart::new(
                                        part.x.clone(),
                                        part.m.clone(),
                                        part.a.clone(),
                                        split,
                                    ),
                                    c => panic!("Not expectecd {}", c),
                                })
                            } else {
                                None
                            };

                            let right = if p_val.end > *amnt {
                                let add = if *op == '>' { 1 } else { 0 };
                                let split = (*amnt + add).max(p_val.start)..p_val.end;

                                Some(match arg {
                                    'x' => SuperPart::new(split, part.m, part.a, part.s),
                                    'm' => SuperPart::new(part.x, split, part.a, part.s),
                                    'a' => SuperPart::new(part.x, part.m, split, part.s),
                                    's' => SuperPart::new(part.x, part.m, part.a, split),
                                    c => panic!("Not expectecd {}", c),
                                })
                            } else {
                                None
                            };

                            match op {
                                '<' => {
                                    // Left go to another place
                                    if let Some(l) = left {
                                        if let Some(loc_list) = new_parts.get_mut(new_loc) {
                                            loc_list.push(l);
                                        } else {
                                            panic!();
                                        }
                                    };

                                    if let Some(r) = right {
                                        part = r;
                                    } else {
                                        break;
                                    };
                                }
                                '>' => {
                                    // Left go to another place
                                    if let Some(r) = right {
                                        if let Some(loc_list) = new_parts.get_mut(new_loc) {
                                            loc_list.push(r);
                                        } else {
                                            panic!();
                                        }
                                    };

                                    if let Some(l) = left {
                                        part = l;
                                    } else {
                                        break;
                                    };
                                }
                                x => panic!("Not expected {}", x),
                            }
                        }
                        Rule::End(new_loc) => {
                            if let Some(loc_list) = new_parts.get_mut(new_loc) {
                                loc_list.push(part.clone());
                            } else {
                                panic!();
                            }
                        }
                    }
                }
            }
        }

        parts = new_parts;
        // Either (A or R) or empty_loc_list
        if parts
            .iter()
            .all(|(loc, loc_list)| (*loc == "A" || *loc == "R") || loc_list.is_empty())
        {
            break;
        }
    }

    dbg!(&parts);
    let a = parts
        .get("A")
        .unwrap()
        .into_iter()
        .map(|p| {
            (p.x.end - p.x.start) as usize
                * (p.m.end - p.m.start) as usize
                * (p.a.end - p.a.start) as usize
                * (p.s.end - p.s.start) as usize
        })
        .sum::<usize>();

    let r = parts
        .get("R")
        .unwrap()
        .into_iter()
        .map(|p| {
            (p.x.end - p.x.start) as usize
                * (p.m.end - p.m.start) as usize
                * (p.a.end - p.a.start) as usize
                * (p.s.end - p.s.start) as usize
        })
        .sum::<usize>();

    assert!(r + a == 4000_usize.pow(4));

    parts
        .get("A")
        .unwrap()
        .into_iter()
        .map(|p| {
            (p.x.end - p.x.start) as usize
                * (p.m.end - p.m.start) as usize
                * (p.a.end - p.a.start) as usize
                * (p.s.end - p.s.start) as usize
        })
        .sum::<usize>()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, (Vec<(&str, Vec<Rule>)>, Vec<Part>)> {
    let (input, (workflows, parts)) = separated_pair(
        separated_list1(
            newline,
            pair(
                alpha1,
                delimited(
                    cnom::char('{'),
                    separated_list1(
                        cnom::char(','),
                        alt((
                            map(
                                separated_pair(
                                    tuple((
                                        anychar,
                                        alt((cnom::char('<'), cnom::char('>'))),
                                        cnom::u32,
                                    )),
                                    cnom::char(':'),
                                    alpha1,
                                ),
                                |line| Rule::Condition(line.0 .0, line.0 .1, line.0 .2, line.1),
                            ),
                            map(alpha1, |a| Rule::End(a)),
                        )),
                    ),
                    cnom::char('}'),
                ),
            ),
        ),
        multispace1,
        separated_list1(
            newline,
            delimited(
                cnom::char('{'),
                map(
                    separated_list1(cnom::char(','), preceded(take(2_usize), cnom::u32)),
                    |l| Part {
                        x: l[0],
                        m: l[1],
                        a: l[2],
                        s: l[3],
                    },
                ),
                cnom::char('}'),
            ),
        ),
    )(input)?;

    Ok((input, (workflows, parts)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(solve(ti), "167409079868000".to_string());
    }

    #[test]
    fn test_2() {
        let ti = "in{x<11:a,R}
a{m<11:b,R}
b{a<11:c,R}
c{s>3990:A,R}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(solve(ti), "10000".to_string());
    }
}
