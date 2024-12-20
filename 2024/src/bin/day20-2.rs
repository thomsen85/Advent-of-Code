use common::{datastructs::vec2::Vec2, strings::string_to_char_grid};
use itertools::Itertools;
use num::Signed;
use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day20.txt"), false));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str, test: bool) -> String {
    let m = string_to_char_grid(input);

    let mut start = None;
    let mut end = None;

    for row in 0..m.len() {
        for col in 0..m[0].len() {
            match m[row][col] {
                'S' => start = Some(Vec2::from_row_col(row, col)),
                'E' => end = Some(Vec2::from_row_col(row, col)),
                _ => (),
            }
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();

    let mut q = VecDeque::from([(0, start)]);
    let mut visited = HashMap::new();

    while let Some((ct, cp)) = q.pop_front() {
        if cp == end {
            println!("End found at time {}", ct);
            visited.insert(cp, ct);
            break;
        }

        if visited.contains_key(&cp) {
            continue;
        }
        visited.insert(cp, ct);

        for np in cp.neighbours_4_ranged(0..m.len() as i32, 0..m[0].len() as i32) {
            if *np.i_arr(&m) == '#' {
                continue;
            }

            q.push_back((ct + 1, np));
        }
    }

    let mut sum = 0;
    for ((p1, t1), (p2, t2)) in visited.iter().tuple_combinations() {
        let dist = *p2 - *p1;
        let dist_b = dist.x.abs() + dist.y.abs();
        if dist_b > 20 {
            continue;
        }

        let saves = (t2 - t1).abs() - dist_b;
        if saves >= 100 {
            sum += 1;
        }
        if test && saves > 1 {
            sum += 1;
            println!("Found save of {}", saves);
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!(
            solve(ti, true),
            (14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1).to_string()
        );
    }
}
