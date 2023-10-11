fn main() {
    let input: Vec<char> = common::utils::lines_from_file("inputs/day6.txt")
        .first()
        .unwrap()
        .to_owned()
        .chars()
        .collect();
    let mut p = 0;
    println!("{:?}", input.len());
    while p + 3 < input.len() {
        let mut m = true;
        for i in 0..4 {
            for j in 0..4 {
                if j != i && input[p + i] == input[p + j] {
                    m = false;
                }
            }
        }

        if m {
            break;
        }
        p += 1
    }

    println!("Match at: {}", p + 4);
}
