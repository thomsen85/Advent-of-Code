use std::collections::VecDeque;

struct Instruction {
    time: usize,
    add_value: i32,
}

impl Instruction {
    fn new(time: usize, add_value: i32) -> Self {
        Self { time, add_value }
    }
}

struct Driver {
    queue: VecDeque<Instruction>,
    register: i32,
    register_history: Vec<i32>,
}

impl Driver {
    fn new(queue: VecDeque<Instruction>) -> Self {
        Self {
            queue,
            register: 1,
            register_history: vec![1],
        }
    }

    fn run(&mut self) {
        while self.next_clock() {
            self.register_history.push(self.register);
        }
        self.register_history.push(self.register);
    }

    fn next_clock(&mut self) -> bool {
        if let Some(instruction) = self.queue.front_mut() {
            instruction.time -= 1;
            if instruction.time <= 0 {
                self.register += instruction.add_value;
                self.queue.pop_front();
            }
            true
        } else {
            false
        }
    }
}

fn main() {
    let input: VecDeque<Instruction> = aoc2022_rust::utils::lines_from_file("inputs/day10.txt")
        .iter()
        .map(|s| s.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|s| {
            if s[0] == "noop" {
                Instruction::new(1, 0)
            } else {
                Instruction::new(2, s[1].parse().unwrap())
            }
        })
        .collect();

    let mut driver = Driver::new(input);
    driver.run();
    //println!("{:?}", driver.register_history.iter().enumerate().map(|l| format!("{}-{}", l.0, l.1)).collect::<Vec<String>>());
    println!("Sum: {}", get_sum(&driver.register_history));
}

fn get_sum(l: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in (20..=220).step_by(40) {
        println!("{} * {}", i, l[i]);
        sum += l[i - 1] * i as i32;
    }
    sum
}
