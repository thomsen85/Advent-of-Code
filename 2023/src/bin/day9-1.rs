use itertools::Itertools;
use nom::{
    character::complete::{newline, space1},
    multi::separated_list1,
    IResult,
};
// For number types
use nom::character::complete as cnom;

fn main() {
    dbg!(solve(include_str!("../../inputs/day9.txt")));
}

fn solve(input: &str) -> String {
    let (_, lines) = parse(input).unwrap();

    let mut sum = 0;
    for line in lines {
        let mut i = 0;
        let mut diff_lines: Vec<Vec<i32>> = Vec::new();
        diff_lines.push(line);

        loop {
            diff_lines.push(
                diff_lines[i]
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect_vec(),
            );

            i += 1;

            if diff_lines[i].iter().all(|a| *a == 0) {
                break;
            }
        }

        diff_lines.last_mut().unwrap().push(0);

        let mut c = diff_lines.len() - 1;
        loop {
            if c <= 0 {
                break;
            }
            let l1 = diff_lines[c].last().unwrap().clone();
            let l2 = diff_lines[c - 1].last().unwrap().clone();

            diff_lines[c - 1].push(l1 + l2);

            c -= 1;
        }

        sum += diff_lines[0].last().unwrap();
    }

    sum.to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, lines) = separated_list1(newline, separated_list1(space1, cnom::i32))(input)?;

    Ok((input, lines))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(solve(ti), "114".to_string());
    }
}
