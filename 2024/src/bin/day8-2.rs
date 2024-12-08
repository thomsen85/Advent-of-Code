use std::collections::{HashMap, HashSet};

use common::{datastructs::vec2::Vec2, utils::string_to_char_grid};

fn main() {
    dbg!(solve(include_str!("../../inputs/day8.txt")));
}

fn solve(input: &str) -> String {
    let m = string_to_char_grid(input);
    let mut antennas: HashMap<char, Vec<Vec2>> = HashMap::new();
    let mut antinodes: HashSet<Vec2> = HashSet::new();

    for r in 0..m.len() {
        for c in 0..m[0].len() {
            if m[r][c] != '.' {
                antennas
                    .entry(m[r][c])
                    .or_default()
                    .push(Vec2::from_row_col(r, c));
            }
        }
    }

    for (_ch, l) in antennas.iter() {
        for p1 in 0..l.len() {
            for p2 in (p1 + 1)..l.len() {
                // Do both ways
                // dbg!(ch, l, p1, p2);
                let l1 = l[p1];
                let l2 = l[p2];
                let diff = l2 - l1;

                let mut prev_l1 = l1 - diff;
                while (0..m.len() as i32).contains(&prev_l1.x)
                    && (0..m[0].len() as i32).contains(&prev_l1.y)
                {
                    antinodes.insert(prev_l1);
                    prev_l1 = prev_l1 - diff;
                }
                let mut prev_l2 = l2 - diff;
                while (0..m.len() as i32).contains(&prev_l2.x)
                    && (0..m[0].len() as i32).contains(&prev_l2.y)
                {
                    antinodes.insert(prev_l2);
                    prev_l2 = prev_l2 + diff;
                }

                let n = vec![l1 + diff / 3, l2 - diff / 3];
                for b in n {
                    let dl1 = b - l1;
                    let dl2 = b - l2;
                    if (0..m.len() as i32).contains(&b.x)
                        && (0..m[0].len() as i32).contains(&b.y)
                        && (dl1 * 2 == dl2 || dl2 * 2 == dl1)
                    {
                        antinodes.insert(b);
                    }
                }
            }
        }
    }

    antinodes.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(solve(ti), "34".to_string());
    }
}
