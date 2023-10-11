use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};

fn main() {
    let input = common::utils::lines_from_file("inputs/day2.txt");
    let mut sum = 0;
    for line in input {
        sum += parse(&line).expect(&format!("Line: {} dint work", line)).1;
    }
    dbg!(sum);
}

fn parse(input: &str) -> IResult<&str, i32> {
    let (input, length) = separated_list1(tag("x"), nom::character::complete::i32)(input)?;
    if let Some((l, w, h)) = length.into_iter().tuples().next() {
        let slack = (l * w).min(h * l).min(w * h);
        return Ok((input, 2 * l * w + 2 * w * h + 2 * h * l + slack));
    }
    unreachable!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let test = "2x3x4";
        let r = parse(test).unwrap().1;
        assert_eq!(r, 58);
    }
}
