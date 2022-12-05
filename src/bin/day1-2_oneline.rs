use itertools::Itertools;

fn main() {
    println!(
        "{:?}",
        aoc2022_rust::utils::lines_from_file("inputs/day1.txt")
            .into_iter()
            .fold(vec![0], |mut acc, x| {
                if x.is_empty() {
                    acc.push(0)
                } else {
                    let len = acc.len() - 1;
                    acc[len] += x.parse::<i32>().unwrap()
                };
                acc
            })
            .iter()
            .sorted()
            .rev()
            .take(3)
            .sum::<i32>()
    )
}
