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

        let old = get_pattern_sums(&map, 1000, 1000);
        let mut new: Option<(bool, usize, (Option<usize>, Option<usize>))> = None;

        for y in 0..map.len() {
            let mut matched = false;
            for x in 0..map[0].len() {
                let mut map_c = map.clone();
                if map_c[y][x] == '.' {
                    map_c[y][x] = '#'
                } else {
                    map_c[y][x] = '.'
                }

                let ans =
                    get_pattern_sums(&map_c, old.2 .0.unwrap_or(1000), old.2 .1.unwrap_or(1000));
                if ans.0 {
                    new = Some(ans);
                    sum += ans.1;
                    matched = true;
                    break;
                }
            }
            if matched {
                break;
            }
        }
        assert!(new.is_some(), "{} was not correct", formation);
    }

    sum.to_string()
}

fn get_pattern_sums(
    map: &Vec<Vec<char>>,
    skip_row: usize,
    skip_col: usize,
) -> (bool, usize, (Option<usize>, Option<usize>)) {
    let mut sum = 0;
    let mut row_breaked = (false, 0);
    for i in 1..map[0].len() {
        if skip_col == i {
            continue;
        }
        let mut matches = true;
        for row in map {
            if !*&row[..i].iter().rev().zip(&row[i..]).all(|(a, b)| a == b) {
                matches = false;
            }
        }
        if matches {
            row_breaked = (true, i);

            // dbg!("col split that match", i);
            break;
        }
    }

    let row_i = {
        if row_breaked.0 {
            sum += row_breaked.1;
            Some(row_breaked.1)
        } else {
            None
        }
    };

    let mut col_breaked = (false, 0);
    for i in 1..map.len() {
        if skip_row == i {
            continue;
        }
        let mut matches = true;
        for col_n in 0..map[0].len() {
            let col = get_column(&map, col_n);
            if !*&col[..i].iter().rev().zip(&col[i..]).all(|(a, b)| a == b) {
                matches = false;
            }
        }
        if matches {
            col_breaked = (true, i);

            // dbg!("row_split at ", i);
            break;
        }
    }

    let col_i = {
        if col_breaked.0 {
            sum += col_breaked.1 * 100;
            Some(col_breaked.1)
        } else {
            None
        }
    };

    (row_breaked.0 || col_breaked.0, sum, (col_i, row_i))
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
        assert_eq!(solve(ti), "400".to_string());
    }
}
