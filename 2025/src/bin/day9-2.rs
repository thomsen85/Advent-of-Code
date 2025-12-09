use itertools::Itertools;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let res = solve(include_str!("../../inputs/day9.txt"));
    let elapsed = now.elapsed();
    dbg!(res);
    println!("Time used {elapsed:?}");
}

struct Polygon {
    sides: Vec<(f64, f64, f64, f64)>,
}

impl Polygon {
    pub fn intersects_any_other_polygon_side_non_inclusive(&self, other: &Self) -> bool {
        for (i, self_side) in self.sides.iter().enumerate() {
            for other_side in other.sides.iter().skip(i + 1) {
                // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
                let (x1, y1, x2, y2) = self_side;
                let (x3, y3, x4, y4) = other_side;

                let below_frac = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

                if below_frac.abs() < f64::EPSILON {
                    continue;
                }

                let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / below_frac;

                let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / below_frac;

                if (0. ..=1.).contains(&t) && (0. ..=1.).contains(&u) {
                    return true;
                }
            }
        }
        false
    }
}

// 4749929916 to high
fn solve(input: &str) -> String {
    let coords = input
        .trim()
        .split("\n")
        .map(|l| l.split_once(",").unwrap())
        .map(|(a, b)| (a.parse::<f64>().unwrap(), b.parse::<f64>().unwrap()))
        .collect_vec();

    let len = coords.len();

    let poly = Polygon {
        sides: coords
            .iter()
            .cycle()
            .take(len + 1)
            .tuple_windows()
            .map(|(a, b)| (a.0, a.1, b.0, b.1))
            .collect(),
    };

    let mut max = 0.;
    for (i1, c1) in coords.iter().enumerate() {
        for c2 in coords.iter().skip(i1 + 1) {
            let area = ((c2.0 - (c1.0)).abs() + 1.) * ((c2.1 - c1.1).abs() + 1.);
            if area <= max {
                continue;
            }

            let d_x = c2.0 - c1.0;
            let d_y = c2.1 - c1.1;

            let m = 0.1;
            let s_x1 = c1.0 + d_x.signum() * m;
            let s_y1 = c1.1 + d_y.signum() * m;
            let s_x2 = c2.0 - d_x.signum() * m;
            let s_y2 = c2.1 - d_y.signum() * m;

            let shrinked_polygon_points =
                vec![(s_x1, s_y1), (s_x1, s_y2), (s_x2, s_y2), (s_x2, s_y1)];
            let shrinked_polygon_sides = shrinked_polygon_points
                .into_iter()
                .cycle()
                .take(5)
                .tuple_windows()
                .map(|(a, b)| (a.0, a.1, b.0, b.1))
                .collect();

            let shrinked_polygon = Polygon {
                sides: shrinked_polygon_sides,
            };

            if poly.intersects_any_other_polygon_side_non_inclusive(&shrinked_polygon) {
                continue;
            }

            max = area;
        }
    }
    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(solve(ti), "24".to_string());
    }
}
