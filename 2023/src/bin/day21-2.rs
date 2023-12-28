use std::{collections::HashSet, f64};

use common::utils;
use glam::IVec2;
use ndarray::{arr1, Array2};
use ndarray_linalg::Inverse;

fn main() {
    dbg!(solve(include_str!("../../inputs/day21.txt"), 26501365));

    //         621289980339486 Too High
    //         621289922886178 Too High
    //         621289922885837 : ?
    // 10, 3 : 621289922898471: ?
    // 20, 5 : 621289980342474
    // 25, 5 : 621289980344769
    // 35, 3 : 621289980353529
    // 45, 5 : 621289980281748
    // 10, 5 : 621289922886372
}

const MOVES: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

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
    let start_i = 0;
    let sample_points = 3;

    println!(
        "Collecting {} sample points after {} gardens",
        sample_points, start_i
    );

    let mut y_p = vec![];
    let mut x_p = vec![];
    let mut samples = 0;

    let start_visited: HashSet<IVec2> = if steps % 2 == 0 {
        HashSet::from([start])
    } else {
        HashSet::from_iter(
            MOVES
                .iter()
                .map(|d| *d + start)
                .filter(|p| map[p.x as usize][p.y as usize] != '#'),
        )
    };

    let mut old_current_visited = HashSet::new();
    let mut current_visited = start_visited.clone();
    let mut reached = current_visited.len();

    let mut step = steps % 2;
    for _ in 0..(steps / 2) {
        let first = get_new_visited(&map, size, &current_visited);
        let second = get_new_visited(&map, size, &first);

        step += 2;
        let next_current_visited = second
            .difference(&current_visited)
            .map(|v| v.to_owned())
            .collect::<HashSet<IVec2>>()
            .difference(&old_current_visited)
            .map(|v| v.to_owned())
            .collect::<HashSet<IVec2>>();

        reached += next_current_visited.len();
        old_current_visited = current_visited;
        current_visited = next_current_visited;

        //println!("Step: {}, Reached: {}", step, reached);
        if step >= start_i * size && step % size == rem {
            y_p.push(reached as f64);
            x_p.push(step as f64);
            samples += 1;
            println!("Collected {}. sample. x: {}, y: {}", samples, reached, step);
        }

        if samples >= sample_points {
            break;
        }
    }

    if x_p.len() < 3 {
        return reached.to_string();
    }

    for (x, y) in x_p.iter().zip(y_p.iter()) {
        println!("{}\t{}", x, y);
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

fn get_new_visited(map: &Vec<Vec<char>>, size: usize, current: &HashSet<IVec2>) -> HashSet<IVec2> {
    let mut new = HashSet::new();
    for p in current {
        for mov in MOVES {
            let new_p = *p + mov;
            if map[new_p.x.rem_euclid(size as i32) as usize]
                [new_p.y.rem_euclid(size as i32) as usize]
                == '#'
            {
                continue;
            }

            new.insert(new_p);
        }
    }
    new
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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
