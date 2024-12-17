use common::{datastructs::vec2::Vec2, graphs::priority::Priority, strings::string_to_char_grid};
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    time::Instant,
};

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day16.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let m = string_to_char_grid(input);

    let mut start_p = None;
    let mut end_p = None;
    for row in 0..m.len() {
        for col in 0..m[0].len() {
            match m[row][col] {
                'S' => {
                    start_p = Some(Vec2::from_row_col(row, col));
                }

                'E' => end_p = Some(Vec2::from_row_col(row, col)),
                _ => (),
            }
        }
    }
    let start_p = start_p.unwrap();
    let end_p = end_p.unwrap();

    let mut heap = BinaryHeap::new();
    let mut distance = HashMap::new();

    heap.push(Priority {
        value: 0,
        data: (start_p, Vec2::ARR_RIGHT, Vec::new()),
    });

    let mut min_score = i32::MAX;
    let mut paths: HashSet<Vec2> = HashSet::new();
    while let Some(Priority { value, data }) = heap.pop() {
        if value > min_score {
            break;
        }

        let ent = distance.entry((data.0, data.1)).or_insert(i32::MAX);
        // If better found
        if *ent < value {
            continue;
        }
        *ent = value;

        let mut path = data.2;
        path.push(data.0);

        if data.0 == end_p {
            min_score = value;
            paths.extend(path);
            continue;
        }

        let next_p = data.0 + data.1;
        if (0..m.len() as i32).contains(&next_p.x)
            && (0..m[0].len() as i32).contains(&next_p.y)
            && *next_p.i_arr(&m) != '#'
        {
            heap.push(Priority {
                value: value + 1,
                data: (data.0 + data.1, data.1, path.clone()),
            });
        }
        heap.push(Priority {
            value: value + 1000,
            data: (data.0, data.1.arr_rot_90_clockwise(), path.clone()),
        });
        heap.push(Priority {
            value: value + 1000,
            data: (data.0, data.1.arr_rot_90_counter_clockwise(), path.clone()),
        });
    }

    paths.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(solve(ti), "45".to_string());
    }

    #[test]
    fn test_2() {
        let ti = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(solve(ti), "64".to_string());
    }
}
