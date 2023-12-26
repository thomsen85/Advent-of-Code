use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, multispace1, newline, space0, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};
// For number types
use nom::character::complete as cnom;

#[derive(Debug, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum Module {
    Broadcast,
    FlipFlop(bool),
    Conjuction(HashMap<String, Option<Pulse>>),
}

impl Module {
    fn tick(&mut self, pulse: Pulse, from: String) -> Option<Pulse> {
        use Module::*;
        use Pulse::*;
        match self {
            Broadcast => Some(pulse),
            FlipFlop(mut b) => match pulse {
                Low => {
                    if b {
                        b = false;
                        Some(Low)
                    } else {
                        b = true;
                        Some(High)
                    }
                }
                High => None,
            },
            Conjuction(v) => {
                v.insert(from, Some(pulse));
                if v.iter().all(|(_k, p)| matches!(p.unwrap_or(Low), High)) {
                    Some(Low)
                } else {
                    Some(High)
                }
            }
        }
    }

    fn reset(&mut self) {
        use Module::*;
        match self {
            Broadcast => todo!(),
            FlipFlop(_) => todo!(),
            Conjuction(_) => todo!(),
        }
    }
}

fn main() {
    dbg!(solve(include_str!("../../inputs/day20.txt")));
}

fn solve(input: &str) -> String {
    use Module::*;
    let mut modules = parse(input).unwrap().1;

    // Preprosses Conjuctions

    let conjunctions = modules
        .iter()
        .filter(|(_, (m, _))| matches!(m, Conjuction(_)))
        .map(|(a, _)| a.to_owned())
        .collect_vec();

    for con_name in conjunctions {
        let connections = modules
            .iter()
            .filter(|(_, (_, m_cons))| m_cons.contains(&con_name))
            .map(|(n, _)| (n.to_owned(), None))
            .collect_vec();

        match modules.get_mut(&con_name).unwrap() {
            (Conjuction(s), _) => s.extend(connections),
            _ => panic!(),
        };
    }

    dbg!(&modules);

    let mut high = 0;
    let mut low = 0;

    const LOOP_TIMES: usize = 1;
    for _i in 0..LOOP_TIMES {
        use Pulse::*;

        let mut queue =
            VecDeque::from(vec![("button".to_string(), "broadcaster".to_string(), Low)]);

        while let Some((from, to, pulse)) = queue.pop_front() {
            let (to_mod, tos) = modules.get_mut(&to).unwrap();

            if let Some(res) = to_mod.tick(pulse, from) {
                match res {
                    Low => low += 1,
                    High => high += 1,
                }
                for to_other in tos {
                    queue.push_back((to.clone(), to_other.clone(), res))
                }
            }
        }
    }

    (low * high).to_string()
}

fn parse(input: &str) -> IResult<&str, HashMap<String, (Module, Vec<String>)>> {
    use Module::*;
    let (input, ans) = separated_list1(
        newline,
        separated_pair(
            alt((
                map(tag("broadcaster"), |name| (name, Broadcast)),
                map(preceded(cnom::char('%'), alpha1), |name| {
                    (name, FlipFlop(false))
                }),
                map(preceded(cnom::char('&'), alpha1), |name| {
                    (name, Conjuction(HashMap::new()))
                }),
            )),
            tag(" -> "),
            separated_list1(tag(", "), alpha1),
        ),
    )(input)?;

    let ans: HashMap<String, (Module, Vec<String>)> = ans
        .into_iter()
        .map(|(l, r)| {
            (
                l.0.to_string(),
                (l.1, r.into_iter().map(|a| a.to_string()).collect_vec()),
            )
        })
        .collect();

    Ok((input, ans))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(solve(ti), "Answer".to_string());
    }
}
