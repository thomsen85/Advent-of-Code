use std::cmp::Ordering;

fn main() {
    dbg!(solve(include_str!("../../inputs/day9.txt")));
}

#[derive(Debug, Clone, Copy)]
enum Part {
    File(usize, usize),
    Free(usize),
}

fn solve(input: &str) -> String {
    use Part::*;

    let mut t = Vec::new();
    let mut free_space = false;
    let mut id = 0;
    for c in input.trim().chars() {
        if free_space {
            t.push(Free(
                c.to_digit(10)
                    .unwrap_or_else(|| panic!("{:?} is not a digit", c)) as usize,
            ));
            id += 1
        } else {
            t.push(File(id, c.to_digit(10).unwrap() as usize));
        }
        free_space = !free_space;
    }

    // let mut p_end = t.len() - 1;

    for p_end in (1..t.len()).rev() {
        let mut p = 0;
        loop {
            if p >= t.len() || p >= p_end {
                break;
            }

            let free_space;
            if let Free(space) = t[p] {
                free_space = space;
            } else {
                p += 1;
                continue;
            }

            let file_id;
            let file_space;
            if let File(id, space) = t[p_end] {
                file_space = space;
                file_id = id;
            } else {
                break;
            }

            match file_space.cmp(&free_space) {
                Ordering::Equal => {
                    t[p] = t[p_end];
                    t[p_end] = Free(file_space);
                    break;
                }
                Ordering::Less => {
                    t[p] = File(file_id, file_space);
                    t[p_end] = Free(file_space);
                    t.insert(p + 1, Free(free_space - file_space));
                    break;
                }
                _ => (),
            }
            p += 1
        }
    }
    dbg!(&t);

    t.iter()
        .scan(0, |acc, a| match a {
            File(id, size) => {
                let s = (*acc..(*acc + size)).map(|i| i * id).sum::<usize>();
                *acc += size;
                Some(s)
            }

            Free(size) => {
                *acc += size;
                Some(0)
            }
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "2333133121414131402";
        assert_eq!(solve(ti), "2858".to_string());
    }
}
