use std::collections::HashSet;

fn main() {
    let input = common::utils::lines_from_file("inputs/day8.txt")
        .into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|o| o.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    scan_h(&input, &mut visible, 0..input[0].len());
    scan_h(&input, &mut visible, (0..input[0].len()).rev());
    scan_v(&input, &mut visible, 0..input.len());
    scan_v(&input, &mut visible, (0..input.len()).rev());
    println!("Amount of visible trees: {}", visible.len());
}

fn scan_h<T>(input: &[Vec<i32>], visible: &mut HashSet<(usize, usize)>, col_range: T)
where
    T: IntoIterator<Item = usize> + Clone,
{
    let mut init_height = -1;
    for row in 0..input.len() {
        for col in col_range.clone() {
            if input[row][col] > init_height {
                if visible.insert((col, row)) {
                    println!("Inserting: {:?}", (col, row));
                } else {
                    println!("{:?} allready found", (col, row));
                }
                init_height = input[row][col];
            }
        }
        init_height = -1
    }
}

fn scan_v<T>(input: &Vec<Vec<i32>>, visible: &mut HashSet<(usize, usize)>, row_range: T)
where
    T: IntoIterator<Item = usize> + Clone,
{
    let mut init_height = -1;
    for col in 0..input[0].len() {
        for row in row_range.clone() {
            if input[row][col] > init_height {
                if visible.insert((col, row)) {
                    println!("Inserting: {:?}", (col, row));
                } else {
                    println!("{:?} allready found", (col, row));
                }
                init_height = input[row][col];
            }
        }
        init_height = -1
    }
}
