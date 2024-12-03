fn main() {
    dbg!(solve(include_str!("../../inputs/day3.txt")));
}

fn solve(mut input: &str) -> String {
    let mut sum = 0;
    while !input.is_empty() {
        if input.starts_with("mul(") {
            input = &input[4..];
            let first = input.split_once(",").unwrap().0;
            let first_val;
            if (1..=3).contains(&first.len()) {
                if let Ok(val) = first.parse::<i32>() {
                    first_val = val;
                    input = &input[(first.len() + 1)..];
                    let second = input.split_once(")").unwrap().0;
                    let second_val;
                    if (1..=3).contains(&second.len()) {
                        if let Ok(val) = second.parse::<i32>() {
                            second_val = val;
                            sum += first_val * second_val;
                        }
                    }
                }
            }
        }
        input = &input[1..];
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(solve(ti), "161".to_string());
    }
}
