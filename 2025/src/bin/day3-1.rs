use itertools::Itertools;
use std::time::Instant;

// Total time used to write: 9min 18sec
fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day3.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let length = line.len();
            let l = line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
            let mut max = 0;
            let mut max_i = 0;
            for i in 0..(length - 1) {
                if l[i] > max {
                    max_i = i;
                    max = l[i];
                }
            }

            let mut max2 = 0;
            for i in (max_i + 1)..length {
                if l[i] > max2 {
                    max2 = l[i];
                }
            }

            max2 + max * 10
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve(ti), "357".to_string());
    }
}
