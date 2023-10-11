#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Formatter},
};

use nom::{
    branch::alt, bytes::complete::tag, bytes::complete::take, multi::separated_list1,
    sequence::preceded, IResult,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct ValveName {
    name: [u8; 2],
}

#[derive(Debug)]
struct Valve {
    name: ValveName,
    to: Vec<ValveName>,
    flow_rate: u8,
}

impl ValveName {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, name) = take(2_usize)(input)?;
        let name = name.as_bytes().to_owned();
        let valve_name = ValveName {
            name: name.try_into().expect("Valve was over 2 chars long"),
        };
        Ok((input, valve_name))
    }
}

impl fmt::Debug for ValveName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name[0] as char, self.name[1] as char)
    }
}

impl Valve {
    /// Parses this type of sting:
    ///
    /// Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, name) = preceded(tag("Valve "), ValveName::parse)(input)?;
        let (input, flow_rate) =
            preceded(tag(" has flow rate="), nom::character::complete::u8)(input)?;
        let (input, to) = preceded(
            alt((
                tag("; tunnel leads to valve "),
                tag("; tunnels lead to valves "),
            )),
            separated_list1(tag(", "), ValveName::parse),
        )(input)?;

        Ok((
            input,
            Valve {
                name,
                to,
                flow_rate,
            },
        ))
    }
}

fn main() {
    let input = common::utils::lines_from_file("inputs/day16t.txt");
    let mut valves = HashMap::new();
    for line in input {
        let valve = Valve::parse(&line)
            .expect(&format!("Could not parse string {}", &line))
            .1;
        valves.insert(valve.name, valve);
    }

    dbg!(valves);
}

fn release_pressure(valves: HashMap<ValveName, Valve>, opened_valves: HashSet<ValveName>) {}
