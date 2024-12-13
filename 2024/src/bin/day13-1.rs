use std::time::Instant;
// For number types

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day13.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let mut sum = 0;
    for price in input.split("\n\n") {
        let (ax, ay) = price
            .lines()
            .nth(0)
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap();
        let (ax, ay) = (
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
        let (bx, by) = (
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
        dbg!(ax, ay, bx, by, x, y);

        for i in 0..100 {
            for j in 0..100 {
                if ax * i + bx * j == x && ay * i + by * j == y {
                    sum += i * 3 + j;
                }
            }
        }
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
        assert_eq!(solve(ti), "480".to_string());
    }
}
