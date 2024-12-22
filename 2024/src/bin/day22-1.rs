use std::time::Instant;

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day22.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    input
        .lines()
        .map(|a| a.parse::<i64>().unwrap())
        .map(|mut i| {
            const N: i64 = 2000;

            for _ in 0..N {
                let r = i * 64;
                i ^= r;
                i %= 16777216;

                let r = i / 32;
                i ^= r;
                i %= 16777216;

                let r = i * 2048;
                i ^= r;
                i %= 16777216;
            }

            i
        })
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "1
10
100
2024";
        assert_eq!(solve(ti), "37327623".to_string());
    }
}
