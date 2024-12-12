use crate::Day;
use std::collections::HashSet;

pub struct Day10;

impl Day for Day10 {
    type Input = Vec<Vec<u8>>;

    fn part1(input: Self::Input) -> String {
        let mut total = 0;
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x] == 0 {
                    let mut score = 0;
                    let mut stack = Vec::new();
                    let mut checked = HashSet::new();
                    stack.push((x, y));
                    while let Some((x, y)) = stack.pop() {
                        let next = input[y][x] + 1;
                        checked.insert((x, y));
                        if input[y][x] == 9 {
                            score += 1;
                            continue;
                        }
                        if y > 0 && !checked.contains(&(x, y - 1)) && input[y - 1][x] == next {
                            stack.push((x, y - 1));
                        }
                        if x > 0 && !checked.contains(&(x - 1, y)) && input[y][x - 1] == next {
                            stack.push((x - 1, y));
                        }
                        if y < input.len() - 1
                            && !checked.contains(&(x, y + 1))
                            && input[y + 1][x] == next
                        {
                            stack.push((x, y + 1));
                        }
                        if x < input[0].len() - 1
                            && !checked.contains(&(x + 1, y))
                            && input[y][x + 1] == next
                        {
                            stack.push((x + 1, y));
                        }
                    }
                    total += score;
                }
            }
        }
        total.to_string()
    }

    fn part2(input: Self::Input) -> String {
        let mut total = 0;
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x] == 0 {
                    let mut score = 0;
                    let mut stack = Vec::new();
                    let mut checked = HashSet::new();
                    stack.push((x, y));
                    while let Some((x, y)) = stack.pop() {
                        let next = input[y][x] + 1;
                        checked.insert((x, y));
                        if input[y][x] == 9 {
                            score += 1;
                            continue;
                        }
                        if y > 0 && input[y - 1][x] == next {
                            stack.push((x, y - 1));
                        }
                        if x > 0 && input[y][x - 1] == next {
                            stack.push((x - 1, y));
                        }
                        if y < input.len() - 1 && input[y + 1][x] == next {
                            stack.push((x, y + 1));
                        }
                        if x < input[0].len() - 1 && input[y][x + 1] == next {
                            stack.push((x + 1, y));
                        }
                    }
                    total += score;
                }
            }
        }
        total.to_string()
    }

    fn parse_input(input: &[impl AsRef<str>]) -> Self::Input {
        input
            .iter()
            .map(|s| {
                s.as_ref()
                    .chars()
                    .map(|c| (c as u8).wrapping_sub(b'0'))
                    .collect()
            })
            .collect()
    }
}
