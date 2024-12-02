pub mod day1;
pub mod day2;

use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn get_input<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let f = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(f);
    let mut lines = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        lines.push(line);
    }
    Ok(lines)
}

#[cfg(test)]
fn get_test_input(s: &str) -> Vec<&str> {
    s.lines().collect()
}
