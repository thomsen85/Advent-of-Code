use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::opt, IResult, Slice,
};

fn main() {
    let i = include_str!("../../inputs/day1.txt");
    dbg!(solve(i));
}

fn solve(input: &str) -> i32 {
    input.lines().map(|l| parse_line(l).unwrap().1).sum()
}

fn parse_line(input: &str) -> IResult<&str, i32> {
    let mut first = 0;
    let mut is_first = true;
    let mut last = 0;

    for i in 0..input.len() {
        if let Some(res) = opt(alt((
            tag("one"),
            tag("two"),
            tag("three"),
            tag("four"),
            tag("five"),
            tag("six"),
            tag("seven"),
            tag("eight"),
            tag("nine"),
            digit1,
        )))(input.slice(i..input.len()))?
        .1
        {
            let num = to_num(res);
            if is_first {
                first = num;
                is_first = false;
            }
            last = num
        }
    }
    dbg!(first * 10 + last);

    Ok((input, first * 10 + last))
}

fn to_num(i: &str) -> i32 {
    match i {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => i
            .chars()
            .take(1)
            .map(|c| c.to_owned())
            .collect::<String>()
            .parse::<i32>()
            .unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ti = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let res = solve(ti);

        assert_eq!(281, res);
    }
}
