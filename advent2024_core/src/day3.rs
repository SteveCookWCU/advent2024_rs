use crate::Day;
use regex::Regex;

pub struct Day3;

impl Day for Day3 {
    type Input = (Vec<u64>, Vec<u64>);

    fn part1(input: Self::Input) -> String {
        input.0.into_iter().chain(input.1).sum::<u64>().to_string()
    }

    fn part2(input: Self::Input) -> String {
        input.0.into_iter().sum::<u64>().to_string()
    }

    fn parse_input(input: &[impl AsRef<str>]) -> Self::Input {
        let regex = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
        let mut enabled = true;
        let mut dos = Vec::new();
        let mut donts = Vec::new();
        regex
            .captures_iter(&input.iter().map(AsRef::as_ref).collect::<Vec<_>>().join(""))
            .for_each(|cap| match &cap[0] {
                "do()" => {
                    enabled = true;
                }
                "don't()" => {
                    enabled = false;
                }
                _ => {
                    if enabled {
                        dos.push(cap[1].parse::<u64>().unwrap() * cap[2].parse::<u64>().unwrap())
                    } else {
                        donts.push(cap[1].parse::<u64>().unwrap() * cap[2].parse::<u64>().unwrap())
                    }
                }
            });
        (dos, donts)
    }
}
