fn main() {
    let input = common::utils::string_from_file("inputs/day1.txt");

    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor == -1 {
            dbg!(i + 1);
            break;
        }
    }
}
