use common::{datastructs::vec2::Vec2, strings::string_to_extracted_nums_t_vec};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeBounds,
    time::Instant,
};
// For number types
use nom::character::complete as cnom;

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day21.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

#[derive(Debug, Clone)]
enum Keypad {
    Directional,
    Numeric,
}

#[derive(PartialEq, Eq, Debug)]
enum Button {
    Num(i32),
    Dir(Vec2),
    Empty,
    A,
}

fn solve(input: &str) -> String {
    let sequences = dbg!(input.lines().collect_vec());
    let num_part = dbg!(input
        .lines()
        .map(string_to_extracted_nums_t_vec::<i32>)
        .collect_vec());

    //
    let robot_arms = [
        (Vec2::from_row_col(3, 2), Keypad::Numeric), // Numeric
        (Vec2::from_row_col(0, 2), Keypad::Directional), //  First
        (Vec2::from_row_col(0, 2), Keypad::Directional), //  Second
    ];

    use Button::*;
    let numeric_keypad = [
        [Num(7), Num(8), Num(9)],
        [Num(4), Num(5), Num(6)],
        [Num(1), Num(2), Num(3)],
        [Empty, Num(0), A],
    ];

    let directional_keypad = [
        [Empty, Dir(Vec2::ARR_UP), A],
        [
            Dir(Vec2::ARR_LEFT),
            Dir(Vec2::ARR_DOWN),
            Dir(Vec2::ARR_RIGHT),
        ],
    ];

    let mut mem: HashMap<(Keypad, Vec2, char), String> = HashMap::new();

    let mut sum = 0;
    for (&seq, num) in sequences.iter().zip(num_part) {
        let mut presses = String::new();
        let mut needed_seq = seq.to_owned();

        let mut robot_arms_c = robot_arms.clone();
        for robot_arm in robot_arms_c.iter_mut() {
            for needed_char in needed_seq.chars()
            // .zip(vec!['A'; needed_seq.len()])
            // .flat_map(|(a, b)| [a, b])
            {
                let mut q = VecDeque::from([(robot_arm.0, String::new())]);
                // let mut visited = HashSet::new();

                // let map = match robot_arm.1 {
                //     Keypad::Directional => &directional_keypad,
                //     Keypad::Numeric => &numeric_keypad,
                // };

                let mut target = match robot_arm.1 {
                    Keypad::Directional => {
                        if needed_char == 'A' {
                            Button::A
                        } else {
                            Button::Dir(match needed_char {
                                '^' => Vec2::ARR_UP,
                                'v' => Vec2::ARR_DOWN,
                                '<' => Vec2::ARR_LEFT,
                                '>' => Vec2::ARR_RIGHT,
                                _ => panic!("{} not valid char", needed_char),
                            })
                        }
                    }
                    Keypad::Numeric => {
                        if needed_char == 'A' {
                            Button::A
                        } else {
                            Button::Num(
                                needed_char
                                    .to_digit(10)
                                    .expect("Invalid char to digit converstion")
                                    as i32,
                            )
                        }
                    }
                };

                let bounds = match robot_arm.1 {
                    Keypad::Directional => (directional_keypad.len(), directional_keypad[0].len()),
                    Keypad::Numeric => (numeric_keypad.len(), numeric_keypad[0].len()),
                };

                while let Some((curr_p, path)) = q.pop_front() {
                    // if visited.contains(&curr_p) {
                    //     continue;
                    // }

                    // check if is at place to be
                    if match robot_arm.1 {
                        Keypad::Directional => &directional_keypad[curr_p.row()][curr_p.col()],
                        Keypad::Numeric => &numeric_keypad[curr_p.row()][curr_p.col()],
                    } == &target
                    {
                        presses.push_str(&path);
                        presses.push('A');
                        robot_arm.0 = curr_p;
                        break;
                    }

                    // Order matters it seems like
                    // for n_p in curr_p.neighbours_4_ranged(0..bounds.0 as i32, 0..bounds.1 as i32) {
                    for d_p in [
                        Vec2::ARR_RIGHT,
                        Vec2::ARR_UP,
                        Vec2::ARR_DOWN,
                        Vec2::ARR_LEFT,
                    ] {
                        let n_p = curr_p + d_p;
                        if !(0..bounds.0 as i32).contains(&n_p.x)
                            || !(0..bounds.1 as i32).contains(&n_p.y)
                        {
                            continue;
                        }
                        if match robot_arm.1 {
                            Keypad::Directional => &directional_keypad[curr_p.row()][curr_p.col()],
                            Keypad::Numeric => &numeric_keypad[curr_p.row()][curr_p.col()],
                        } == &Empty
                        {
                            continue;
                        }

                        let mut new_path = path.clone();
                        new_path.push(match n_p - curr_p {
                            Vec2::ARR_UP => '^',
                            Vec2::ARR_DOWN => 'v',
                            Vec2::ARR_LEFT => '<',
                            Vec2::ARR_RIGHT => '>',
                            _ => panic!(),
                        });
                        q.push_back((n_p, new_path))
                    }
                }
            }

            println!("Seq: {}, typed with {}", needed_seq, presses);
            needed_seq = presses.clone();
            presses.clear();
        }
        sum += dbg!(needed_seq.len()) * dbg!(*num.first().unwrap() as usize);
    }
    sum.to_string()
}

fn parse(input: &str) -> IResult<&str, ()> {
    Ok((input, ()))
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
}
