use std::collections::HashSet;

use nom::IResult;

struct Vertex {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    unimplemented!();
}

fn solve(i: String) -> usize {}

fn parse(i: &str) -> IResult<&str, HashSet<Vertex>> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
            .to_owned();
        assert_eq!(solve(input), 64);
    }
}
