use itertools::Itertools;

fn main() {
    // 439880 to high
    dbg!(
        solve(include_str!("../../inputs/day10.txt").trim().to_string())
            .len()
            .to_string()
    );
}

fn solve(mut input: String) -> String {
    dbg!(&input);
    for _ in 0..50 {
        let s = input.chars().collect_vec();

        let mut sub = 1;
        let mut seq = Vec::new();
        for c in 1..input.len() {
            if s[c] == s[c - 1] {
                sub += 1;
            } else {
                seq.push(sub.to_string());
                seq.push(s[c - 1].to_string());
                sub = 1;
            }
        }
        seq.push(sub.to_string());
        seq.push(s.iter().last().unwrap().to_string());
        input = seq.join("").to_string();
    }

    input
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("1", "11")]
    #[case("11", "21")]
    #[case("21", "1211")]
    #[case("1211", "111221")]
    #[case("111221", "312211")]
    fn test_1(#[case] inn: &str, #[case] out: &str) {
        assert_eq!(solve(inn.to_string()), out.to_string());
    }
}
