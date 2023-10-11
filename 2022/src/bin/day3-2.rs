use common::utils::lines_from_file;

fn main() {
    let input = lines_from_file("inputs/day3.txt");

    let mut res = Vec::new();

    for i in (0..input.len()).step_by(3) {
        println!("{:?}", &input[i..i + 3]);
        for c in input[i].chars() {
            if input[i + 1].contains(c) && input[i + 2].contains(c) {
                res.push(c);
                break;
            }
        }
    }

    let val: u32 = res.into_iter().map(char_to_priority).sum();

    println!("Sum = {}", val);
}

fn char_to_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}
