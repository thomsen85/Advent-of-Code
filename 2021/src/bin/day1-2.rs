use common::strings::string_to_t_vec;
use itertools::Itertools;
fn main() {
    dbg!(solve(include_str!("../../inputs/day1.txt")));
}

fn solve(input: &str) -> String {
    string_to_t_vec::<i32>(input)
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(solve(ti), "5".to_string());
    }
}
