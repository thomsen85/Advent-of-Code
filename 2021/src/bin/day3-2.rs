use common::datastructs::pwm::PositionWeightMatrix;
use itertools::Itertools;

fn main() {
    dbg!(solve(include_str!("../../inputs/day3.txt")));
}

fn solve(input: &str) -> String {
    let mut ox = input.lines().collect_vec();
    let mut ox_i = 0;
    while ox.len() > 1 {
        let ox_pwm = PositionWeightMatrix::from_grid_str(&ox.join("\n"));

        ox = ox
            .into_iter()
            .filter(|line| {
                line.chars().nth(ox_i).unwrap()
                    == if ox_pwm.matrix.get("0").unwrap()[ox_i]
                        > ox_pwm.matrix.get("1").unwrap()[ox_i]
                    {
                        '0'
                    } else {
                        '1'
                    }
            })
            .collect_vec();
        ox_i += 1;
    }

    let mut co = input.lines().collect_vec();
    let mut co_i = 0;
    while co.len() > 1 {
        let co_pwm = PositionWeightMatrix::from_grid_str(&co.join("\n"));

        co = co
            .into_iter()
            .filter(|line| {
                line.chars().nth(co_i).unwrap()
                    == if co_pwm.matrix.get("1").unwrap()[co_i]
                        < co_pwm.matrix.get("0").unwrap()[co_i]
                    {
                        '1'
                    } else {
                        '0'
                    }
            })
            .collect_vec();
        co_i += 1;
    }
    dbg!(&ox, &co);
    (isize::from_str_radix(ox[0], 2).unwrap() * isize::from_str_radix(co[0], 2).unwrap())
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(solve(ti), "230".to_string());
    }
}
