use common::utils::string_to_char_grid;

fn main() {
    dbg!(solve(include_str!("../../inputs/day3.txt")));
}

fn solve(input: &str) -> String {
    let s = input.lines().next().unwrap().len();
    let a = string_to_char_grid(input).iter().enumerate().fold(
        vec![vec![0; s]; 2],
        |mut acc, (_, x)| {
            x.iter().enumerate().for_each(|(ci, c)| {
                let c = c.to_digit(10).unwrap() as usize;
                acc[c][ci] += 1;
            });
            acc
        },
    );

    let g = a[0]
        .iter()
        .zip(a[1].clone())
        .map(|(a, b)| if *a > b { '0' } else { '1' })
        .collect::<String>();

    let e = a[0]
        .iter()
        .zip(a[1].clone())
        .map(|(a, b)| if *a < b { '0' } else { '1' })
        .collect::<String>();

    dbg!(&g);
    dbg!(&e);

    (isize::from_str_radix(&g, 2).unwrap() * isize::from_str_radix(&e, 2).unwrap()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(solve(ti), "198".to_string());
    }
}
