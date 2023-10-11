#![allow(dead_code)]
// Program needs optimizations
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::newline,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
struct Map {
    valves: HashMap<String, Rc<RefCell<Valve>>>,
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
    to: Vec<Rc<RefCell<Valve>>>,
}

impl Valve {
    fn new(name: String, flow_rate: i32) -> Self {
        Self {
            name,
            flow_rate,
            to: Vec::new(),
        }
    }
    fn add_to(&mut self, to: Rc<RefCell<Valve>>) {
        self.to.push(to);
    }
}

#[derive(Debug, Clone)]
enum Move {
    Move(String),
    Open(String),
}

impl Move {
    fn get_pos(&self) -> &str {
        return match self {
            Self::Move(s) => s,
            Self::Open(s) => s,
        };
    }
}

fn main() {
    let graph = parser(&common::utils::string_from_file("inputs/day16t.txt"))
        .unwrap()
        .1;

    let mut path = Vec::new();
    path.push(Move::Move("AA".to_owned()));
    let v = get_path(&graph, 15, path, HashSet::new(), 0);
    dbg!(v);
}

fn get_path(
    map: &Map,
    minutes: i32,
    mut path: Vec<Move>,
    mut opened_valves: HashSet<String>,
    score: i32,
) -> (Vec<Move>, HashSet<String>, i32) {
    if minutes <= 0 {
        return (path, opened_valves, score);
    }

    let score = score
        + &opened_valves
            .iter()
            .map(|v| map.valves.get(v).unwrap().borrow().flow_rate)
            .sum();

    let current_pos = &path
        .last()
        .expect("Initialized without start")
        .get_pos()
        .to_owned();
    let current_opened = opened_valves.contains(current_pos);

    let mut top_path = None;

    let current_valve = map.valves.get(current_pos).unwrap().borrow();
    if !current_opened && current_valve.flow_rate != 0 {
        path.push(Move::Open(current_pos.clone().to_owned()));
        opened_valves.insert(current_pos.to_owned());
        top_path = Some(get_path(
            map,
            minutes - 1,
            path.clone(),
            opened_valves.clone(),
            score,
        ));
    }

    for to in &current_valve.to {
        path.push(Move::Move(to.borrow().name.clone()));
        let p = get_path(map, minutes - 1, path.clone(), opened_valves.clone(), score);
        if let Some(t) = &top_path {
            if t.2 < p.2 {
                top_path = Some(p);
            }
        } else {
            top_path = Some(p)
        }
    }

    let p = get_path(map, minutes - 1, path.clone(), opened_valves.clone(), score);
    if let Some(t) = &top_path {
        if t.2 < p.2 {
            top_path = Some(p);
        }
    } else {
        top_path = Some(p);
    }
    return top_path.unwrap();
}

fn parser(input: &str) -> IResult<&str, Map> {
    let (input, out) = separated_list1(newline, valve_parser)(input)?;

    let out: Vec<_> = out
        .into_iter()
        .map(|(valve, to)| (Rc::new(RefCell::new(valve)), to))
        .collect();

    let mut res = HashMap::with_capacity(out.len());
    out.iter().for_each(|(valve, _)| {
        res.insert(valve.borrow().name.to_owned(), Rc::clone(valve));
    });

    for (valve, tos) in out {
        let valve_c = Rc::clone(&valve);
        let mut valve_m = RefCell::borrow_mut(&valve_c);
        for to in tos {
            let rf = Rc::clone(res.get(to).expect(&format!("{to} not in list")));
            valve_m.to.push(rf);
        }
    }

    let graph = Map { valves: res };

    Ok((input, graph))
}

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
fn valve_parser(input: &str) -> IResult<&str, (Valve, Vec<&str>)> {
    let (input, name) = preceded(tag("Valve "), take(2_usize))(input)?;
    let (input, flow_rate) =
        preceded(tag(" has flow rate="), nom::character::complete::i32)(input)?;

    let (input, to) = preceded(
        alt((
            tag("; tunnel leads to valve "),
            tag("; tunnels lead to valves "),
        )),
        separated_list1(tag(", "), take(2_usize)),
    )(input)?;

    Ok((input, (Valve::new(name.to_owned(), flow_rate), to)))
}
