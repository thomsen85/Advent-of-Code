use itertools::Itertools;

fn main() {
    dbg!(solve(include_str!("../../inputs/day11.txt")));
}

fn solve(input: &str) -> String {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    // Expand rows
    let mut row = 0;
    loop {
        if map[row].iter().all(|c| *c == '.') {
            row += 1;
            map.insert(row, vec!['.'; map[row].len()]);
        }

        row += 1;
        if row >= map.len() {
            break;
        }
    }

    let mut column = 0;
    loop {
        if map.iter().map(|line| line[column]).all(|c| c == '.') {
            column += 1;
            map.iter_mut().for_each(|a| a.insert(column, '.'));
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
            sum += dist(p1.0, p1.1, p2.0, p2.1);
        }
    }
    sum /= 2;

    sum.to_string()
}

fn dist(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let dx = x1.abs_diff(x2);
    let dy = y1.abs_diff(y2);

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
