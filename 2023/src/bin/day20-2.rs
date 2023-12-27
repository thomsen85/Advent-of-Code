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
use num::Integer;

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
            FlipFlop(b) => match pulse {
                Low => {
                    if *b {
                        *b = false;
                        Some(Low)
                    } else {
                        *b = true;
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

    let to_rx = modules
        .iter()
        .filter(|(_name, (_module, tos))| tos.contains(&"rx".to_string()))
        .map(|(n, _)| n.to_owned())
        .collect_vec();

    assert!(to_rx.len() == 1);
    // Is a singe conjunction, If all are high from the input iit outputs low
    let to_rx = to_rx.into_iter().next().unwrap();

    dbg!(&to_rx);

    // See that all of these are conjuctions with only one conenction making them inverters
    // let to_to_rx = modules
    //     .iter()
    //     .filter(|(_name, (_module, tos))| tos.contains(&to_rx.0))
    //     .collect_vec();

    // dbg!(to_to_rx);

    let mut c: usize = 0;
    let mut rep: HashMap<String, usize> = HashMap::new();
    loop {
        c += 1;
        use Pulse::*;

        let mut queue =
            VecDeque::from(vec![("button".to_string(), "broadcaster".to_string(), Low)]);

        while let Some((from, to, pulse)) = queue.pop_front() {
            // println!("{} -{:?}-> {}", from, pulse, to);
            let Some((to_mod, tos)) = modules.get_mut(&to) else {
                continue;
            };

            if let Some(res) = to_mod.tick(pulse, from) {
                for to_other in tos {
                    if to_other == &to_rx && matches!(res, High) {
                        //println!("{} | {:?} From {:?} to {:?}", c, pulse, to, to_other);
                        if !rep.contains_key(&to) {
                            rep.insert(to.clone(), c);
                        }
                    }
                    queue.push_back((to.clone(), to_other.clone(), res))
                }
            }
        }
        if rep.len() == 4 {
            break;
        }
    }
    dbg!(&rep);

    let mut reps = rep.values();

    let mut lcm = *reps.next().unwrap();
    while let Some(i) = reps.next() {
        dbg!(lcm);
        lcm = lcm.lcm(i);
    }

    lcm.to_string()
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
        assert_eq!(solve(ti), "32000000".to_string());
    }

    #[test]
    fn test_2() {
        let ti = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(solve(ti), "11687500".to_string());
    }
}
