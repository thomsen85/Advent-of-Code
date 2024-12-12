use std::{collections::HashSet, time::Instant};

use common::{datastructs::vec2::Vec2, strings::string_to_char_grid};

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day12.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let m = string_to_char_grid(input);
    let mut visited = vec![vec![0; m[1].len()]; m.len()];
    let mut s_id = 0;
    let mut shapes: Vec<Vec<Vec2>> = Vec::new();

    let mut stack = Vec::new();
    let start = Vec2::new(0, 0);
    stack.push((start, *start.i_arr(&m)));
    loop {
        shapes.push(vec![]);
        dbg!(&stack);
        while let Some((curr, curr_id)) = stack.pop() {
            if *curr.i_arr(&visited) == 1 {
                continue;
            }
            *curr.i_arr_mut(&mut visited) = 1;
            shapes[s_id].push(curr);

            for n in curr.neighbours_4_ranged(0..m.len() as i32, 0..m[0].len() as i32) {
                if *n.i_arr(&m) == curr_id {
                    stack.push((n, curr_id));
                }
            }
        }

        // Find next not visited squre
        s_id += 1;

        for r in 0..m.len() {
            for c in 0..m[0].len() {
                if visited[r][c] == 0 {
                    stack.push((Vec2::from_row_col(r, c), m[r][c]));
                    break;
                }
            }
            if !stack.is_empty() {
                break;
            }
        }

        if stack.is_empty() {
            break;
        }
    }

    //
    let area = shapes.iter().map(|shape| shape.len());
    //
    let sides = shapes.iter().map(|shape| {
        let shape_points = shape.iter().copied().collect::<HashSet<_>>();
        let mut amnt_sides = 0;

        let mut sides_visited = HashSet::new();
        for &p in shape {
            let valid_sides = p
                .neighbours_4_ranged(-1..=m.len() as i32, -1..=m[0].len() as i32)
                .into_iter()
                .filter(|n| !shape_points.contains(n));

            for side in valid_sides {
                let angle = side - p;

                if sides_visited.contains(&(side, angle)) {
                    continue;
                }
                sides_visited.insert((side, angle));
                amnt_sides += 1;

                let mut stack = vec![
                    (
                        side + angle.arr_rot_90_clockwise(),
                        angle.arr_rot_90_clockwise(),
                    ),
                    (
                        side + angle.arr_rot_90_counter_clockwise(),
                        angle.arr_rot_90_counter_clockwise(),
                    ),
                ];

                while let Some((p, direction)) = stack.pop() {
                    if shape_points.contains(&p) || !shape_points.contains(&(p - angle)) {
                        continue;
                    }
                    sides_visited.insert((p, angle));

                    stack.push((p + direction, direction))
                }
            }
        }

        amnt_sides
    });

    area.zip(sides)
        .map(|(a, b)| dbg!(a) as i32 * dbg!(b))
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(solve(ti), "80".to_string());
    }

    #[test]
    fn test_2() {
        let ti = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(solve(ti), "436".to_string());
    }

    #[test]
    fn test_3() {
        let ti = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(solve(ti), "236".to_string());
    }

    #[test]
    fn test_4() {
        let ti = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(solve(ti), "368".to_string());
    }

    #[test]
    fn test_5() {
        let ti = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(solve(ti), "1206".to_string());
    }
}
