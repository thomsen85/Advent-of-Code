use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, multispace1, newline, space0, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
// For number types
use nom::character::complete as cnom;

#[derive(Debug, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

enum Module {
    Brodcast(),
    FlipFlop(bool),
    Conjuction(Vec<Pulse>),
}

impl Module {
    fn tick(&mut self, pulse: Pulse) -> Option<Pulse> {
        use Module::*;
        use Pulse::*;
        match self {
            Brodcast() => Some(pulse),
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
            Conjuction(mut v) => {
                v.push(pulse);
                // Dette funker ikke
                if v.iter().all(|p| matches!(p, High)) {
                    Some(Low)
                } else {
                    Some(High)
                }
            }
        }
    }
}

fn main() {
    dbg!(solve(include_str!("../../inputs/day20.txt")));
}

fn solve(input: &str) -> String {
    let p = parse(input).unwrap().1;

    todo!("Add solution");

    " ".to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<((&str, Module), Vec<&str>)>> {
    use Module::*;
    // let (input, ans) = separated_list1(
    //     newline,
    //     separated_pair(
    //         alt((
    //             map(tag("broadcaster"), |name| (name, Brodcast),
    //             map(preceded(cnom::char('%'), alpha1), |name| (name, FlipFlop(false))),
    //             map(preceded(cnom::char('&'), alpha1), |name| (name, Conjuction(Vec::new()))),
    //         )),
    //         tag(" -> "),
    //         separated_list1(tag(", "), alpha1),
    //     ),
    // )(input)?;

    todo!();
    Ok((input, ()))
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
