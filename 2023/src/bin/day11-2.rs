use itertools::Itertools;

fn main() {
    dbg!(solve(include_str!("../../inputs/day11.txt")));
}

fn solve(input: &str) -> String {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let expantion = 1_000_000 - 1;
    let rows = map
        .iter()
        .enumerate()
        .filter_map(|(r, row)| {
            if row.iter().all(|c| *c == '.') {
                return Some(r);
            }
            None
        })
        .collect_vec();

    let mut column = 0;
    let mut columns = Vec::new();
    loop {
        if map.iter().map(|line| line[column]).all(|c| c == '.') {
            columns.push(column);
        }

        column += 1;
        if column >= map[0].len() {
            break;
        }
    }

    let points = map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect_vec();

    let mut sum = 0;
    for p1 in &points {
        for p2 in &points {
            let y_crossings = rows
                .iter()
                .filter(|r| {
                    let mn = p1.1.min(p2.1);
                    let mx = p1.1.max(p2.1);
                    (**r > mn) && (**r < mx)
                })
                .count();
            let x_crossing = columns
                .iter()
                .filter(|r| {
                    let mn = p1.0.min(p2.0);
                    let mx = p1.0.max(p2.0);
                    (**r > mn) && (**r < mx)
                })
                .count();

            sum += dist(
                p1.0,
                p1.1,
                p2.0,
                p2.1,
                x_crossing * expantion,
                y_crossings * expantion,
            );
        }
    }
    sum /= 2;

    sum.to_string()
}

fn dist(x1: usize, y1: usize, x2: usize, y2: usize, x_cross: usize, y_cross: usize) -> usize {
    let dx = x1.abs_diff(x2) + x_cross;
    let dy = y1.abs_diff(y2) + y_cross;

    let min = dx.min(dy);
    let max = dx.max(dy);

    let diagonal_steps = min;
    let staight_steps = max - min;

    diagonal_steps * 2 + staight_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(solve(ti), "374".to_string());
    }
}
