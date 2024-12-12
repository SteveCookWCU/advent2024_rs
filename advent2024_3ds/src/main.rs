use advent2024_core::day1::Day1;
use advent2024_core::day2::Day2;
use advent2024_core::day3::Day3;
use advent2024_core::Day;
#[cfg(target_os = "horizon")]
use ctru::prelude::*;

const MAX_DAY: u8 = 3;

#[cfg(target_os = "horizon")]
fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let console = Console::new(gfx.top_screen.borrow_mut());

    let mut day = 1;
    let (mut p1, mut p2) = (String::new(), String::new());
    println!("Press START to exit");
    println!("Press SELECT to run");
    while apt.main_loop() {
        hid.scan_input();
        let keys = hid.keys_down();
        if keys.contains(KeyPad::R) {
            day += 1;
        } else if keys.contains(KeyPad::L) {
            day -= 1;
        } else if keys.contains(KeyPad::START) {
            break;
        } else if keys.contains(KeyPad::SELECT) {
            (p1, p2) = match day {
                1 => {
                    Day1::run(&get_input(include_str!("../../inputs/day1.txt")))
                }
                2 => {
                    Day2::run(&get_input(include_str!("../../inputs/day2.txt")))
                }
                3 => {
                    Day3::run(&get_input(include_str!("../../inputs/day3.txt")))
                }
                _ => {
                    (String::new(), String::new())
                }
            }
        }
        println!("\x1b[3;0H{:0>2}", day);
        if !p1.is_empty() || !p2.is_empty() {
            println!("Part 1: {}", p1);
            println!("Part 2: {}", p2);
        }
        gfx.wait_for_vblank();
    }
}

fn get_input(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<_>>()
}

#[cfg(not(target_os = "horizon"))]
fn main() {}
