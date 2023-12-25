use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
// For number types
use nom::character::complete as cnom;

fn main() {
    dbg!(solve(
        include_str!("../../inputs/day24.txt"),
        200_000_000_000_000.,
        400_000_000_000_000.
    ));
}

fn solve(input: &str, test_min: f64, test_max: f64) -> String {
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
        .collect_vec();

    let mut sum = 0;

    for h1_i in 0..hails.len() {
        let (x1, y1, _z1, dx1, dy1, _dz1) = hails[h1_i];
        for h2_i in 0..hails.len() {
            if h1_i == h2_i {
                continue;
            }
            let (x2, y2, _z2, dx2, dy2, _dz2) = hails[h2_i];

            println!(
                "Hailstone A: {}, {}, {} @ {}, {}, {}",
                x1, y1, _z1, dx1, dy1, _dz1
            );
            println!(
                "Hailstone B: {}, {}, {} @ {}, {}, {}",
                x2, y2, _z2, dx2, dy2, _dz2
            );

            let t = (x1 - x2) / (dx2 - dx1);
            let x = t * dx1 + x1;
            dbg!(x, t);
            if t > 0. || x < test_min || x > test_max {
                continue;
            }

            let y = t * dy1 + y1;
            if y < test_min || y > test_max || y != y2 + dy2 * t {
                dbg!(y1 + dy1 * t, y2 + dy2 * t);
                continue;
            }
            sum += 1;
        }
    }

    sum.to_string()
}

fn parse(input: &str) -> IResult<&str, ()> {
    todo!("Add parser");

    Ok((input, ()))
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
