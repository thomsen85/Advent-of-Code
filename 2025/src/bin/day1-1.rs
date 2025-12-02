fn main() {
    dbg!(solve(include_str!("../../inputs/day1.txt")));
}

fn solve(input: &str) -> String {
    let mut pos: i32 = 50;
    let mut zeros = 0;
    for rot in input.lines() {
        let num = rot.split_at(1).1.parse::<i32>().unwrap() % 100;

        if rot.starts_with("L") {
            pos -= num;
            if pos < 0 {
                pos += 100;
            }
        } else {
            pos += num;
            if pos >= 100 {
                pos -= 100
            }
        }

        if pos == 0 {
            zeros += 1;
        }
    }
    zeros.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let a = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("4".to_string(), solve(a));
    }
}
