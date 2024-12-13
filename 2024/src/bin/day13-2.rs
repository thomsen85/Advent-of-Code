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
    let mut sum = 0;
    for price in input.split("\n\n") {
        let (ax, ay) = price
            .lines()
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap();
        let (a1, a2) = (
            ax.split_once("+").unwrap().1.parse::<i64>().unwrap(),
            ay.split_once("+").unwrap().1.parse::<i64>().unwrap(),
        );
        let (bx, by) = price
            .lines()
            .nth(1)
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap();
        let (b1, b2) = (
            bx.split_once("+").unwrap().1.parse::<i64>().unwrap(),
            by.split_once("+").unwrap().1.parse::<i64>().unwrap(),
        );
        let (x, y) = price
            .lines()
            .nth(2)
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap();
        let (x, y) = (
            x.split_once("=").unwrap().1.parse::<i64>().unwrap(),
            y.split_once("=").unwrap().1.parse::<i64>().unwrap(),
        );
        let (c1, c2) = (x + ADD, y + ADD);

        // Why no work :(
        // let a: Array2<f64> = array![[ax as f64, bx as f64], [bx as f64, by as f64]];
        // let b: Array1<f64> = array![x as f64, y as f64];
        //
        // if let Some(ans) = a.solve_into(b) {
        //     sum += ans.0 * 3 + ans.1
        // }
        //

        // Copy pasta wikipedia cramers rule :(
        let x = (c1 * b2 - b1 * c2) / (a1 * b2 - b1 * a2);
        let y = (a1 * c2 - c1 * a2) / (a1 * b2 - b1 * a2);

        // Check if correct
        if x * a1 + y * b1 == c1 && a2 * x + b2 * y == c2 {
            sum += x * 3 + y;
        }

        // Also no work
        // let b_p = (ax * y - ay * x) % (-ay * bx + by * ax);
        // let b_p
        // if b_p == 0 {
        //     // let b = (ax * y - ay * x) / (-ay * bx + by * ax);
        //     // let a = (x - bx * b) / ax;
        //     sum += a * 3 + b
        // }
        // dbg!(b_p, b);
        // for i in 0..100 {
        //     for j in 0..100 {
        //         if ax * i + bx * j == x && ay * i + by * j == y {
        //             sum += i * 3 + j;
        //         }
        //     }
        // }
    }

    sum.to_string()
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
        assert_eq!(solve(ti), "8753186089080".to_string());
    }

    #[test]
    fn test_2() {
        let ti = "Button A: X+100, Y+100
Button B: X+20, Y+20
Prize: X=1000, Y=10000
";
        assert_eq!(solve(ti), "".to_string());
    }
}
