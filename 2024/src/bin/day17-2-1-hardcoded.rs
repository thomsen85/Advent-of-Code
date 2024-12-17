use common::strings::string_to_extracted_nums_t_vec;
use core::panic;
use itertools::Itertools;
use std::{
    ops::BitXor,
    time::{Duration, Instant},
};
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

    // constrains solving is probablly the best approch
    // propegation over backtracking because of the size of the problem
    // first easy constrain is this one: min < a < max
    let min = 8i64.pow(program.len() as u32 - 1);
    let max = 8i64.pow(program.len() as u32) - 1;

    dbg!(program.len() * 3);
    dbg!(min);
    dbg!(max);

    // solve_three_bit_part(0, 0);
    // solve_three_bit_part(3, 7);
    // solve_three_bit_part(3, 7);
    solve_three_bit_part(5, 58);

    let mut possibilities = Vec::new();
    possibilities.push(7 << ((program.len() - 1) * 3));
    let mut i = 2;
    loop {
        println!("Finding the {}.", i);
        let mut new_possibilities = Vec::new();
        dbg!(&possibilities);
        for p in possibilities {
            for t in 0..8 {
                println!("Trying {}", t);
                let test_val = (t << ((program.len() - i) * 3)) + p;
                assert!(test_val > min && test_val < max, "{:b}", test_val);
                dbg!(test_val);
                let correct = fun_name(&program, test_val, b, c);
                if correct >= i {
                    println!("{} gives corrects", t);
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
    // fun_name(&program, 58 << ((program.len() - 2) * 3), b, c);
    // acc, index
    let mut stack = vec![(0, program.len() as i64 - 1)];
    while let Some((acc, index)) = stack.pop() {
        dbg!(acc, index);
        if index <= 0 {
            assert!(acc > min && acc < max);
            dbg!(acc, index);
            dbg!(fun_name(&program, acc, b, c));
            continue;
        }

        let possibilities = solve_three_bit_part(program[index as usize] as i64, acc);

        for (a, b3, c1) in possibilities {
            stack.push((a, index - 1));
        }
    }

    // let each = program
    //     .iter()
    //     .rev()
    //     .map(|num| solve_three_bit_part(*num as i64, 0))
    //     .collect_vec();

    let diff = dbg!(max - min);

    // fun_name(&program, max, b, c);

    " ".to_string()
}

fn solve_three_bit_part(target: i64, comes_before: i64) -> Vec<(i64, i64, i64)> {
    let mut possibilities = Vec::new();
    for a2 in 0..8 {
        let a2 = a2 + (comes_before << 3);
        let b4 = a2 % 8;
        let b3 = b4 ^ 7;
        for c1 in 0..8 {
            // c1 = a / 2.pow(b3) => c1 = a >> b3
            if c1 != a2 / 2i64.pow(b3 as u32) {
                // This means that it doesnt alignt with c1
                // dbg!(c1, a / 2i64.pow(b3 as u32), a, b3, "Furfilled");
                continue;
            }
            let b2 = b3 ^ c1;
            let b1 = b2 ^ 7;

            if b1 == target {
                // What a can be if c3 is
                possibilities.push((a2, b3, c1));
            }
        }
    }
    dbg!(possibilities)
}

fn fun_name(program: &Vec<u8>, mut a: i64, mut b: i64, mut c: i64) -> usize {
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

    // dbg!(i - min);
    // println!("Actual: {}", l);
    // println!("Target: {}", l_target);
    // println!("===== diff====");
    dbg!(adiff);
    dbg!(bdiff);
    // dbg!(correct);
    // std::thread::sleep(Duration::from_millis(50));
    dbg!(correct)
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
