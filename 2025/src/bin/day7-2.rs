use common::strings::string_to_char_grid;
use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    time::Instant,
};

fn main() {
    let now = Instant::now();
    let res = solve(include_str!("../../inputs/day7.txt"));
    let elapsed = now.elapsed();
    dbg!(res);
    println!("Time used {elapsed:?}");
}

fn solve(input: &str) -> String {
    let g = string_to_char_grid(input);

    let start_col = g[0].iter().position(|c| *c == 'S').unwrap();

    let mut connections: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut rev_connections: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    let mut stack = vec![(0, start_col, (0, start_col))];
    let mut visited = HashSet::new();
    while let Some((row, col, split_from)) = stack.pop() {
        // Split already visited
        if visited.contains(&(row, col)) {
            connections
                .entry(split_from)
                .or_insert(vec![])
                .push((row, col));
            rev_connections
                .entry((row, col))
                .or_insert(vec![])
                .push(split_from);
            continue;
        }

        if row >= g.len() {
            connections
                .entry(split_from)
                .or_insert(vec![])
                .push((row, col));
            rev_connections
                .entry((row, col))
                .or_insert(vec![])
                .push(split_from);
            continue;
        }

        if g[row][col] == '^' {
            visited.insert((row, col));
            connections
                .entry(split_from)
                .or_insert(vec![])
                .push((row, col));
            rev_connections
                .entry((row, col))
                .or_insert(vec![])
                .push(split_from);

            if col > 0 {
                stack.push((row, col - 1, (row, col)));
            }
            if col < g[0].len() {
                stack.push((row, col + 1, (row, col)));
            }
        } else {
            stack.push((row + 1, col, split_from))
        }
    }

    // bfs to count total, maybe to graph not nessecary
    let root_connections = connections.get(&(0, start_col)).unwrap();
    let mut queue = BTreeMap::new();
    for connection in root_connections {
        queue.insert(0, connection);
    }

    let mut counts = HashMap::new();
    counts.insert((0, start_col), 1);
    let mut count: usize = 0;

    while let Some((_depth, curr)) = queue.pop_first() {
        let mut local_count: usize = 0;
        for rev_c in rev_connections.get(curr).unwrap() {
            local_count += counts
                .get(rev_c)
                .unwrap_or_else(|| panic!("{:?} not in count:\n{:?}", rev_c, counts));
        }
        counts.insert(*curr, local_count);

        if let Some(next_connections) = connections.get(curr) {
            for next_connection in next_connections {
                queue.insert(
                    next_connection.0 * 1000 + next_connection.1,
                    next_connection,
                );
            }
        } else {
            count += local_count
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ti = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(solve(ti), "40".to_string());
    }
}
