use common::strings::string_to_extracted_nums_t_vec;
use core::panic;
use std::{ops::BitXor, time::Instant};
// For number types

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day17.txt")));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let [_a, b, c] = string_to_extracted_nums_t_vec::<i64>(registers)
        .try_into()
        .unwrap();
    let program = string_to_extracted_nums_t_vec::<u8>(program.split_once(": ").unwrap().1);

    let mut possibilities = vec![7 << ((program.len() - 1) * 3)];
    let mut i = 2;
    loop {
        let mut new_possibilities = Vec::new();
        for p in possibilities {
            for t in 0..8 {
                let test_val = (t << ((program.len() - i) * 3)) + p;
                let correct = get_correct_back(&program, test_val, b, c);
                if correct >= i {
                    new_possibilities.push(test_val);
                    if correct == program.len() {
                        return test_val.to_string();
                    }
                }
            }
        }

        i += 1;
        possibilities = new_possibilities;
        if i > program.len() {
            break;
        }
    }

    todo!();
}

fn get_correct_back(program: &[u8], mut a: i64, mut b: i64, mut c: i64) -> usize {
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

        if ip >= program.len() - 1 {
            break;
        }
    }

    let correct = sout
        .iter()
        .zip(program.iter())
        .rev()
        .take_while(|(&a, &b)| a == b as i64)
        .count();

    correct
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
