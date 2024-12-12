use common::strings::string_to_t_grid;

fn main() {
    dbg!(solve(include_str!("../../inputs/day11.txt")));
}

fn solve(input: &str) -> String {
    let m = string_to_t_grid::<usize>(input, " ")
        .first()
        .unwrap()
        .clone();

    let mut prev_m = m.clone();
    for _i in 0..25 {
        let mut new_m = Vec::new();
        for num in &prev_m {
            if *num == 0 {
                new_m.push(1);
                continue;
            }

            let num_s = num.to_string();
            if num_s.len() % 2 == 0 {
                new_m.push(num_s[..num_s.len() / 2].parse::<usize>().unwrap());
                new_m.push(num_s[num_s.len() / 2..].parse::<usize>().unwrap());
                continue;
            }

            new_m.push(num * 2024);
        }

        // dbg!(&new_m);
        prev_m = new_m
    }

    prev_m.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "125 17";
        assert_eq!(solve(ti), "55312".to_string());
    }
}
