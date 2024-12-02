use advent2024_rs::day1::Day1;
use advent2024_rs::day2::Day2;
use advent2024_rs::day3::Day3;
use advent2024_rs::{get_input, Day};
use std::io::{stdin, stdout, BufRead, Write};

fn main() -> anyhow::Result<()> {
    print!("Please input a day: ");
    stdout().flush()?;
    let mut input = String::new();
    stdin().lock().read_line(&mut input)?;

    let day = input.trim().parse::<u32>()?;
    let lines = get_input(format!("./inputs/day{}.txt", day))?;

    let (part1, part2) = match day {
        1 => Day1::run(&lines),
        2 => Day2::run(&lines),
        3 => Day3::run(&lines),
        _ => (String::new(), String::new()),
    };

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
