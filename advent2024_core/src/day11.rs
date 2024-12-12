use crate::Day;
use std::collections::HashMap;

pub struct Day11;

impl Day for Day11 {
    type Input = HashMap<u64, u64>;

    fn part1(mut input: Self::Input) -> String {
        for _ in 0..25 {
            let mut new_map = HashMap::new();
            for (num, count) in input {
                if num == 0 {
                    *new_map.entry(1).or_insert(0) += count;
                } else if ((num.checked_ilog10().unwrap_or(0) + 1) % 2) == 0 {
                    let s = num.to_string();
                    let first = s[..s.len() / 2].parse::<u64>().unwrap();
                    let second = s[s.len() / 2..].parse::<u64>().unwrap();
                    *new_map.entry(second).or_insert(0) += count;
                    *new_map.entry(first).or_insert(0) += count;
                } else {
                    *new_map.entry(num * 2024).or_insert(0) += count;
                }
            }
            input = new_map;
        }
        input.into_values().sum::<u64>().to_string()
    }

    fn part2(mut input: Self::Input) -> String {
        for _ in 0..75 {
            let mut new_map = HashMap::new();
            for (num, count) in input {
                if num == 0 {
                    *new_map.entry(1).or_insert(0) += count;
                } else if ((num.checked_ilog10().unwrap_or(0) + 1) % 2) == 0 {
                    let s = num.to_string();
                    let first = s[..s.len() / 2].parse::<u64>().unwrap();
                    let second = s[s.len() / 2..].parse::<u64>().unwrap();
                    *new_map.entry(second).or_insert(0) += count;
                    *new_map.entry(first).or_insert(0) += count;
                } else {
                    *new_map.entry(num * 2024).or_insert(0) += count;
                }
            }
            input = new_map;
        }
        input.into_values().sum::<u64>().to_string()
    }

    fn parse_input(input: &[impl AsRef<str>]) -> Self::Input {
        let mut map: HashMap<u64, u64> = HashMap::new();
        input[0]
            .as_ref()
            .split(' ')
            .for_each(|s| *map.entry(s.parse().unwrap()).or_insert(0) += 1);
        map
    }
}
