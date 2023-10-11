use common::utils::lines_from_file;

fn main() {
    let input = lines_from_file("inputs/day3.txt");

    let mut res = Vec::new();
    for sack in input {
        let len = sack.len() / 2;
        let (f, b) = sack.split_at(len);

        for char in f.chars() {
            if b.contains(char) {
                res.push(char);
                break;
            }
        }
    }

    let val = res.into_iter().fold(0, |acc, c| acc + char_to_priority(c));

    println!("Sum = {}", val);
}

fn char_to_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}
