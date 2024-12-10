use common::{datastructs::vec2::Vec2, strings::string_to_char_grid};

fn main() {
    dbg!(solve(include_str!("../../inputs/day4.txt")));
}

fn solve(input: &str) -> String {
    let g = string_to_char_grid(input);
    let word = "XMAS";
    let mut sum = 0;

    let mut stack = Vec::new();

    for row_i in 0..g.len() {
        for col_i in 0..g.first().unwrap().len() {
            if g[row_i][col_i] == 'X' {
                stack.push((Vec2::new(row_i as i32, col_i as i32), 1, None));
            }
        }
    }

    while let Some(curr) = stack.pop() {
        for neighbour in curr
            .0
            .neighbours_8_ranged(0..g.len() as i32, 0..g.first().unwrap().len() as i32)
        {
            if let Some(direction) = curr.2 {
                if neighbour - curr.0 != direction {
                    continue;
                }
            }

            if g[neighbour.row()][neighbour.col()] == word.chars().nth(curr.1).unwrap() {
                if curr.1 + 1 == word.len() {
                    sum += 1;
                    break;
                }
                stack.push((neighbour, curr.1 + 1, Some(neighbour - curr.0)));
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
        assert_eq!(solve(ti), "18".to_string());
    }
}
