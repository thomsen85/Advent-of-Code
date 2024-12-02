use itertools::Itertools;

// For number types

fn main() {
    dbg!(solve(include_str!("../../inputs/day2.txt")));
}

fn solve(input: &str) -> String {
    let mut depth = 0;
    let mut aim = 0;
    let mut f = 0;

    for line in input.lines() {
        let (w, m) = line.split(" ").collect_tuple().unwrap();
        let m = m.parse::<i32>().unwrap();

        match w {
            "forward" => {
                f += m;
                depth += m * aim;
            }
            "down" => {
                aim += m;
            }
            "up" => {
                aim -= m;
            }
            _ => panic!(),
        }
    }

    (depth * f).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(solve(ti), "900".to_string());
    }
}
