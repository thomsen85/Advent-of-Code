use itertools::Itertools;
fn main() {
    dbg!(solve(include_str!("../../inputs/day1.txt")));
}

fn solve(input: &str) -> String {
    let mut first = Vec::new();
    let mut second = Vec::new();
    input
        .lines()
        .map(|l| dbg!(l.split_whitespace().collect_vec()))
        .for_each(|v| {
            let a = v[0];
            let b = v[1];
            first.push(a.parse::<i32>().unwrap());
            second.push(b.parse::<i32>().unwrap());
        });

    first
        .iter()
        .map(|a| second.iter().filter(|b| *a == **b).count() as i32 * *a)
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(solve(ti), "31".to_string());
    }
}
