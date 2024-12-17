use common::strings::string_to_extracted_nums_t_vec;
use core::panic;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
use num::{pow::Pow, PrimInt};
use std::{
    ops::BitXor,
    time::{Duration, Instant},
};
// For number types
use nom::character::complete as cnom;

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day17.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

enum Constraint {
    GreaterThan(i64),
    LessThan(i64),
}

struct Variable {
    name: String,
}

struct ConstraintSolver {
    constrains: Vec<(Variable, Constraint)>,
}
impl ConstraintSolver {
    fn new() -> Self {
        Self {
            constrains: todo!(),
        }
    }

    fn add_variable(&self, arg: &str) {
        todo!()
    }

    fn add_constraint(&self, min: (&str, Constraint)) -> _ {
        todo!()
    }
}

fn solve(input: &str) -> String {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let [mut a, mut b, mut c] = string_to_extracted_nums_t_vec::<i64>(registers)
        .try_into()
        .unwrap();
    let program = string_to_extracted_nums_t_vec::<u8>(program.split_once(": ").unwrap().1);

    dbg!(a, b, c);
    dbg!(&program);

    // constrains solving is probablly the best approch
    // propegation over backtracking because of the size of the problem
    // first easy constrain is this one: min < a < max
    let min = 8i64.pow(program.len() as u32 - 1);
    let max = 8i64.pow(program.len() as u32) - 1;

    let cs = ConstraintSolver::new();
    cs.add_variable("a");
    cs.add_constraint(("a", Constraint::GreaterThan(min)));
    cs.add_constraint(("a", Constraint::LessThan(max)));
    dbg!(min);
    dbg!(max);

    let diff = dbg!(max - min);

    let mut pl = true;
    let mut left = min;
    let mut right = max;

    let mut last_left = None;

    loop {
        a = if pl { left } else { right };
        dbg!(a);
        let mut restart = false;
        let mut sout_p = 0;
        let mut sout = Vec::new();
        let mut ip = 0;
        loop {
            let instruction = program[ip];
            let operand = program[ip + 1];
            let combo_operand = match operand {
                0..=3 => operand as i64,
                4 => a,
                5 => b,
                6 => c,
                7 => 0,
                _ => panic!("{}", operand),
            };
            match instruction {
                0 => {
                    a /= 2i64.pow(combo_operand as u32);
                }
                1 => {
                    b = b.bitxor(operand as i64);
                }
                2 => {
                    b = combo_operand % 8;
                }
                3 => {
                    // jump
                    if a != 0 {
                        ip = operand as usize;
                        continue;
                    }
                }
                4 => {
                    b = b.bitxor(c);
                }
                5 => sout.push(combo_operand % 8),
                6 => {
                    b = a / 2i64.pow(combo_operand as u32);
                }
                7 => {
                    c = a / 2i64.pow(combo_operand as u32);
                }

                _ => panic!("{} not yet planed for", instruction),
            };

            ip += 2;
            // dbg!(ip);
            // dbg!(a);

            if ip >= program.len() - 1 {
                if sout_p != program.len() {
                    restart = true;
                }
                break;
            }
        }

        let l = sout.iter().join(",");
        let l_target = program.iter().join(",");
        assert_eq!(l.len(), l_target.len());
        let adiff = l
            .chars()
            .zip(l_target.chars())
            .map(|(a, b)| if a == b && a != ',' { '_' } else { a })
            .collect::<String>();
        let bdiff = l
            .chars()
            .zip(l_target.chars())
            .map(|(a, b)| if a == b && a != ',' { '_' } else { b })
            .collect::<String>();

        let correct = sout
            .iter()
            .zip(program.iter())
            .rev()
            .take_while(|(&a, &b)| a == b as i64)
            .count();
        let first_incorrect_diff = sout
            .iter()
            .zip(program.iter())
            .rev()
            .find(|(&a, &b)| a != b as i64)
            .map(|(a, b)| a.abs_diff(*b as i64))
            .unwrap();

        let score = 8i64.pow(correct as u32) - first_incorrect_diff as i64;

        if pl {
            last_left = Some(score);
            pl = false;
        } else {
            let l_val = last_left.unwrap();

            // if the left score is better, keep that
            if l_val > score {
                right = (left + right) / 2;
            } else {
                left = (left + right) / 2;
            }
            pl = true;
            last_left = None;
            dbg!(left, right);
            // dbg!(i - min);
            dbg!(adiff);
            dbg!(bdiff);
            dbg!(correct);
            std::thread::sleep(Duration::from_millis(50));
        }

        if restart {
            continue;
        } else {
            break;
        }
    }

    " ".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(solve(ti), "117440".to_string());
    }
}
