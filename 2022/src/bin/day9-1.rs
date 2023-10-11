use std::collections::HashSet;

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
    h: Vec2,
    t: Vec2,
    history: HashSet<Vec2>,
}

impl Rope {
    fn new() -> Self {
        let mut history = HashSet::new();
        history.insert(Vec2::new(0, 0));
        Self {
            h: Vec2::new(0, 0),
            t: Vec2::new(0, 0),
            history,
        }
    }

    fn mov(&mut self, mov: (&str, i32)) {
        for _ in 0..mov.1 {
            match mov.0 {
                "U" => self.h.y += 1,
                "D" => self.h.y -= 1,
                "R" => self.h.x += 1,
                "L" => self.h.x -= 1,
                _ => panic!("Invalid input"),
            }

            println!("H at: {:?}", self.h);

            if self.h.distance_to(&self.t) > 1.7 {
                self._mov_t_to_h();
            }
            println!("T at: {:?}", self.t);
        }
    }

    fn _mov_t_to_h(&mut self) {
        let mut closest_mov = Vec2::new(0, 0);
        let mut closest_dist = f64::MAX;
        for x in -1..=1 {
            for y in -1..=1 {
                if x != 0 || y != 0 {
                    let mov = Vec2::new(x, y);
                    let dist = self.h.distance_to(&self.t.add(&mov));
                    if dist < closest_dist {
                        closest_dist = dist;
                        closest_mov = mov;
                    }
                }
            }
        }
        self.t = self.t.add(&closest_mov);
        self.history.insert(self.t);
    }
}

fn main() {
    let mut rope = Rope::new();

    common::utils::lines_from_file("inputs/day9.txt")
        .iter()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|o| (o[0], o[1].parse::<i32>().unwrap()))
        .for_each(|mov| rope.mov(mov));

    println!("{:?}", rope.history.len())
}
