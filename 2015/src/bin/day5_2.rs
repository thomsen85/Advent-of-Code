// Requirements:
// It contains a pair of any two letters that appears at least twice in the string
// without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa
// (aa, but it overlaps).
// It contains at least one letter which repeats with exactly one letter between them,
// like xyx, abcdefeghi (efe), or even aaa.

fn main() {
    let input = common::utils::lines_from_file("inputs/day5.txt");
    let mut nice_strings = 0;

    for string in input {
        if is_good_string(string) {
            nice_strings += 1;
        }
    }
    dbg!(nice_strings);
}

fn is_good_string(input: String) -> bool {
    // It contains a pair of any two letters that appears at least twice in the string
    // without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa
    // (aa, but it overlaps).
    let mut has_pair = false;
    for i in 2..input.len() {
        let pair = &input[(i - 2)..i];
        for j in i..input.len() - 1 {
            if &input[j..(j + 2)] == pair {
                has_pair = true;
            }
        }
    }

    if !has_pair {
        return false;
    }

    // It contains at least one letter which repeats with exactly one letter between them,
    // like xyx, abcdefeghi (efe), or even aaa.
    let mut twice = false;
    for i in 2..input.len() {
        if input.chars().nth(i - 2) == input.chars().nth(i) {
            twice = true;
        }
    }

    if !twice {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::is_good_string;

    #[test]
    fn test_nice() {
        let nice = "qjhvhtzxzqqjkmpb";
        assert!(is_good_string(nice.to_owned()))
    }
    #[test]
    fn test_not_nice_1() {
        let nice = "uurcxstgmygtbstg";
        assert!(!is_good_string(nice.to_owned()))
    }
    #[test]
    fn test_not_nice_2() {
        let nice = "ieodomkazucvgmuy";
        assert!(!is_good_string(nice.to_owned()))
    }
    #[test]
    fn test_nice_2() {
        let nice = "xxyxx";
        assert!(is_good_string(nice.to_owned()))
    }
}
