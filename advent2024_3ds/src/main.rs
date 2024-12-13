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
use advent2024_core::Day;
#[cfg(target_os = "horizon")]
use ctru::prelude::*;
use std::time::Instant;

const MAX_DAY: u8 = 3;

#[cfg(target_os = "horizon")]
fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let console = Console::new(gfx.top_screen.borrow_mut());

    let mut day = 1;
    let (mut p1, mut p2, mut t) = (String::new(), String::new(), 0.0);

    while apt.main_loop() {
        println!("\x1b[0;0HPress START to exit");
        println!("Press SELECT to run");
        hid.scan_input();
        let keys = hid.keys_down();
        if keys.contains(KeyPad::R) {
            day += 1;
        } else if keys.contains(KeyPad::L) {
            day -= 1;
        } else if keys.contains(KeyPad::START) {
            break;
        } else if keys.contains(KeyPad::SELECT) {
            let start = Instant::now();
            (p1, p2) = match day {
                1 => Day1::run(&get_input(include_str!("../../inputs/day1.txt"))),
                2 => Day2::run(&get_input(include_str!("../../inputs/day2.txt"))),
                3 => Day3::run(&get_input(include_str!("../../inputs/day3.txt"))),
                4 => Day4::run(&get_input(include_str!("../../inputs/day4.txt"))),
                5 => Day5::run(&get_input(include_str!("../../inputs/day5.txt"))),
                6 => Day6::run(&get_input(include_str!("../../inputs/day6.txt"))),
                7 => Day7::run(&get_input(include_str!("../../inputs/day7.txt"))),
                8 => Day8::run(&get_input(include_str!("../../inputs/day8.txt"))),
                9 => Day9::run(&get_input(include_str!("../../inputs/day9.txt"))),
                10 => Day10::run(&get_input(include_str!("../../inputs/day10.txt"))),
                11 => Day11::run(&get_input(include_str!("../../inputs/day11.txt"))),
                12 => Day12::run(&get_input(include_str!("../../inputs/day12.txt"))),
                13 => Day13::run(&get_input(include_str!("../../inputs/day13.txt"))),
                _ => (String::new(), String::new()),
            };
            t = start.elapsed().as_secs_f32();
            console.clear();
        }
        println!("\x1b[3;0H{:0>2}", day);
        if !p1.is_empty() || !p2.is_empty() {
            println!("\x1b[4;0HPart 1: {}", p1);
            println!("Part 2: {}", p2);
            println!("Time: {}s", t)
        }
        gfx.wait_for_vblank();
    }
}

fn get_input(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<_>>()
}

#[cfg(not(target_os = "horizon"))]
fn main() {}
