use std::collections::HashSet;

use common::{datastructs::vec2::Vec2, strings::string_to_char_grid};

fn main() {
    dbg!(solve(include_str!("../../inputs/day4.txt")));
}

fn solve(input: &str) -> String {
    let g = string_to_char_grid(input);
    let word = "MAS";
    let mut sum = 0;

    let mut stack = Vec::new();

    for row_i in 0..g.len() {
        for col_i in 0..g.first().unwrap().len() {
            if g[row_i][col_i] == 'M' {
                stack.push((Vec2::new(row_i as i32, col_i as i32), 1, None));
            }
        }
    }

    let mut diags = HashSet::new();

    while let Some(n) = stack.pop() {
        let nb = vec![
            Vec2::ARR_UP_RIGHT,
            Vec2::ARR_DOWN_RIGHT,
            Vec2::ARR_DOWN_LEFT,
            Vec2::ARR_UP_LEFT,
        ];
        for nbr in nb {
            if let Some(dir) = n.2 {
                if nbr != dir {
                    continue;
                }
            }

            let nv = nbr + n.0;
            if nv.x >= g.first().unwrap().len() as i32
                || nv.x < 0
                || nv.y >= g.len() as i32
                || nv.y < 0
            {
                continue;
            }
            if g[nv.x as usize][nv.y as usize] == word.chars().nth(n.1).unwrap() {
                if n.1 + 1 == word.len() {
                    if diags.contains(&n.0) {
                        sum += 1;
                    } else {
                        diags.insert(n.0);
                    }
                    break;
                } else {
                    stack.push((nv, n.1 + 1, Some(nbr)));
                }
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(solve(ti), "9".to_string());
    }
}
