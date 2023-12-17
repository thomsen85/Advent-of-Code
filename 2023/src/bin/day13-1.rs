use itertools::Itertools;
fn main() {
    dbg!(solve(include_str!("../../inputs/day13.txt")));
}

fn solve(input: &str) -> String {
    let mut sum = 0;
    for formation in input.split("\n\n") {
        let map = formation
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let mut row_breaked = (false, 0);
        for i in 1..map[0].len() {
            let mut matches = true;
            for row in &map {
                if !*&row[..i].iter().rev().zip(&row[i..]).all(|(a, b)| a == b) {
                    matches = false;
                }
            }
            if matches {
                row_breaked = (true, i);

                dbg!(i);
                break;
            }
        }
        if row_breaked.0 {
            sum += row_breaked.1
        }

        let mut col_breaked = (false, 0);
        for i in 1..map.len() {
            let mut matches = true;
            for col_n in 0..map[0].len() {
                let col = get_column(&map, col_n);
                if !*&col[..i].iter().rev().zip(&col[i..]).all(|(a, b)| a == b) {
                    matches = false;
                }
            }
            if matches {
                col_breaked = (true, i);

                dbg!(i);
                break;
            }
        }

        if col_breaked.0 {
            sum += col_breaked.1 * 100
        }
    }

    sum.to_string()
}

fn get_column(map: &Vec<Vec<char>>, col: usize) -> Vec<char> {
    map.iter().map(|line| line[col]).collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(solve(ti), "405".to_string());
    }
}
