use common::strings::string_to_extracted_nums_t_vec;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
use num::Float;
use std::time::Instant;
// For number types

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day13.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}
const ADD: i64 = 10000000000000;
// const ADD: i64 = 0;

fn solve(input: &str) -> String {
    input
        .split("\n\n")
        .map(|price| {
            let [a1, a2, b1, b2, c1, c2]: [_; 6] = string_to_extracted_nums_t_vec::<i64>(price)
                .try_into()
                .unwrap();

            let (c1, c2) = (c1 + ADD, c2 + ADD);

            let a = array![[a1, b1], [a2, b2]];
            let af = a.map(|&x| x as f64);
            let b = array![c1, c2];
            let bf = b.map(|&x| x as f64);

            let v = af.solve(&bf).unwrap().mapv(|v| v.round() as i64);

            if a.dot(&v) == b {
                v[0] * 3 + v[1]
            } else {
                0
            }
        })
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(solve(ti), "875318608908".to_string());
    }
}
