use crate::Day;

pub struct Day2;

impl Day for Day2 {
    type Input = Vec<Vec<u32>>;

    fn part1(input: Self::Input) -> String {
        let mut sum = 0;
        for numbers in input {
            if valid_line(&numbers) {
                sum += 1;
            }
        }
        sum.to_string()
    }

    fn part2(input: Self::Input) -> String {
        let mut sum = 0;
        'outer: for numbers in input {
            if valid_line(&numbers) {
                sum += 1;
            } else {
                for index in 0..numbers.len() {
                    let numbers = numbers
                        .iter()
                        .enumerate()
                        .filter_map(|(i, x)| if i == index { None } else { Some(*x) })
                        .collect::<Vec<_>>();
                    if valid_line(&numbers) {
                        sum += 1;
                        continue 'outer;
                    }
                }
            }
        }

        sum.to_string()
    }

    fn parse_input(input: &[impl AsRef<str>]) -> Self::Input {
        input
            .iter()
            .map(|line| {
                line.as_ref()
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect()
            })
            .collect()
    }
}

fn valid_line(numbers: &[u32]) -> bool {
    let mut increasing = None;
    numbers.windows(2).all(|pair| {
        if pair[0] == pair[1] || pair[0].abs_diff(pair[1]) > 3 {
            false
        } else if let Some(inc) = increasing {
            !(inc && pair[0] > pair[1] || !inc && pair[1] > pair[0])
        } else {
            increasing = Some(pair[0] < pair[1]);
            true
        }
    })
}
