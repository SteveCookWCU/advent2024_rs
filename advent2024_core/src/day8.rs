use crate::Day;
use std::collections::{HashMap, HashSet};

pub struct Day8;

impl Day for Day8 {
    type Input = (i64, i64, HashMap<char, Vec<(i64, i64)>>);

    fn part1(input: Self::Input) -> String {
        let mut antinodes = HashSet::new();
        let (height, width, antennas) = input;
        for (_, mut positions) in antennas {
            let mut used = Vec::with_capacity(positions.len());
            used.push(positions.pop().unwrap());
            while let Some(next_node) = positions.pop() {
                for prev_node in used.iter().copied() {
                    let rise = next_node.0.abs_diff(prev_node.0) as i64;
                    let run = next_node.1.abs_diff(prev_node.1) as i64;
                    let (next_node_mod_y, prev_node_mod_y) = if next_node.0 < prev_node.0 {
                        (-1, 1)
                    } else {
                        (1, -1)
                    };
                    let (next_node_mod_x, prev_node_mod_x) = if next_node.1 < prev_node.1 {
                        (-1, 1)
                    } else {
                        (1, -1)
                    };
                    let (mut y, mut x) = next_node;
                    y += next_node_mod_y * rise;
                    x += next_node_mod_x * run;
                    if (0..height).contains(&y) && (0..width).contains(&x) {
                        antinodes.insert((y, x));
                    }
                    (y, x) = prev_node;
                    y += prev_node_mod_y * rise;
                    x += prev_node_mod_x * run;
                    if (0..height).contains(&y) && (0..width).contains(&x) {
                        antinodes.insert((y, x));
                    }
                }
                used.push(next_node);
            }
        }
        antinodes.len().to_string()
    }

    fn part2(input: Self::Input) -> String {
        let mut antinodes = HashSet::new();
        let (height, width, antennas) = input;
        for positions in antennas.values() {
            for position in positions.iter().copied() {
                antinodes.insert(position);
            }
        }
        for mut positions in antennas.into_values() {
            let mut used = Vec::with_capacity(positions.len());
            used.push(positions.pop().unwrap());
            while let Some(next_node) = positions.pop() {
                for prev_node in used.iter().copied() {
                    let rise = next_node.0.abs_diff(prev_node.0) as i64;
                    let run = next_node.1.abs_diff(prev_node.1) as i64;
                    let (next_node_mod_y, prev_node_mod_y) = if next_node.0 < prev_node.0 {
                        (-1, 1)
                    } else {
                        (1, -1)
                    };
                    let (next_node_mod_x, prev_node_mod_x) = if next_node.1 < prev_node.1 {
                        (-1, 1)
                    } else {
                        (1, -1)
                    };
                    let (mut y, mut x) = next_node;
                    y += next_node_mod_y * rise;
                    x += next_node_mod_x * run;
                    while (0..height).contains(&y) && (0..width).contains(&x) {
                        antinodes.insert((y, x));
                        y += next_node_mod_y * rise;
                        x += next_node_mod_x * run;
                    }
                    (y, x) = prev_node;
                    y += prev_node_mod_y * rise;
                    x += prev_node_mod_x * run;
                    while (0..height).contains(&y) && (0..width).contains(&x) {
                        antinodes.insert((y, x));
                        y += prev_node_mod_y * rise;
                        x += prev_node_mod_x * run;
                    }
                }
                used.push(next_node);
            }
        }
        antinodes.len().to_string()
    }

    fn parse_input(input: &[impl AsRef<str>]) -> Self::Input {
        let mut map = HashMap::new();
        let height = input.len();
        let width = input[0].as_ref().len();
        for (y, row) in input.iter().map(|s| s.as_ref().chars()).enumerate() {
            for (x, c) in row.enumerate() {
                if c != '.' {
                    map.entry(c)
                        .or_insert_with(Vec::new)
                        .push((y as i64, x as i64));
                }
            }
        }
        (height as i64, width as i64, map)
    }
}
