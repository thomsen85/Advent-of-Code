use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1, anychar, char, multispace0, multispace1, newline, space0, space1,
    },
    combinator::map,
    sequence::{delimited, preceded, tuple},
    IResult,
};
// For number types
use nom::character::complete as cnom;

#[derive(Debug)]
enum V {
    Const(i64),
    Dep(String, char, String),
}

fn main() {
    dbg!(solve(include_str!("../../inputs/day21.txt")));
}

fn solve(input: &str) -> String {
    let mut m = HashMap::new();
    for line in input.lines() {
        let line = parse(line).unwrap().1;
        m.insert(line.0, line.1);
    }
    let mut max = i64::MAX / 100;
    let mut min = i64::MIN / 100;
    let mut t = 0;
    let mut d = 200;
    loop {
        if let V::Dep(a, _, b) = m.get("root").unwrap() {
            let a = get(&m, a, t);
            let b = get(&m, b, t);
            dbg!(a, b, a == b);
            dbg!(a - b);

            if a == b {
                return t.to_string();
            } else if a > b {
                min = t
            } else {
                max = t;
            }
            t = (max + min) / 2;
        }

        if d < 0 {
            dbg!("depth reached ");
            break;
        }

        t += 1;
        d -= 1
    }

    t.to_string()
}

fn get(m: &HashMap<String, V>, v: &str, t: i64) -> i64 {
    if v == "humn" {
        return t;
    }
    match m.get(v).unwrap() {
        V::Dep(a, o, b) => {
            let a = get(m, a, t);
            let b = get(m, b, t);
            match o {
                '+' => a + b,
                '*' => a * b,
                '-' => a - b,
                '/' => a / b,
                _ => panic!(),
            }
        }
        V::Const(a) => *a,
    }
}

fn parse(s: &str) -> IResult<&str, (String, V)> {
    let (s, a) = alpha1(s)?;

    let (s, _) = tag(": ")(s)?;
    let (s, v) = alt((
        map(cnom::i64, V::Const),
        map(
            tuple((
                alpha1,
                preceded(multispace1, anychar),
                preceded(multispace1, alpha1),
            )),
            |(a, c, b): (&str, char, &str)| V::Dep(a.to_string(), c, b.to_string()),
        ),
    ))(s)?;

    Ok((s, (a.to_string(), v)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        assert_eq!(solve(ti), "301".to_string());
    }
}
