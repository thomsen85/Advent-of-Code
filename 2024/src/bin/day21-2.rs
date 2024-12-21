use common::{datastructs::vec2::Vec2, strings::string_to_extracted_nums_t_vec};
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day21.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Keypad {
    Directional,
    Numeric,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Button {
    Num(i32),
    Dir(Vec2),
    Empty,
    A,
}

use Button::*;
const NUMERIC_KEYPAD: [[Button; 3]; 4] = [
    [Num(7), Num(8), Num(9)],
    [Num(4), Num(5), Num(6)],
    [Num(1), Num(2), Num(3)],
    [Empty, Num(0), A],
];

const DIRECTIONAL_KEYPAD: [[Button; 3]; 2] = [
    [Empty, Dir(Vec2::ARR_UP), A],
    [
        Dir(Vec2::ARR_LEFT),
        Dir(Vec2::ARR_DOWN),
        Dir(Vec2::ARR_RIGHT),
    ],
];

fn solve(input: &str) -> String {
    let sequences = dbg!(input.lines().collect_vec());
    let num_part = dbg!(input
        .lines()
        .map(string_to_extracted_nums_t_vec::<i32>)
        .collect_vec());

    let robot_arms = [
        (Vec2::from_row_col(3, 2), Keypad::Numeric), // Numeric
        (Vec2::from_row_col(0, 2), Keypad::Directional), //  First
        (Vec2::from_row_col(0, 2), Keypad::Directional), //  Second
    ];

    let mut sum = 0;
    let mut cache = HashMap::new();
    for (&seq, num) in sequences.iter().zip(num_part) {
        for (c1, c2) in ['A'].into_iter().chain(seq.chars()).tuple_windows() {
            sum += dfs(
                Keypad::Numeric,
                keypad_char_to_button(Keypad::Numeric, c1),
                keypad_char_to_button(Keypad::Numeric, c2),
                25,
                &mut cache,
            ) * num[0] as usize;
        }
    }
    sum.to_string()
}

fn keypad_char_to_button(keypad: Keypad, c: char) -> Button {
    if c == 'A' {
        return Button::A;
    }

    match keypad {
        Keypad::Directional => Button::Dir(match c {
            '^' => Vec2::ARR_UP,
            'v' => Vec2::ARR_DOWN,
            '<' => Vec2::ARR_LEFT,
            '>' => Vec2::ARR_RIGHT,
            _ => panic!("{} not valid char", c),
        }),
        Keypad::Numeric => {
            Button::Num(c.to_digit(10).expect("Invalid char to digit converstion") as i32)
        }
    }
}

fn find_button(keypad: Keypad, button: Button) -> Vec2 {
    match keypad {
        Keypad::Directional => {
            for row in 0..DIRECTIONAL_KEYPAD.len() {
                for col in 0..DIRECTIONAL_KEYPAD[0].len() {
                    if DIRECTIONAL_KEYPAD[row][col] == button {
                        return Vec2::from_row_col(row, col);
                    }
                }
            }
        }
        Keypad::Numeric => {
            for row in 0..NUMERIC_KEYPAD.len() {
                for col in 0..NUMERIC_KEYPAD[0].len() {
                    if NUMERIC_KEYPAD[row][col] == button {
                        return Vec2::from_row_col(row, col);
                    }
                }
            }
        }
    }
    todo!()
}

fn dfs(
    keypad: Keypad,
    from: Button,
    to: Button,
    depth: u32,
    mem: &mut HashMap<(Button, Button, u32), usize>,
) -> usize {
    if let Some(&res) = mem.get(&(from, to, depth)) {
        return res;
    }

    let from_pos = find_button(keypad, from);

    let bounds = match keypad {
        Keypad::Directional => (DIRECTIONAL_KEYPAD.len(), DIRECTIONAL_KEYPAD[0].len()),
        Keypad::Numeric => (NUMERIC_KEYPAD.len(), NUMERIC_KEYPAD[0].len()),
    };

    let mut queue = VecDeque::from([(from_pos, Vec::new())]);
    let mut valid_paths: Vec<Vec<Button>> = Vec::new();

    while let Some((curr_p, path)) = queue.pop_front() {
        if match keypad {
            Keypad::Directional => &DIRECTIONAL_KEYPAD[curr_p.row()][curr_p.col()],
            Keypad::Numeric => &NUMERIC_KEYPAD[curr_p.row()][curr_p.col()],
        } == &to
        {
            // Add an A to the end to apply it :)
            let mut path = path;
            path.push(Button::A);

            if let Some(prev) = valid_paths.last() {
                if prev.len() == path.len() {
                    valid_paths.push(path);
                } else {
                    break;
                }
            } else {
                valid_paths.push(path);
            }
            continue;
        }

        for d_p in [
            Vec2::ARR_RIGHT,
            Vec2::ARR_UP,
            Vec2::ARR_DOWN,
            Vec2::ARR_LEFT,
        ] {
            let n_p = curr_p + d_p;
            if !(0..bounds.0 as i32).contains(&n_p.x) || !(0..bounds.1 as i32).contains(&n_p.y) {
                continue;
            }

            // Dont traverse empty spaces
            if match keypad {
                Keypad::Directional => &DIRECTIONAL_KEYPAD[curr_p.row()][curr_p.col()],
                Keypad::Numeric => &NUMERIC_KEYPAD[curr_p.row()][curr_p.col()],
            } == &Empty
            {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(Button::Dir(d_p));

            queue.push_back((n_p, new_path))
        }
    }

    let min_route_len = valid_paths.iter().map(|a| a.len()).min().unwrap();

    if depth == 0 {
        return min_route_len;
    }

    // let min_routes = valid_paths.into_iter().filter(|a| a.len() == min_route_len);

    let next_keypad = Keypad::Directional; // Next is alwasys Directional til the end
    let optimal = valid_paths
        .iter()
        .map(|route| {
            [Button::A]
                .iter()
                .chain(route) // New depth always starts on A
                .tuple_windows()
                .map(|(&new_from, &new_to)| dfs(next_keypad, new_from, new_to, depth - 1, mem))
                .sum()
        })
        .min()
        .unwrap();

    mem.insert((from, to, depth), optimal);

    optimal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "029A
980A
179A
456A
379A";
        assert_eq!(solve(ti), "126384".to_string());
    }

    #[test]
    fn test_dfs_1() {
        let mut cache = HashMap::new();
        let depth = 0;
        assert_eq!(
            2,
            dfs(
                Keypad::Numeric,
                Button::A,
                Button::Num(0),
                depth,
                &mut cache
            )
        );
    }
    #[test]
    fn test_dfs_2() {
        let mut cache = HashMap::new();
        let depth = 1;
        assert_eq!(
            8,
            dfs(
                Keypad::Numeric,
                Button::A,
                Button::Num(0),
                depth,
                &mut cache
            )
        );
    }
    #[test]
    fn test_dfs_1_longer() {
        let seq = "A029A";
        let mut cache = HashMap::new();
        let depth = 0;
        assert_eq!(
            12,
            seq.chars()
                .map(|c| keypad_char_to_button(Keypad::Numeric, c))
                .tuple_windows()
                .map(|(a, b)| dfs(Keypad::Numeric, a, b, depth, &mut cache))
                .sum::<usize>()
        );
    }
    #[test]
    fn test_dfs_seq_2() {
        let seq = "A029A";
        let mut cache = HashMap::new();
        let depth = 1;
        assert_eq!(
            28,
            seq.chars()
                .map(|c| keypad_char_to_button(Keypad::Numeric, c))
                .tuple_windows()
                .map(|(a, b)| dfs(Keypad::Numeric, a, b, depth, &mut cache))
                .sum::<usize>()
        );
    }
}
