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
    crt: Vec<char>,
}

impl Driver {
    fn new(queue: VecDeque<Instruction>) -> Self {
        Self {
            queue,
            register: 1,
            register_history: vec![1],
            crt: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.next_clock() {
            self.register_history.push(self.register);
        }
        self.register_history.push(self.register);
    }

    fn next_clock(&mut self) -> bool {
        let mut ret: bool = false;
        self.draw_to_screen();
        if let Some(instruction) = self.queue.front_mut() {
            instruction.time -= 1;
            if instruction.time <= 0 {
                self.register += instruction.add_value;
                self.queue.pop_front();
            }
            ret = true;
        }

        ret
    }

    fn draw_to_screen(&mut self) {
        println!("Sprite pos: {:?}", self.register - 1..self.register + 2);
        println!("CRT draws pixel in position {}", self.crt.len());
        if (self.register - 1..self.register + 2).contains(&((self.crt.len() as i32) % 40)) {
            self.crt.push('#')
        } else {
            self.crt.push('.')
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
    println!(
        "{}",
        driver
            .crt
            .iter()
            .enumerate()
            .fold("".to_string(), |mut acc, x| {
                if x.0 % 40 == 0 {
                    acc.push('\n');
                }
                acc.push(*x.1);
                acc
            })
    );
}
