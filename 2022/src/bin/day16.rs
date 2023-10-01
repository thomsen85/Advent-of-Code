use std::{ops::DerefMut, cell::RefCell, rc::Rc, collections::{HashMap, BinaryHeap}};

use nom::{
    bytes::complete::{tag, take},
    character::complete::{newline, multispace1},
    multi::separated_list1,
    sequence::{preceded},
    IResult,
};
#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
    to: Vec<String>,
}

impl Valve {
    fn new(name: String, flow_rate: i32, to: Vec<String>) -> Self {
        Self {
            name,
            flow_rate,
            to,
        }
    }
}

fn main() {
    let input = parser(&aoc2022_rust::utils::string_from_file("inputs/day16t.txt"))
        .unwrap()
        .1;

    dbg!(input);
}

fn get_optimal_path(input: &HashMap<String, Valve>, start_point: String) {
    
    
}

fn parser(input: &str) -> IResult<&str, HashMap<String, Valve>> {
    let (input, out) = separated_list1(newline, valve_parser)(input)?;
    let mut res = HashMap::with_capacity(out.len());
    out.into_iter().for_each(|v| {res.insert(v.name.clone(), v);});
    Ok((input, res))
}

//Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
fn valve_parser(input: &str) -> IResult<&str, Valve> {
    let (input, name) = preceded(tag("Valve "), take(2_usize))(input)?;
    let (input, flow_rate) =
        preceded(tag(" has flow rate="), nom::character::complete::i32)(input)?;
    let (input, to) = preceded(
        tag("; tunnels lead to valves "),
        separated_list1(tag(", "), take(2_usize)),
    )(input)?;

    let to = to.into_iter().map(str::to_owned).collect();
    Ok((input, (Valve::new(name.to_owned(), flow_rate, to))))
}
