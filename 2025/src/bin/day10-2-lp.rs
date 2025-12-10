use std::time::Instant;

use good_lp::{
    Expression, ProblemVariables, Solution, SolutionStatus, SolverModel, microlp, variable,
};
use itertools::Itertools;

fn main() {
    let now = Instant::now();
    let res = solve(include_str!("../../inputs/day10.txt"));
    let elapsed = now.elapsed();
    dbg!(res);
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let lines = input
        .trim()
        .split("\n")
        .map(|line| {
            let (schema, rest) = line.split_once("]").unwrap();
            let schema = schema[1..].chars().collect_vec();
            let (rest, joltage) = rest.split_once("{").unwrap();
            let joltage = joltage[..(joltage.len() - 1)]
                .split(",")
                .map(|x| x.parse::<i16>().unwrap())
                .collect_vec();

            let buttons = rest
                .replace(" ", "")
                .split(")")
                .take_while(|x| !x.is_empty())
                .map(|btn| {
                    btn[1..]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            (schema, buttons, joltage)
        })
        .collect_vec();

    let mut presses: f64 = 0.;

    for (_target_schema, buttons, target_joltages) in lines {
        let mut problem = ProblemVariables::new();
        // minimize the sum of variables, where variables are constant times button.
        let vars = (0..buttons.len())
            .map(|_i| problem.add(variable().integer().min(0)))
            .collect_vec();

        let mut objective: Expression = 0.into();
        for v in &vars {
            objective += v;
        }
        let mut problem = problem.minimise(objective).using(microlp);

        // constraint is for all indexes, sum of all buttons that touch that index == target
        // joltage
        for (target_joltage_i, target_joltage) in target_joltages.iter().enumerate() {
            let mut a: Expression = 0.into();
            for (button_i, button) in buttons.iter().enumerate() {
                if !button.contains(&target_joltage_i) {
                    continue;
                }

                a += vars[button_i]
            }
            problem = problem.with(a.eq(*target_joltage));
        }
        let s = problem.solve().unwrap();
        assert!(matches!(s.status(), SolutionStatus::Optimal));

        for v in vars {
            presses += s.value(v);
        }
    }
    presses.to_string() // to low: 18901
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve(ti), "33".to_string());
    }
}
