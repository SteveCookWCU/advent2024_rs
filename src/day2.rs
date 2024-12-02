pub fn part1(input: &[impl AsRef<str>]) -> String {
    let mut sum = 0;
    for line in input.iter().map(AsRef::as_ref) {
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        if valid_line(&numbers) {
            sum += 1;
        }
    }

    sum.to_string()
}

pub fn part2(input: &[impl AsRef<str>]) -> String {
    let mut sum = 0;
    'outer: for line in input.iter().map(AsRef::as_ref) {
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
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

fn valid_line(numbers: &[u32]) -> bool {
    let mut increasing = None;
    numbers.windows(2).all(|pair| {
        if pair[0] == pair[1] {
            false
        } else if let Some(inc) = increasing {
            !(inc && pair[1].wrapping_sub(pair[0]) > 3 || !inc && pair[0].wrapping_sub(pair[1]) > 3)
        } else {
            increasing = Some(pair[0] < pair[1]);
            ((pair[0] as i32) - (pair[1] as i32)).abs() <= 3
        }
    })
}
