use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alpha1, anychar, multispace0, multispace1, newline, space0, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult, Parser,
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

#[derive(Debug)]
enum Rule<'a> {
    Condition(char, char, u32, &'a str),
    End(&'a str),
}

fn main() {
    dbg!(solve(include_str!("../../inputs/day19.txt")));
}

fn solve(input: &str) -> String {
    let (workflows, parts) = parse(input).unwrap().1;
    let workflows: HashMap<&str, Vec<Rule>> = HashMap::from_iter(workflows.into_iter());
    //dbg!(&workflows);

    let mut sum = 0;
    for part in parts {
        let mut loc = "in";
        while loc != "R" && loc != "A" {
            let wf = workflows.get(loc).expect(&format!("{} not found", loc));
            for rule in wf {
                match rule {
                    Rule::Condition(arg, op, amnt, new_loc) => {
                        let p_val = match arg {
                            'x' => part.x,
                            'm' => part.m,
                            'a' => part.a,
                            's' => part.s,
                            c => panic!("Not expectecd {}", c),
                        };
                        if match op {
                            '<' => p_val < *amnt,
                            '>' => p_val > *amnt,
                            x => panic!("Not expected {}", x),
                        } {
                            loc = new_loc;
                            break;
                        }
                    }
                    Rule::End(new_loc) => loc = new_loc,
                }
            }
            //dbg!(loc, &part);
        }

        if loc == "A" {
            sum += part.x + part.m + part.a + part.s;
        }
    }

    sum.to_string()
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
        assert_eq!(solve(ti), "19114".to_string());
    }
}
