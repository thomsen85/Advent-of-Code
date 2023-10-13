use nom::{
    branch::alt,
    bytes::complete::tag,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
enum Action {
    TurnOff,
    TurnOn,
    Toggle,
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "turn off" => Self::TurnOff,
            "turn on" => Self::TurnOn,
            "toggle" => Self::Toggle,
            _ => unimplemented!("{} not supported action", value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    f_x: usize,
    f_y: usize,
    t_x: usize,
    t_y: usize,
}

impl Range {
    /// Parsing i.e "752,335 through 957,733"
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (f_x, f_y)) = separated_pair(
            nom::character::complete::i32,
            tag(","),
            nom::character::complete::i32,
        )(input)?;
        let (input, _) = tag(" through ")(input)?;

        let (input, (t_x, t_y)) = separated_pair(
            nom::character::complete::i32,
            tag(","),
            nom::character::complete::i32,
        )(input)?;

        Ok((
            input,
            Self {
                f_x: f_x as usize,
                f_y: f_y as usize,
                t_x: t_x as usize,
                t_y: t_y as usize,
            },
        ))
    }
}

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

fn main() {
    let input = common::utils::lines_from_file("inputs/day6.txt");
    let result = get_lights_on(input);
    dbg!(result);
}

fn apply_action_on_grid(action: Action, range: Range, grid: &mut [[bool; WIDTH]; HEIGHT]) {
    for y in range.f_y..=range.t_y {
        for x in range.f_x..=range.t_x {
            match action {
                Action::TurnOff => grid[y][x] = false,
                Action::TurnOn => grid[y][x] = true,
                Action::Toggle => grid[y][x] = !grid[y][x],
            }
        }
    }
}

fn get_lights_on(input: Vec<String>) -> i32 {
    let mut grid = [[false; WIDTH]; HEIGHT];
    for line in input {
        let (action, range) = get_action_and_range(&line).unwrap().1;
        apply_action_on_grid(action, range, &mut grid);
    }

    grid.iter().fold(0, |acc, row| {
        row.iter()
            .fold(0, |acc_v, v| if *v { acc_v + 1 } else { acc_v })
            + acc
    })
}

/// Parses "turn off 499,499 through 500,500"
fn get_action_and_range(input: &str) -> IResult<&str, (Action, Range)> {
    let (input, action) = alt((tag("turn on"), tag("turn off"), tag("toggle")))(input)?;
    let action: Action = action.into();
    let (input, range) = preceded(tag(" "), Range::parse)(input)?;
    Ok((input, (action, range)))
}

#[cfg(test)]
mod tests {
    use crate::get_lights_on;

    #[test]
    fn example1() {
        let test = vec!["turn on 0,0 through 999,999".to_owned()];
        assert_eq!(get_lights_on(test), 1000000);
    }
}
