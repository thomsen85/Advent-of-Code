use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
use std::{collections::HashMap, time::Instant};
// For number types
use nom::character::complete as cnom;

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day24.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let (var, instructions) = input.split_once("\n\n").unwrap();
    let mut vars = var
        .lines()
        .map(|a| a.split_once(": ").unwrap())
        .map(|a| (a.0.to_owned(), a.1.parse::<u8>().unwrap()))
        .collect::<HashMap<String, u8>>();

    let mut ins = instructions
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(exp, d)| {
            let (a, b, c) = exp.split_whitespace().collect_tuple().unwrap();
            ((a, b, c), d)
        })
        .collect_vec();

    let mut i = 0;

    let x_bin = vars
        .iter()
        .filter(|(k, _v)| k.starts_with("x"))
        .sorted()
        .rev()
        .map(|(_k, v)| v.to_string())
        .join("");

    let y_bin = vars
        .iter()
        .filter(|(k, _v)| k.starts_with("y"))
        .sorted()
        .rev()
        .map(|(_k, v)| v.to_string())
        .join("");

    let x = u64::from_str_radix(&x_bin, 2).unwrap();
    let y = u64::from_str_radix(&y_bin, 2).unwrap();
    let z = x + y;
    let z_bin = format!("{:b}", z);

    dbg!(ins
        .iter()
        .filter(|((v1, op, v2), ans)| {
            *op == "XOR"
                && (v1.starts_with("x") || v1.starts_with("y"))
                && (v2.starts_with("x") || v2.starts_with("y"))
        })
        .map(|((v1, op, v2), ans)| {
            (
                [*v1, *v2].into_iter().sorted().join(" ^ "),
                ans,
                ins.iter()
                    .filter(|((v11, op1, v21), ans1)| *op1 == "XOR" && (v11 == ans || v21 == ans))
                    .map(|(_, a)| a)
                    .collect_vec(),
            )
        })
        .filter(|(a, b, c)| Some(&a[1..3]) != c.first().map(|v| &v[1..]))
        .sorted_by_key(|a| a.0.clone())
        .collect_vec());

    // XOR Wrongs
    // x00 -> ? // Correct because first
    // x07 -> rts | z07
    // x12 -> jpj | z12
    // x26 -> kgj | z26
    // x34 -> ? vvw is only leads to OR. z34 comes from chv XOR fqf
    // chv is y34 and x34 so wrong is
    let wrong = vec!["vvw", "chv", "rts", "z07", "jpj", "z12", "kgj", "z26"]
        .iter()
        .sorted()
        .join(",");

    dbg!(wrong);

    todo!();
    while !ins.is_empty() {
        i %= ins.len();
        if !vars.contains_key(ins[i].0 .0) || !vars.contains_key(ins[i].0 .2) {
            i += 1;
            continue;
        }
        let v1 = vars.get(ins[i].0 .0).unwrap();
        let v2 = vars.get(ins[i].0 .2).unwrap();

        let ans = match ins[i].0 .1 {
            "XOR" => v1 ^ v2,
            "OR" => v1 | v2,
            "AND" => v1 & v2,
            _ => panic!(),
        };

        *vars.entry(ins[i].1.to_string()).or_insert(0) = ans;
        ins.remove(i);
        i += 1;
    }

    let b = vars
        .into_iter()
        .filter(|(k, _v)| k.starts_with("z"))
        .sorted()
        .rev()
        .map(|(_k, v)| v.to_string())
        .join("");

    u64::from_str_radix(&b, 2).unwrap().to_string();
    println!("{}", b);
    println!("{}", z_bin);
    "".to_string()
}

fn parse(input: &str) -> IResult<&str, ()> {
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!(solve(ti), "2024".to_string());
    }
}
