// Requirements:
// It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
// It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
// It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

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

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const ILLEGAL_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn is_good_string(input: String) -> bool {
    // It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
    let vowels = input
        .chars()
        .into_iter()
        .fold(0, |acc, x| if VOWELS.contains(&x) { acc + 1 } else { acc });
    if vowels < 3 {
        return false;
    }
    // It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).

    let mut twice = false;
    for i in 1..input.len() {
        if input.chars().nth(i - 1) == input.chars().nth(i) {
            twice = true;
        }
    }

    if !twice {
        return false;
    }
    // It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

    for illegal in ILLEGAL_STRINGS {
        if input.contains(illegal) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::is_good_string;

    #[test]
    fn test_nice() {
        let nice = "ugknbfddgicrmopn";
        assert!(is_good_string(nice.to_owned()))
    }
    #[test]
    fn test_not_nice_1() {
        let nice = "jchzalrnumimnmhp";
        assert!(!is_good_string(nice.to_owned()))
    }
    #[test]
    fn test_not_nice_2() {
        let nice = "haegwjzuvuyypxyu";
        assert!(!is_good_string(nice.to_owned()))
    }
    #[test]
    fn test_not_nice_3() {
        let nice = "dvszwmarrgswjxmb";
        assert!(!is_good_string(nice.to_owned()))
    }
}
