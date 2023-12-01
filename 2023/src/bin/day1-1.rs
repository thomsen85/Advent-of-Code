fn main() {
    let i = include_str!("../../inputs/day1.txt");
    dbg!(solve(i));
}

fn solve(input: &str) -> u32 {
    let i = input.lines();
    let mut sum = 0;

    for l in i {
        let mut first = 0;
        let mut is_first = true;
        let mut last = 0;
        for c in l.chars() {
            if c.is_numeric() {
                if is_first {
                    first = c.to_digit(10).unwrap();
                    is_first = false;
                }
                last = c.to_digit(10).unwrap();
            }
        }
        sum += first * 10 + last;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ti = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let res = solve(ti);

        assert_eq!(142, res);
    }
}
