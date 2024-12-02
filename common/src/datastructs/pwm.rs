use std::collections::HashMap;

pub struct PositionWeightMatrix {
    pub matrix: HashMap<String, Vec<usize>>,
}

impl PositionWeightMatrix {
    pub fn from_grid_str_with_column_seperator(input: &str, column_seperator: &str) -> Self {
        let columns = input
            .lines()
            .next()
            .unwrap()
            .split(column_seperator)
            .count();

        let mut matrix = HashMap::new();
        for line in input.lines() {
            for (i, c) in line.split(column_seperator).enumerate() {
                matrix.entry(c.to_string()).or_insert(vec![0; columns])[i] += 1;
            }
        }
        Self { matrix }
    }
    pub fn from_grid_str(input: &str) -> Self {
        let columns = input.lines().next().unwrap().chars().count();

        let mut matrix = HashMap::new();
        for line in input.lines() {
            for (i, c) in line.chars().enumerate() {
                matrix.entry(c.to_string()).or_insert(vec![0; columns])[i] += 1;
            }
        }
        Self { matrix }
    }
}
