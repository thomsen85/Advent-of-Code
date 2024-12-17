use common::strings::string_to_extracted_nums_t_vec;
use core::panic;
use itertools::Itertools;
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
    let [mut a, mut b, mut c] = string_to_extracted_nums_t_vec::<i64>(registers)
        .try_into()
        .unwrap();
    let program = string_to_extracted_nums_t_vec::<u8>(program.split_once(": ").unwrap().1);

    dbg!(a, b, c);
    dbg!(&program);

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
            5 => {
                sout.push(combo_operand % 8);
            }
            6 => {
                b = a / 2i64.pow(combo_operand as u32);
            }
            7 => {
                c = a / 2i64.pow(combo_operand as u32);
            }

            _ => panic!("{} not yet planed for", instruction),
        };

        ip += 2;
        dbg!(ip);
        dbg!(a);
        if ip >= program.len() - 1 {
            break;
        }
    }

    sout.iter().join(",").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(solve(ti), "4,6,3,5,6,3,5,2,1,0".to_string());
    }
}
