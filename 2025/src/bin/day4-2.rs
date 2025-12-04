use std::time::Instant;

use common::strings::string_to_char_grid;

// Delta time: 4 min 34 sek
fn main() {
    let now = Instant::now();
    let res = solve(include_str!("../../inputs/day4.txt"));
    let elapsed = now.elapsed();
    dbg!(res);
    println!("Time used {elapsed:?}");
}
fn solve(input: &str) -> String {
    let mut p = string_to_char_grid(input);

    let mut ans = 0;
    loop {
        let mut p_clone = p.clone();
        let mut changed = false;

        for y in 0..p.len() as i32 {
            for x in 0..p[0].len() as i32 {
                if p[y as usize][x as usize] != '@' {
                    continue;
                }
                let mut count = 0;
                for j in -1..=1 {
                    for i in -1..=1 {
                        if y + j < 0
                            || x + i < 0
                            || j + y >= p.len() as i32
                            || i + x >= p[0].len() as i32
                            || (i == 0 && j == 0)
                        {
                            continue;
                        }
                        if p[(y + j) as usize][(x + i) as usize] == '@' {
                            count += 1
                        }
                    }
                }

                if count < 4 {
                    changed = true;
                    ans += 1;
                    p_clone[y as usize][x as usize] = '.';
                }
            }
        }
        p = p_clone;

        if !changed {
            break;
        }
    }
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(solve(ti), "43".to_string());
    }
}
