use common::{datastructs::vec2::Vec2, strings::string_to_single_int_grid};

fn main() {
    dbg!(solve(include_str!("../../inputs/day10.txt")));
}

fn solve(input: &str) -> String {
    let m = string_to_single_int_grid(input);

    let mut s = Vec::new();
    for r in 0..m.len() {
        for c in 0..m[0].len() {
            if m[r][c] == 0 {
                s.push(Vec2::from_row_col(r, c));
            }
        }
    }

    let mut sum = 0;
    for start in s {
        let mut tops = 0;
        let mut stack = Vec::new();

        stack.push(start);

        while let Some(curr) = stack.pop() {
            if m[curr.row()][curr.col()] == 9 {
                tops += 1;
                continue;
            }
            for n in curr.neighbours_4_ranged(0..m.len() as i32, 0..m[0].len() as i32) {
                if m[curr.row()][curr.col()] + 1 == m[n.row()][n.col()] {
                    stack.push(n);
                }
            }
        }
        sum += tops;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_3() {
        let ti = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(solve(ti), "81".to_string());
    }
}
