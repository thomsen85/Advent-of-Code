fn main() {
    let input = common::utils::string_from_file("inputs/day1.txt");
    let floor = input
        .chars()
        .into_iter()
        .fold(0, |acc, x| if x == '(' { acc + 1 } else { acc - 1 });

    dbg!(floor);
}
