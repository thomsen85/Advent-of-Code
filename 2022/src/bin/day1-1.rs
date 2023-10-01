fn main() {
    let input = aoc2022_rust::utils::lines_from_file("inputs/day1.txt");
    let mut acc = 0;
    let mut res = Vec::new();
    for entry in input {
        if !entry.is_empty() {
            acc += entry.parse::<i32>().unwrap();
        } else {
            res.push(acc);
            acc = 0;
        }
    }
    println!("{:?}", res.into_iter().max().unwrap());
}
