use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, other: &Self) -> f64 {
        ((self.x - other.x).pow(2) as f64 + (self.y - other.y).pow(2) as f64).sqrt()
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Debug)]
struct Rope {
    rope: Vec<Vec2>,
    history: HashSet<Vec2>,
}

impl Rope {
    fn new(length: u32) -> Self {
        let mut history = HashSet::new();
        let mut rope = Vec::new();
        for _ in 0..length {
            rope.push(Vec2::new(0, 0));
        }
        history.insert(Vec2::new(0, 0));
        Self { rope, history }
    }

    fn mov(&mut self, mov: (&str, i32)) {
        for _ in 0..mov.1 {
            match mov.0 {
                "U" => self.rope[0].y += 1,
                "D" => self.rope[0].y -= 1,
                "R" => self.rope[0].x += 1,
                "L" => self.rope[0].x -= 1,
                _ => panic!("Invalid input"),
            }

            for i in 1..self.rope.len() {
                if self.rope[i - 1].distance_to(&self.rope[i]) > 1.5 {
                    self._mov_t_to_h(i - 1, i);
                }
                // print!("{}[2J", 27 as char);
                // println!("{}", self);
                // println!("-------------------------------");
                // sleep(Duration::from_millis(10));
            }
        }
    }

    fn _mov_t_to_h(&mut self, head: usize, tail: usize) {
        let mut closest_mov = Vec2::new(0, 0);
        let mut closest_dist = f64::MAX;
        for x in -1..=1 {
            for y in -1..=1 {
                if x != 0 || y != 0 {
                    let mov = Vec2::new(x, y);
                    let dist = self.rope[head].distance_to(&self.rope[tail].add(&mov));
                    if dist < closest_dist {
                        closest_dist = dist;
                        closest_mov = mov;
                    }
                }
            }
        }
        self.rope[tail] = self.rope[tail].add(&closest_mov);
        if tail == self.rope.len() - 1 {
            self.history.insert(self.rope[tail]);
            // println!("{:?}", self.rope[tail])
        }
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const size: i32 = 20;
        for y in -size..size {
            for x in -size..size {
                let mut empty = true;
                for (i, pos) in self.rope.iter().enumerate() {
                    if pos == &Vec2::new(x, -y) {
                        empty = false;
                        f.write_str(&(i + 1).to_string())?;
                        break;
                    }
                }

                if empty {
                    for (_i, pos) in self.history.iter().enumerate() {
                        if pos == &Vec2::new(x, -y) {
                            empty = false;
                            f.write_char('#')?;
                            break;
                        }
                    }
                }
                if empty {
                    f.write_char(' ')?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn main() {
    let mut rope = Rope::new(10);

    common::utils::lines_from_file("inputs/day9.txt")
        .iter()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|o| (o[0], o[1].parse::<i32>().unwrap()))
        .for_each(|mov| rope.mov(mov));

    println!("{:?}", rope.history.len())
}
