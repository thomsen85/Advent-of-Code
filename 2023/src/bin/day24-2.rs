use std::{
    ops::{Index, IndexMut},
    time::Instant,
};

use itertools::Itertools;
use num::{BigRational, FromPrimitive, Zero};

fn main() {
    let timer = Instant::now();
    let ans = solve(include_str!("../../inputs/day24.txt"));
    let time = timer.elapsed();
    println!(
        "Answer: {}\n\nTook {} ms",
        ans,
        time.as_micros() as f64 / 1000.
    );
}

#[derive(Debug, Clone)]
struct Point {
    x: BigRational,
    y: BigRational,
    z: BigRational,
}

#[derive(Debug, Clone)]
struct Hailstone {
    p: Point,
    v: Point,
}

#[derive(Debug, Clone)]
struct BRMatrix {
    shape: (usize, usize),
    elements: Vec<BigRational>,
}

impl Index<(usize, usize)> for BRMatrix {
    type Output = BigRational;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.elements[index.0 * self.shape.1 + index.1]
    }
}

impl IndexMut<(usize, usize)> for BRMatrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.elements[index.0 * self.shape.1 + index.1]
    }
}

impl BRMatrix {
    /// Ineffective solver using gauss-jordan. Assuiming self i (n, n+1) Augmented matrix
    /// returning x (n, 1)
    fn into_solve(mut self) -> Self {
        let n = self.shape.0;
        for i in 0..n {
            if self[(i, i)].is_zero() {
                panic!("Divide by zero");
            }

            for j in 0..n {
                if i == j {
                    continue;
                }

                let ratio = self[(j, i)].clone() / self[(i, i)].clone();
                for k in 1..=n {
                    let s = ratio.clone() * self[(i, k)].clone();
                    self[(j, k)] -= s;
                }
            }
        }

        let mut x = Vec::new();
        for i in 0..n {
            x.push(self[(i, n)].clone() / self[(i, i)].clone());
        }

        BRMatrix {
            shape: (4, 1),
            elements: x,
        }
    }
}

fn solve(input: &str) -> String {
    let hails = input
        .lines()
        .map(|line| {
            line.replace(" @ ", ", ")
                .split(", ")
                .map(|c| {
                    BigRational::from_i64(c.trim().parse().expect(&format!("{} not a digit", c)))
                        .unwrap()
                })
                .collect_vec()
        })
        .map(|l| Hailstone {
            p: Point {
                x: l[0].clone(),
                y: l[1].clone(),
                z: l[2].clone(),
            },
            v: Point {
                x: l[3].clone(),
                y: l[4].clone(),
                z: l[5].clone(),
            },
        })
        .collect_vec();

    let (h1, h2, h3) = hails.into_iter().take(3).collect_tuple().unwrap();

    let Hailstone { v: v0, p: p0 } = h1;
    let Hailstone { v: v1, p: p1 } = h2;
    let Hailstone { v: v2, p: p2 } = h3;

    let am = BRMatrix {
        shape: (6, 7),
        elements: vec![
            v1.y.clone() - v0.y.clone(),
            v0.x.clone() - v1.x.clone(),
            BigRational::zero(),
            p0.y.clone() - p1.y.clone(),
            p1.x.clone() - p0.x.clone(),
            BigRational::zero(),
            (p0.y.clone() * v0.x.clone() - p1.y.clone() * v1.x.clone())
                - (p0.x.clone() * v0.y.clone() - p1.x.clone() * v1.y.clone()),
            v2.y.clone() - v0.y.clone(),
            v0.x.clone() - v2.x.clone(),
            BigRational::zero(),
            p0.y.clone() - p2.y.clone(),
            p2.x.clone() - p0.x.clone(),
            BigRational::zero(),
            (p0.y.clone() * v0.x.clone() - p2.y.clone() * v2.x.clone())
                - (p0.x.clone() * v0.y.clone() - p2.x.clone() * v2.y.clone()),
            v1.z.clone() - v0.z.clone(),
            BigRational::zero(),
            v0.x.clone() - v1.x.clone(),
            p0.z.clone() - p1.z.clone(),
            BigRational::zero(),
            p1.x.clone() - p0.x.clone(),
            (p0.z.clone() * v0.x.clone() - p1.z.clone() * v1.x.clone())
                - (p0.x.clone() * v0.z.clone() - p1.x.clone() * v1.z.clone()),
            v2.z.clone() - v0.z.clone(),
            BigRational::zero(),
            v0.x.clone() - v2.x.clone(),
            p0.z.clone() - p2.z.clone(),
            BigRational::zero(),
            p2.x.clone() - p0.x.clone(),
            (p0.z.clone() * v0.x.clone() - p2.z.clone() * v2.x.clone())
                - (p0.x.clone() * v0.z.clone() - p2.x.clone() * v2.z.clone()),
            BigRational::zero(),
            v1.z.clone() - v0.z.clone(),
            v0.y.clone() - v1.y.clone(),
            BigRational::zero(),
            p0.z.clone() - p1.z.clone(),
            p1.y.clone() - p0.y.clone(),
            (p0.z.clone() * v0.y.clone() - p1.z.clone() * v1.y.clone())
                - (p0.y.clone() * v0.z.clone() - p1.y.clone() * v1.z.clone()),
            BigRational::zero(),
            v2.z.clone() - v0.z.clone(),
            v0.y.clone() - v2.y.clone(),
            BigRational::zero(),
            p0.z.clone() - p2.z.clone(),
            p2.y.clone() - p0.y.clone(),
            (p0.z.clone() * v0.y.clone() - p2.z.clone() * v2.y.clone())
                - (p0.y.clone() * v0.z.clone() - p2.y.clone() * v2.z.clone()),
        ],
    };

    let ans = am.into_solve();
    ans.elements.iter().take(3).sum::<BigRational>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(solve(ti), "47".to_string());
    }

    #[test]
    fn test_solve() {
        let elements = vec![1, 3, 4, 2, 2, 4];
        let am = BRMatrix {
            shape: (2, 3),
            elements: elements
                .into_iter()
                .map(|a| BigRational::from_i32(a).unwrap())
                .collect_vec(),
        };
        assert_eq!(
            am.into_solve()
                .elements
                .iter()
                .map(|x| format!("{}", x))
                .join(", "),
            "1, 1".to_string()
        );
    }
}
