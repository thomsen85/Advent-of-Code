fn main() {
    let input = aoc2022_rust::utils::lines_from_file("inputs/day8.txt")
        .into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|o| o.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut res: Vec<usize> = Vec::new();
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            res.push(get_scenic_score(&input, col, row))
        }
    }
    println!("Largest scenic score: {}", res.iter().max().unwrap());
}

fn get_scenic_score(input: &Vec<Vec<i32>>, col: usize, row: usize) -> usize {
    let height = input[row][col];
    let mut scores: Vec<usize> = Vec::new();
    // Up
    for i in 1..=row {
        if input[row - i][col] >= height || i == row {
            scores.push(i);
            break;
        }
    }
    // Down
    for i in 1..(input.len() - row) {
        if input[row + i][col] >= height || i == input.len() - row - 1 {
            scores.push(i);
            break;
        }
    }
    // Left
    for i in 1..=col {
        if input[row][col - i] >= height || i == col {
            scores.push(i);
            break;
        }
    }
    // Right
    for i in 1..(input[0].len() - col) {
        if input[row][col + i] >= height || i == input[0].len() - col - 1 {
            scores.push(i);
            break;
        }
    }

    scores.iter().product()
}
