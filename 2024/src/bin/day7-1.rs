use itertools::Itertools;

fn main() {
    dbg!(solve(include_str!("../../inputs/day7.txt")));
}

fn solve(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let (tot, exp) = line.split_once(": ").unwrap();
        let tot = tot.parse::<i64>().unwrap();

        let exp = exp
            .split_whitespace()
            .map(|a| a.parse::<i64>().unwrap())
            .collect_vec();

        let mut stack = vec![(&exp[1..], exp[0])];

        while let Some((e, s)) = stack.pop() {
            if e.is_empty() {
                if s == tot {
                    sum += tot;
                    break;
                }
                continue;
            }

            stack.push((&e[1..], s * e[0]));
            stack.push((&e[1..], s + e[0]));
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(solve(ti), "3749".to_string());
    }
}
