use itertools::Itertools;
use std::{collections::HashMap, time::Instant};
fn main() {
    let now = Instant::now();
    let res = solve(include_str!("../../inputs/day8.txt"));
    let elapsed = now.elapsed();
    dbg!(res);
    println!("Time used {elapsed:?}");
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct JunctionBox {
    x: isize,
    y: isize,
    z: isize,
}

impl TryFrom<&str> for JunctionBox {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let [x, y, z] = value
            .split(",")
            .map(|n| n.parse::<isize>().unwrap())
            .collect_vec()
            .try_into()
            .unwrap();
        Ok(Self { x, y, z })
    }
}

impl JunctionBox {
    fn dist_to(&self, other: &Self) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f32)
            .sqrt()
    }
}

fn solve(input: &str) -> String {
    let boxes = input
        .trim()
        .split("\n")
        .map(|a| a.try_into().unwrap())
        .collect::<Vec<JunctionBox>>();

    let mut all_unique_pairs = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, box_1)| boxes.iter().skip(i + 1).map(|box_2| (*box_1, *box_2)))
        .collect::<Vec<(JunctionBox, JunctionBox)>>();

    all_unique_pairs.sort_by(|a, b| a.0.dist_to(&a.1).partial_cmp(&b.0.dist_to(&b.1)).unwrap());

    all_unique_pairs.reverse();
    let mut circuits: HashMap<usize, Vec<JunctionBox>> = HashMap::new();
    let mut curr_id = 0;
    let mut assigned_circuit: HashMap<JunctionBox, usize> = HashMap::new();

    let mut last = None;
    while let Some((a, b)) = all_unique_pairs.pop() {
        let a_e = assigned_circuit.get(&a);
        let b_e = assigned_circuit.get(&b);

        if circuits.len() == 1 && assigned_circuit.len() == boxes.len() {
            break;
        }
        last = Some((a, b));

        if a_e.is_some() && b_e.is_some() {
            let a_id = *a_e.unwrap();
            let b_id = *b_e.unwrap();
            if a_id != b_id {
                let mut b_boxes = circuits.remove(&b_id).unwrap();

                for b in &b_boxes {
                    assigned_circuit.insert(*b, a_id);
                }
                circuits.get_mut(&a_id).unwrap().append(&mut b_boxes);
            }
            continue;
        }

        if a_e.is_none() && b_e.is_none() {
            circuits.insert(curr_id, vec![a, b]);
            assigned_circuit.insert(b, curr_id);
            assigned_circuit.insert(a, curr_id);
            curr_id += 1
        } else if let Some(id) = a_e {
            circuits.get_mut(id).unwrap().push(b);
            assigned_circuit.insert(b, *id);
        } else if let Some(id) = b_e {
            circuits.get_mut(id).unwrap().push(a);
            assigned_circuit.insert(a, *id);
        }
    }

    (last.unwrap().0.x * last.unwrap().1.x).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(solve(ti), "25272".to_string());
    }
}
