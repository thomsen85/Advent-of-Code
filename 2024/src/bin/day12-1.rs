use std::collections::HashSet;

use common::{datastructs::vec2::Vec2, strings::string_to_char_grid};
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, space0, space1},
    sequence::{delimited, preceded},
    IResult,
};
// For number types
use nom::character::complete as cnom;

fn main() {
    dbg!(solve(include_str!("../../inputs/day12.txt")));
}

fn solve(input: &str) -> String {
    let m = string_to_char_grid(input);
    let mut visited = vec![vec![0; m[1].len()]; m.len()];
    let mut s_id = 0;
    let mut shapes: Vec<Vec<Vec2>> = Vec::new();

    let mut stack = Vec::new();
    let start = Vec2::new(0, 0);
    stack.push((start, *start.i_arr(&m)));
    loop {
        shapes.push(vec![]);
        dbg!(&stack);
        while let Some((curr, curr_id)) = stack.pop() {
            if *curr.i_arr(&visited) == 1 {
                continue;
            }
            *curr.i_arr_mut(&mut visited) = 1;
            shapes[s_id].push(curr);

            for n in curr.neighbours_4_ranged(0..m.len() as i32, 0..m[0].len() as i32) {
                if *n.i_arr(&m) == curr_id {
                    stack.push((n, curr_id));
                }
            }
        }

        // Find next not visited squre
        s_id += 1;

        for r in 0..m.len() {
            for c in 0..m[0].len() {
                if visited[r][c] == 0 {
                    stack.push((Vec2::from_row_col(r, c), m[r][c]));
                    break;
                }
            }
            if !stack.is_empty() {
                break;
            }
        }

        if stack.is_empty() {
            break;
        }
    }

    //
    let area = shapes.iter().map(|shape| shape.len());
    //
    let perimiter = shapes.iter().map(|shape| {
        let mut vi = HashSet::new();

        let mut total = 0;
        for p in shape {
            let neighbours = p
                .neighbours_4_ranged(-1..m.len() as i32 + 1, -1..m[0].len() as i32 + 1)
                .into_iter()
                .map(|n| if vi.contains(&n) { -1 } else { 1 })
                .sum::<i32>();

            total += neighbours;
            vi.insert(p);
        }
        total
    });

    area.zip(perimiter)
        .map(|(a, b)| dbg!(a) as i32 * dbg!(b))
        .sum::<i32>()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, ()> {
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(solve(ti), "140".to_string());
    }

    #[test]
    fn test_2() {
        let ti = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(solve(ti), "772".to_string());
    }

    #[test]
    fn test_4() {
        let ti = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(solve(ti), "1930".to_string());
    }
}
