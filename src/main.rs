use advent2024_rs::{day1, get_input};
use std::io::{stdin, BufRead};

fn main() -> anyhow::Result<()> {
    print!("Please input a day: ");
    let mut input = String::new();
    stdin().lock().read_line(&mut input)?;

    let day = input.parse::<u32>()?;
    let lines = get_input(format!("./inputs/day{}.txt", day))?;

    let (part1, part2) = match day {
        1 => (day1::part1(&lines), day1::part2(&lines)),
        _ => (String::new(), String::new()),
    };

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
