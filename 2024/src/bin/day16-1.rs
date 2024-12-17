use common::{datastructs::vec2::Vec2, graphs::priority::Priority, strings::string_to_char_grid};
use std::{
    collections::{BinaryHeap, HashMap},
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
    for row in 0..m.len() {
        for col in 0..m[0].len() {
            if m[row][col] == 'S' {
                start_p = Some(Vec2::from_row_col(row, col));
            }
        }
    }
    let start_p = start_p.unwrap();

    let mut heap = BinaryHeap::new();
    let mut distance = HashMap::new();

    heap.push(Priority {
        value: 0,
        data: (start_p, Vec2::ARR_RIGHT),
    });

    let mut min_score = 0;
    while let Some(Priority { value, data }) = heap.pop() {
        let ent = distance.entry(data).or_insert(i32::MAX);

        if *data.0.i_arr(&m) == 'E' {
            min_score = value;
            break;
        }

        // If better found
        if *ent <= value {
            continue;
        }
        *ent = value;

        let next_p = data.0 + data.1;
        if (0..m.len() as i32).contains(&next_p.x)
            && (0..m[0].len() as i32).contains(&next_p.y)
            && *next_p.i_arr(&m) != '#'
        {
            heap.push(Priority {
                value: value + 1,
                data: (data.0 + data.1, data.1),
            });
        }
        heap.push(Priority {
            value: value + 1000,
            data: (data.0, data.1.arr_rot_90_clockwise()),
        });
        heap.push(Priority {
            value: value + 1000,
            data: (data.0, data.1.arr_rot_90_counter_clockwise()),
        });
    }

    min_score.to_string()
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
        assert_eq!(solve(ti), "7036".to_string());
    }
}
