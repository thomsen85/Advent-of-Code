fn main() {
    let input = common::utils::lines_from_file("inputs/day4.txt");

    let mut amount = 0;
    for line in input {
        let ((f1, f2), (b1, b2)) = parser(line);

        let fd = f2 - f1;
        let bd = b2 - b1;
        assert!(fd >= 0);
        assert!(bd >= 0);

        let range = (b1 - fd)..=(b1 + bd);
        if range.contains(&f1) {
            amount += 1;
        }
    }
    println!("Amount = {}", amount);
}

fn parser(string: String) -> ((i32, i32), (i32, i32)) {
    let (f, b) = string.split_once(',').unwrap();
    let (f1, f2) = f.split_once('-').unwrap();
    let (b1, b2) = b.split_once('-').unwrap();

    (
        (f1.parse().unwrap(), f2.parse().unwrap()),
        (b1.parse().unwrap(), b2.parse().unwrap()),
    )
}
