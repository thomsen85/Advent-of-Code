use rstest::rstest;
use std::{collections::HashSet, f64};

use common::utils;
use glam::IVec2;
use ndarray::{arr1, Array2};
use ndarray_linalg::Inverse;

fn main() {
    dbg!(solve(include_str!("../../inputs/day21.txt"), 26501365));

    // 621289980339486 Too High
    // 621289922886178 Too High
    // 621289922885837 : ?
    // 10 , 3 : 621289922898471: ?
}

fn solve(input: &str, steps: usize) -> String {
    println!("Solving for {} steps", steps);
    let map = utils::string_to_grid(input);

    let start = map
        .iter()
        .enumerate()
        .flat_map(|(r, line)| {
            line.iter()
                .enumerate()
                .map(move |(col, c)| (IVec2::new(r as i32, col as i32), c))
        })
        .find(|(_, c)| **c == 'S')
        .unwrap()
        .0;

    // Assert that it is square
    {
        let rows = map.len();
        let columns = map[0].len();
        assert_eq!(rows, columns);
    }
    let size = map.len();
    println!("Map is of size {}x{}", size, size);

    let rem = steps % size;
    let start_i = 10;
    let sample_points = 3;

    println!(
        "Collecting {} sample points after {} gardens",
        sample_points, start_i
    );

    let mut visited: HashSet<IVec2> = HashSet::from([start]);
    let mut y_p = vec![];
    let mut x_p = vec![];
    let mut samples = 0;

    const MOVES: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];
    for step in 0..steps {
        let visited_c = visited.clone();
        visited.clear();

        for p in visited_c {
            for mov in MOVES {
                let new_p = p + mov;
                if map[new_p.x.rem_euclid(size as i32) as usize]
                    [new_p.y.rem_euclid(size as i32) as usize]
                    == '#'
                {
                    continue;
                }

                visited.insert(new_p);
            }
        }

        if step >= start_i * size && (step + 1) % size == rem {
            y_p.push(visited.len() as f64);
            x_p.push(1. + step as f64);
            samples += 1;
            println!("Collected {}. sample", samples);
        }

        if samples >= sample_points {
            break;
        }
    }

    if x_p.len() < 3 {
        return visited.len().to_string();
    }
    // https://en.wikipedia.org/wiki/Polynomial_regression
    // 1 x_n (x^2)_n
    // n = x_p.len()
    // 4, 2 array

    let n = x_p.len();
    let m: usize = 3;
    let mut x: Array2<f64> = Array2::ones((n, m));
    for (i, val) in x_p.iter().enumerate() {
        x[[i, 1]] = *val;
        x[[i, 2]] = val.powi(2);
    }
    println!("x:\n{}", &x);

    // 1, 4
    let y = arr1(&y_p);
    let y = y.to_shape((n, 1)).unwrap().to_owned();
    println!("y:\n{}", &y);

    let b = x
        .t()
        .to_owned()
        .dot(&x.clone())
        .inv()
        .unwrap()
        .dot(&x.clone().t().dot(&y));
    let e = y - x.dot(&b);

    println!("e:\n{}", &e);

    dbg!(steps);
    let ans = dbg!(arr1(&[1., steps as f64, steps.pow(2) as f64]).dot(&b))[0];

    (ans.round() as usize).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(6, "16")]
    #[case(10, "50")]
    #[case(50, "1594")]
    #[case(100, "6536")]
    #[case(500, "167004")]
    #[case(1000, "668697")]
    #[case(5000, "16733044")]

    fn test_1(#[case] steps: usize, #[case] count: &str) {
        let ti = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(solve(ti, steps), count.to_string());
    }
}
