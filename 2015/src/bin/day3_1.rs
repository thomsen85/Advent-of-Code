use std::collections::HashSet;

fn main() {
    let input = common::utils::string_from_file("inputs/day3.txt");
    let len = get_houses(&input);
    dbg!(len);
}

fn get_houses(input: &str) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut coords = HashSet::new();
    coords.insert((x, y));
    for c in input.chars() {
        match c {
            '^' => y += 1,
            '<' => x -= 1,
            'v' => y -= 1,
            '>' => x += 1,
            _ => panic!("Unexpected char {:?}", c),
        }
        coords.insert((x, y));
    }
    coords.len()
}

#[cfg(test)]
mod tests {
    use crate::get_houses;

    #[test]
    fn simple_test() {
        let test = "^>v<";
        assert_eq!(get_houses(test), 4);
    }
    #[test]
    fn simple_test2() {
        let test = "^v^v^v^v^v";
        assert_eq!(get_houses(test), 2);
    }
}
