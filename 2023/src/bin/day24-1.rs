use glam::{DMat2, DVec2};
use itertools::Itertools;

fn main() {
    dbg!(solve(
        include_str!("../../inputs/day24.txt"),
        200_000_000_000_000.,
        400_000_000_000_000.
    ));
}

fn solve(input: &str, test_min: f64, test_max: f64) -> String {
    assert!(test_max < 2_f64.powi(52)); // 52 accurate mantissa

    let hails = input
        .lines()
        .map(|line| {
            line.replace(" @ ", ", ")
                .split(", ")
                .map(|c| {
                    c.trim()
                        .parse::<f64>()
                        .expect(&format!("{} couldt be parsed", c))
                })
                .collect_tuple::<(f64, f64, f64, f64, f64, f64)>()
                .unwrap()
        })
        .map(|x| (DVec2::new(x.0, x.1), DVec2::new(x.3, x.4)))
        .collect_vec();

    let mut sum = 0;

    for h1_i in 0..hails.len() {
        let (p1, d1) = hails[h1_i];

        for h2_i in 0..hails.len() {
            if h1_i == h2_i {
                continue;
            }
            let (p2, d2) = hails[h2_i];
            // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection

            // Putter konstanter på høyre side som b
            // Koefissienter som A
            // løser for x
            let b = p2 - p1;

            let a = DMat2::from_cols(d1, -d2);

            if a.determinant() == 0.0 {
                continue;
            }
            let x = a.inverse().mul_vec2(b);

            let dx = x.x * d1;
            let p = p1 + dx;

            if x.min_element() < 0. {
                continue;
            }

            if p.min_element() > test_min && p.max_element() < test_max {
                sum += 1;
            }
        }
    }

    (sum / 2).to_string()
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
        assert_eq!(solve(ti, 7., 27.), "2".to_string());
    }
}
