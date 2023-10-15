#![allow(dead_code)]

const ROW_LENGTH: usize = 7;
type Piece = &'static [&'static [u8]];
type Row = [u8; ROW_LENGTH];

const ITERS: i32 = 2022;

const H_LONG_PIECE: Piece = &[&[1, 1, 1, 1]];
const PLUS_PIECE: Piece = &[&[0, 1, 0], &[1, 1, 1], &[0, 1, 0]];
const L_PIECE: Piece = &[&[0, 0, 1], &[0, 0, 1], &[1, 1, 1]];
const V_LONG_PIECE: Piece = &[&[1], &[1], &[1], &[1]];
const BOX_PIECE: Piece = &[&[1, 1], &[1, 1]];

const PIECE_ORDER: [Piece; 5] = [H_LONG_PIECE, PLUS_PIECE, L_PIECE, V_LONG_PIECE, BOX_PIECE];
fn main() {
    let input = common::utils::string_from_file("inputs/day17.txt")
        .trim()
        .to_owned();
    dbg!(input.chars().collect::<Vec<char>>());
    dbg!(get_height_of_sim(input));
}

fn print_map(map: &Vec<Row>, piece: Piece, p_x: usize, p_y: usize) {
    let height = map.len().max(piece.len() + p_y);
    for y in (1..height).rev() {
        let mut line = String::new();
        for x in 0..ROW_LENGTH {
            if let Some(row) = map.get(y) {
                if row[x] == 1 {
                    line.push('#');
                    continue;
                }
            }
            if y >= p_y && x >= p_x {
                if let Some(p_row) = piece.get(y - p_y) {
                    if let Some(val) = p_row.get(x - p_x) {
                        if *val == 1 {
                            line.push('@');
                            continue;
                        }
                    }
                }
            }
            line.push('.');
        }
        println!("|{}|", line);
    }

    println!("+{}+\n", "-".repeat(ROW_LENGTH));
}

fn get_height_of_sim(input: String) -> usize {
    let mut highest_point = 0;
    let mut piece_index = 0;
    let mut jet_index = 0;

    let mut rocks_settled = 0;
    let mut map: Vec<Row> = Vec::from([[1; ROW_LENGTH], [0; ROW_LENGTH], [0; ROW_LENGTH]]);

    let mut current_x = 2;
    let mut current_y = 5;

    while rocks_settled < 2022 {
        // Check if settable
        if map_and_piece_overlaps(&map, PIECE_ORDER[piece_index], current_x, current_y - 1) {
            insert_piece_on_map(&mut map, PIECE_ORDER[piece_index], current_x, current_y);
            current_x = 2;
            highest_point = (current_y + PIECE_ORDER[piece_index].len()).max(highest_point);
            current_y = highest_point + 3;
            rocks_settled += 1;
            piece_index = (piece_index + 1) % PIECE_ORDER.len();
            // dbg!(rocks_settled);
            // print_map(&map, PIECE_ORDER[piece_index], current_x, current_y);
        } else {
            // Fall
            current_y -= 1;
        }

        // Push
        if input.chars().nth(jet_index).unwrap() == '<'
            && current_x > 0
            && !map_and_piece_overlaps(&map, PIECE_ORDER[piece_index], current_x - 1, current_y)
        {
            current_x -= 1;
        } else if input.chars().nth(jet_index).unwrap() == '>'
            && (current_x + PIECE_ORDER[piece_index][0].len()) < ROW_LENGTH
            && !map_and_piece_overlaps(&map, PIECE_ORDER[piece_index], current_x + 1, current_y)
        {
            current_x += 1;
        }
        jet_index = (jet_index + 1) % input.len();

        // print_map(&map, PIECE_ORDER[piece_index], current_x, current_y);
    }

    current_y - 4
}

fn map_and_piece_overlaps(map: &Vec<Row>, piece: Piece, p_x: usize, p_y: usize) -> bool {
    for y in 0..piece.len() {
        for x in 0..piece[0].len() {
            if piece[piece.len() - 1 - y][x] == 1 {
                if let Some(row) = map.get(y + p_y) {
                    if row[x + p_x] == 1 {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn insert_piece_on_map(map: &mut Vec<Row>, piece: Piece, x: usize, y: usize) {
    let required_height = y + piece.len();
    if required_height > map.len() {
        for _ in 0..(required_height - map.len()) {
            map.push([0; ROW_LENGTH]);
        }
    }

    for (y_offset, row) in piece.iter().rev().enumerate() {
        for (x_offset, val) in row.iter().enumerate() {
            if *val == 0 {
                continue;
            }

            map[y + y_offset][x + x_offset] = *val;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::insert_piece_on_map;

    use super::*;

    #[test]
    fn example_1() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_owned();
        assert_eq!(get_height_of_sim(input), 3068);
    }

    #[test]
    fn test_is_overlapping() {
        let map: Vec<Row> = Vec::from([[1; 7], [0; 7], [0; 7]]);
        assert!(map_and_piece_overlaps(&map, PLUS_PIECE, 2, 0));
        assert!(!map_and_piece_overlaps(&map, PLUS_PIECE, 2, 1));

        let map: Vec<Row> = Vec::from([[1; 7], [1, 0, 0, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0, 0]]);
        assert!(map_and_piece_overlaps(&map, PLUS_PIECE, 0, 1));
        assert!(!map_and_piece_overlaps(&map, PLUS_PIECE, 0, 2));
    }

    #[test]
    fn test_insert_piece_on_map() {
        let mut map = Vec::new();
        insert_piece_on_map(&mut map, PLUS_PIECE, 4, 3);

        let expected_map = vec![
            [0; ROW_LENGTH],
            [0; ROW_LENGTH],
            [0; ROW_LENGTH],
            [0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 1, 1, 1],
            [0, 0, 0, 0, 0, 1, 0],
        ];

        assert_eq!(map, expected_map);
    }
}
