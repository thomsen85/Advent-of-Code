fn main() {
    dbg!(solve(include_str!("../../inputs/day1.txt")));
}

fn solve(input: &str) -> String {
    let mut pos: i32 = 50;
    let mut zeros = 0;
    for rot in input.lines() {
        let num = rot.split_at(1).1.parse::<i32>().unwrap();
        let a = num / 100;
        let num = num % 100;
        let mut overflowed = false;
        let prev_pos = pos;

        dbg!(rot);
        if num != 0 {
            if rot.starts_with("L") {
                pos -= num;
                if pos < 0 {
                    dbg!("overflowL");

                    pos += 100;
                    if prev_pos != 0 {
                        zeros += 1;
                    }
                    overflowed = true;
                }
            } else {
                pos += num;
                if pos >= 100 {
                    dbg!("overflowR");
                    zeros += 1;
                    pos -= 100;
                    overflowed = true;
                }
            }
            if !overflowed && pos == 0 {
                dbg!("zero");
                zeros += 1
            }
        }

        zeros += a;
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
        assert_eq!("6".to_string(), solve(a));
    }

    #[test]
    fn test_2() {
        let a = "R1000";
        assert_eq!("10", solve(a))
    }

    #[test]
    fn test_3() {
        let a = "L50
R1000
R100
";
        assert_eq!("12", solve(a))
    }
}
