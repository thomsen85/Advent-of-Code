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
    let (input, mut length) = separated_list1(tag("x"), nom::character::complete::i32)(input)?;
    length.sort();
    let (l, w, h) = (length[0], length[1], length[2]);
    let ribbon = l * 2 + w * 2 + l * w * h;
    Ok((input, ribbon))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_test() {
        let test = "2x3x4";
        let r = parse(test).unwrap().1;
        assert_eq!(r, 34);
    }
}
