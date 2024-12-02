use advent2024_rs::{day1, day2, get_input};
use std::io::{stdin, stdout, BufRead, Write};

fn main() -> anyhow::Result<()> {
    print!("Please input a day: ");
    stdout().flush()?;
    let mut input = String::new();
    stdin().lock().read_line(&mut input)?;

    let day = input.trim().parse::<u32>()?;
    let lines = get_input(format!("./inputs/day{}.txt", day))?;

    let (part1, part2) = match day {
        1 => (day1::part1(&lines), day1::part2(&lines)),
        2 => (day2::part1(&lines), day2::part2(&lines)),
        _ => (String::new(), String::new()),
    };

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
