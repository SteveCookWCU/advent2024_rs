use std::collections::HashMap;

pub fn part1(input: &[impl AsRef<str>]) -> String {
    let (mut first_list, mut second_list) = parse_lists(input);

    first_list.sort();
    second_list.sort();

    let mut sum = 0;

    for i in 0..first_list.len() {
        sum += (first_list[i] - second_list[i]).abs();
    }

    sum.to_string()
}

pub fn part2(input: &[impl AsRef<str>]) -> String {
    let (left_list, right_list) = parse_lists(input);
    let mut map = HashMap::new();
    for first in &left_list {
        if !map.contains_key(first) {
            let mut count = 0;
            for second in &right_list {
                if first == second {
                    count += 1;
                }
            }
            map.insert(first, count);
        }
    }

    let mut sum = 0;

    for first in &left_list {
        sum += first * map.get(&first).unwrap();
    }

    sum.to_string()
}

fn parse_lists(input: &[impl AsRef<str>]) -> (Vec<i32>, Vec<i32>) {
    let mut first_list = Vec::with_capacity(input.len());
    let mut second_list = Vec::with_capacity(input.len());
    for line in input.iter().map(AsRef::as_ref) {
        let (first, second) = line.split_once("   ").unwrap();
        let first = first.parse::<i32>().unwrap();
        let second = second.parse::<i32>().unwrap();
        first_list.push(first);
        second_list.push(second);
    }
    (first_list, second_list)
}
