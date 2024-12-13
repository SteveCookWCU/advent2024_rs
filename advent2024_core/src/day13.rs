use crate::Day;

pub struct Day13;

#[derive(Clone, Copy)]
pub struct Button {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
pub struct Prize {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
pub struct Game {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

impl Day for Day13 {
    type Input = Vec<Game>;

    fn part1(input: Self::Input) -> String {
        let mut sum = 0;
        for Game {
            button_a: a,
            button_b: b,
            prize: p,
        } in input
        {
            let determinant = (a.x * b.y) - (b.x * a.y);
            let x_cramer_determinant = (p.x * b.y) - (b.x * p.y);
            let y_cramer_determinant = (p.y * a.x) - (a.y * p.x);

            let x_rem = x_cramer_determinant % determinant;
            let y_rem = y_cramer_determinant % determinant;
            let x_quotient = x_cramer_determinant / determinant;
            let y_quotient = y_cramer_determinant / determinant;
            if x_rem == 0 && y_rem == 0 && x_quotient <= 100 && y_quotient <= 100 {
                sum += 3 * x_quotient + y_quotient;
            }
        }
        sum.to_string()
    }

    fn part2(input: Self::Input) -> String {
        let mut sum = 0;
        for Game {
            button_a: a,
            button_b: b,
            prize: mut p,
        } in input
        {
            p.x += 10000000000000;
            p.y += 10000000000000;
            let determinant = (a.x * b.y) - (b.x * a.y);
            let x_dividend = (p.x * b.y) - (b.x * p.y);
            let y_dividend = (p.y * a.x) - (a.y * p.x);

            let x_rem = x_dividend % determinant;
            let y_rem = y_dividend % determinant;
            let x_quotient = x_dividend / determinant;
            let y_quotient = y_dividend / determinant;
            if x_rem == 0 && y_rem == 0 {
                sum += 3 * x_quotient + y_quotient;
            }
        }
        sum.to_string()
    }

    fn parse_input(input: &[impl AsRef<str>]) -> Self::Input {
        input
            .chunks(4)
            .map(|chunk| {
                let button_a = chunk[0]
                    .as_ref()
                    .trim_start_matches("Button A: ")
                    .split_once(", ")
                    .map(|(left, right)| Button {
                        x: left.trim_start_matches("X+").parse::<i64>().unwrap(),
                        y: right.trim_start_matches("Y+").parse::<i64>().unwrap(),
                    })
                    .unwrap();
                let button_b = chunk[1]
                    .as_ref()
                    .trim_start_matches("Button B: ")
                    .split_once(", ")
                    .map(|(left, right)| Button {
                        x: left.trim_start_matches("X+").parse::<i64>().unwrap(),
                        y: right.trim_start_matches("Y+").parse::<i64>().unwrap(),
                    })
                    .unwrap();
                let prize = chunk[2]
                    .as_ref()
                    .trim_start_matches("Prize: ")
                    .split_once(", ")
                    .map(|(left, right)| Prize {
                        x: left.trim_start_matches("X=").parse::<i64>().unwrap(),
                        y: right.trim_start_matches("Y=").parse::<i64>().unwrap(),
                    })
                    .unwrap();
                Game {
                    button_a,
                    button_b,
                    prize,
                }
            })
            .collect::<Vec<_>>()
    }
}
