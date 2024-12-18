use advent2024_core::day1::Day1;
use advent2024_core::day10::Day10;
use advent2024_core::day11::Day11;
use advent2024_core::day12::Day12;
use advent2024_core::day13::Day13;
use advent2024_core::day2::Day2;
use advent2024_core::day3::Day3;
use advent2024_core::day4::Day4;
use advent2024_core::day5::Day5;
use advent2024_core::day6::Day6;
use advent2024_core::day7::Day7;
use advent2024_core::day8::Day8;
use advent2024_core::day9::Day9;
use advent2024_core::{get_input, Day};
use std::io::{stdin, stdout, BufRead, Write};
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    print!("Please input a day: ");
    stdout().flush()?;
    let mut input = String::new();
    stdin().lock().read_line(&mut input)?;

    let day = input.trim().parse::<u32>()?;
    let lines = get_input(format!("../inputs/day{}.txt", day))?;
    let start = Instant::now();
    let (part1, part2) = match day {
        1 => Day1::run(&lines),
        2 => Day2::run(&lines),
        3 => Day3::run(&lines),
        4 => Day4::run(&lines),
        5 => Day5::run(&lines),
        6 => Day6::run(&lines),
        7 => Day7::run(&lines),
        8 => Day8::run(&lines),
        9 => Day9::run(&lines),
        10 => Day10::run(&lines),
        11 => Day11::run(&lines),
        12 => Day12::run(&lines),
        13 => Day13::run(&lines),
        _ => (String::new(), String::new()),
    };
    let t = start.elapsed().as_secs_f32();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Time: {}s", t);

    Ok(())
}
