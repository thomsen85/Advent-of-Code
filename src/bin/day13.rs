use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum NestedListItem {
    List(NestedList),
    Item(i32),
}

impl Display for NestedListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NestedListItem::Item(v) => f.write_str(&v.to_string())?,
            NestedListItem::List(v) => f.write_str(&format!("{}", v))?,
        }
        Ok(())
    }
}

impl Ord for NestedListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            NestedListItem::Item(l_item) => match other {
                NestedListItem::Item(r_item) => {
                    return l_item.cmp(&r_item);
                }
                NestedListItem::List(r_list) => {
                    let l_list = NestedList::from(l_item.to_owned());
                    return l_list.cmp(r_list);
                }
            },
            NestedListItem::List(l_list) => match other {
                NestedListItem::Item(r_item) => {
                    let r_list = NestedList::from(r_item.to_owned());
                    return l_list.cmp(&r_list);
                }
                NestedListItem::List(r_list) => {
                    return l_list.cmp(r_list);
                }
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct NestedList {
    list: Vec<NestedListItem>,
}

impl NestedList {
    fn new() -> Self {
        Self { list: Vec::new() }
    }
}

impl From<i32> for NestedList {
    fn from(value: i32) -> Self {
        Self {
            list: vec![NestedListItem::Item(value)],
        }
    }
}

impl Display for NestedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = self.list.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(&format!("{}", x));
            acc.push(',');
            acc
        });

        f.write_str(&format!("[{}]", content).replace(",]", "]"))?;
        Ok(())
    }
}

impl PartialOrd for NestedList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.list.cmp(&other.list))
    }
}

impl Ord for NestedList {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut index = 0;
        loop {
            if index >= self.list.len() || index >= other.list.len() {
                return self.list.len().cmp(&other.list.len());
            }

            let l_i = &self.list[index];
            let r_i = &other.list[index];
            let cmp = l_i.cmp(r_i);
            if cmp.is_ne() {
                return cmp;
            }
            index += 1
        }
    }
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut input: Vec<NestedList> = aoc2022_rust::utils::lines_from_file("inputs/day13.txt")
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|s| parse(&s))
        .collect();

    input.push(parse("[[2]]"));
    input.push(parse("[[6]]"));
    input.sort();

    let index_2 = input.binary_search(&parse("[[2]]")).unwrap() + 1;
    let index_6 = input.binary_search(&parse("[[6]]")).unwrap() + 1;
    dbg!(index_2, index_6, index_2 * index_6);
}

fn part1() {
    let input: Vec<Vec<String>> = aoc2022_rust::utils::paragraph_from_file("inputs/day13.txt")
        .into_iter()
        .map(|s| s.lines().map(|s| s.to_string()).collect())
        .collect();

    let mut index_sum = 0;
    for (i, pair) in input.iter().enumerate() {
        let index = i + 1;
        let l = parse(&pair[0]);
        let r = parse(&pair[1]);
        let res = l.cmp(&r);
        if res.is_lt() {
            index_sum += index;
        }
    }
    println!("Index Sum: {}", index_sum);
}

fn parse(string: &str) -> NestedList {
    list_parser(0, &string[1..string.len()]).1
}

fn list_parser(mut at: usize, slice: &str) -> (usize, NestedList) {
    let mut res = NestedList::new();
    let mut temp_number = Vec::new();
    while at < slice.len() {
        let c = slice.chars().nth(at).unwrap();
        if c == ']' {
            if !temp_number.is_empty() {
                let num = temp_number
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0, |acc, (n, x)| acc + x * 10_u32.pow(n as u32));
                res.list.push(NestedListItem::Item(num as i32));
                temp_number.clear();
            }
            return (at, res);
        } else if c == '[' {
            at += 1;
            let (new_at, list) = list_parser(at, slice);
            at = new_at;
            res.list.push(NestedListItem::List(list))
        } else if c.is_numeric() {
            temp_number.push(c.to_digit(10).unwrap())
        } else if c == ',' {
            if !temp_number.is_empty() {
                let num = temp_number
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0, |acc, (n, x)| acc + x * 10_u32.pow(n as u32));
                res.list.push(NestedListItem::Item(num as i32));
                temp_number.clear();
            }
        }
        at += 1;
    }
    (at, res)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn empty_parse_test() {
        let test_sting = "[[[]]]";
        let parse_result = parse(test_sting);
        assert_eq!(1, parse_result.list.len());
    }

    #[test]
    fn test_ordering1() {
        let test_string_1 = "[1,[2,[3,[4,[5,6,0]]]],9,9]";
        let test_string_2 = "[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert!(parse(test_string_1).cmp(&parse(test_string_2)).is_gt());
    }

    #[test]
    fn test_ordering2() {
        let test_string_1 = "[1,[2,[3,[4,[5,6,0]]]],9,9]";
        let test_string_2 = "[1,[2,[3,[4,[5,6,0]]]],9,9]";

        assert!(parse(test_string_1).cmp(&parse(test_string_2)).is_eq());
    }

    #[test]
    fn test_ordering3() {
        let test_string_1 = "[1,[2,[3,[4,[5,6,0]]]],[],9]";
        let test_string_2 = "[1,[2,[3,[4,[5,6,0]]]],9,9]";

        assert!(parse(test_string_1).cmp(&parse(test_string_2)).is_lt());
    }

    #[test]
    fn test_ordering4() {
        let test_string_1 = "[[[0,3,5,[],6],[[5,5,0,7,2],[9,6,7],2],1]]";
        let test_string_2 = "[]";

        assert!(parse(test_string_1).cmp(&parse(test_string_2)).is_gt());
    }

    #[test]
    fn test_ordering5() {
        let test_string_1 = "[1,1,3,1,1]";
        let test_string_2 = "[[1],[2,3,4]]";

        assert!(parse(test_string_1).cmp(&parse(test_string_2)).is_lt());
    }

    #[test]
    fn test_ordering6() {
        let test_string_1 = "[1,1,3,1,1]";
        let test_string_2 = "[[[]]]";

        assert!(parse(test_string_1).cmp(&parse(test_string_2)).is_gt());
    }
}
