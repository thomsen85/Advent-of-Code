use common::{
    datastructs::vec2::Vec2, graphs::priority::Priority, strings::string_to_extracted_nums_t_vec,
};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    time::Instant,
};
// For number types

fn main() {
    let now = Instant::now();
    dbg!(solve(include_str!("../../inputs/day18.txt"), 70, 1024));
    let elapsed = now.elapsed();
    println!("Time used {elapsed:?}");
}

fn solve(input: &str, size: usize, bytes: usize) -> String {
    let nums = input
        .lines()
        .map(|l| string_to_extracted_nums_t_vec::<i32>(l).try_into().unwrap())
        .map(|[col, row]: [i32; 2]| Vec2::from_row_col(row as usize, col as usize))
        .take(bytes)
        .collect::<HashSet<_>>();

    let mut q = BinaryHeap::new();
    let mut visited = HashMap::new();
    let start = Vec2::from_row_col(0, 0);
    let end = Vec2::from_row_col(size, size); // kansje minus 1

    q.push(Priority {
        value: 0,
        data: start,
    });

    while let Some(Priority { value, data }) = q.pop() {
        if let Some(val) = visited.get(&data) {
            if *val >= value {
                continue;
            }
        }
        visited.insert(data, value);

        if data == end {
            return value.to_string();
        }

        for n in data.neighbours_4_ranged(0..=size as i32, 0..=size as i32) {
            if nums.contains(&n) {
                continue;
            }

            q.push(Priority {
                value: value + 1,
                data: n,
            });
        }
    }

    " ".to_string()
}

fn parse(input: &str) -> IResult<&str, ()> {
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(solve(ti, 6, 12), "22".to_string());
    }
}
