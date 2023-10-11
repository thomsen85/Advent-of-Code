use std::collections::HashSet;

fn main() {
    let input = common::utils::string_from_file("inputs/day3.txt");
    let len = get_houses(&input);
    dbg!(len);
}

fn get_houses(input: &str) -> usize {
    let mut x1 = 0;
    let mut y1 = 0;

    let mut x2 = 0;
    let mut y2 = 0;

    let mut santa = true;

    let mut coords = HashSet::new();
    coords.insert((x1, y1));
    for c in input.chars() {
        if santa {
            match c {
                '^' => y1 += 1,
                '<' => x1 -= 1,
                'v' => y1 -= 1,
                '>' => x1 += 1,
                _ => panic!("Unexpected char {:?}", c),
            }
            coords.insert((x1, y1));
        } else {
            match c {
                '^' => y2 += 1,
                '<' => x2 -= 1,
                'v' => y2 -= 1,
                '>' => x2 += 1,
                _ => panic!("Unexpected char {:?}", c),
            }
            coords.insert((x2, y2));
        }
        santa = !santa;
    }
    coords.len()
}

#[cfg(test)]
mod tests {
    use crate::get_houses;

    #[test]
    fn simple_test() {
        let test = "^>v<";
        assert_eq!(get_houses(test), 3);
    }
    #[test]
    fn simple_test2() {
        let test = "^v^v^v^v^v";
        assert_eq!(get_houses(test), 11);
    }
}
