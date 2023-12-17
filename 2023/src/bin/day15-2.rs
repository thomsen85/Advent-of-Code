use std::collections::HashMap;

fn main() {
    dbg!(solve(include_str!("../../inputs/day15.txt")));
}

fn solve(input: &str) -> String {
    let mut boxes: HashMap<u8, Vec<(String, u8)>> = HashMap::new();
    for line in input.replace("\n", "").split(",") {
        if line.contains("=") {
            let (label, val) = line.split_once("=").unwrap();
            let index = hash(label);
            if let Some(bx) = boxes.get_mut(&index.try_into().unwrap()) {
                if let Some(place) = bx.iter().position(|(labl, _val)| labl == label) {
                    bx[place].1 = val.parse().unwrap();
                } else {
                    bx.push((label.to_string(), val.parse::<u8>().unwrap()));
                }
            } else {
                boxes.insert(index as u8, vec![(label.to_string(), val.parse().unwrap())]);
            }
        } else {
            let label = line.replace("-", "");
            let index = hash(&label);
            if let Some(bx) = boxes.get_mut(&(index as u8)) {
                if let Some(pos) = bx.iter().position(|(labl, _val)| labl == &label) {
                    bx.remove(pos);
                }
            }
        }
    }

    boxes
        .iter()
        .flat_map(|(key, val)| {
            val.iter()
                .enumerate()
                .map(move |(index, (_label, lence))| (key, index + 1, lence))
        })
        .fold(0, |acc, (box_nr, index, lence)| {
            acc + (*box_nr as usize + 1) * index * (*lence as usize)
        })
        .to_string()
}

fn hash(seq: &str) -> u32 {
    seq.chars()
        .map(|c| c as u32)
        .fold(0, |acc, x| ((x + acc) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve(ti), "1320".to_string());
    }
}
