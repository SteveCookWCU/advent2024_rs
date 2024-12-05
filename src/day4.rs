use crate::Day;

pub struct Day4;

impl Day for Day4 {
    type Input = Vec<Vec<char>>;

    fn part1(input: Self::Input) -> String {
        let mut count = 0;
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] == 'X' {
                    if y >= 3 {
                        if input[y - 1][x] == 'M'
                            && input[y - 2][x] == 'A'
                            && input[y - 3][x] == 'S'
                        {
                            count += 1;
                        }
                        if x >= 3
                            && input[y - 1][x - 1] == 'M'
                            && input[y - 2][x - 2] == 'A'
                            && input[y - 3][x - 3] == 'S'
                        {
                            count += 1;
                        }
                        if x < input[y].len() - 3
                            && input[y - 1][x + 1] == 'M'
                            && input[y - 2][x + 2] == 'A'
                            && input[y - 3][x + 3] == 'S'
                        {
                            count += 1;
                        }
                    }
                    if x >= 3
                        && input[y][x - 1] == 'M'
                        && input[y][x - 2] == 'A'
                        && input[y][x - 3] == 'S'
                    {
                        count += 1;
                    }
                    if y < input[y].len() - 3 {
                        if input[y + 1][x] == 'M'
                            && input[y + 2][x] == 'A'
                            && input[y + 3][x] == 'S'
                        {
                            count += 1;
                        }
                        if x >= 3
                            && input[y + 1][x - 1] == 'M'
                            && input[y + 2][x - 2] == 'A'
                            && input[y + 3][x - 3] == 'S'
                        {
                            count += 1;
                        }
                        if x < input[y].len() - 3
                            && input[y + 1][x + 1] == 'M'
                            && input[y + 2][x + 2] == 'A'
                            && input[y + 3][x + 3] == 'S'
                        {
                            count += 1;
                        }
                    }
                    if x < input[y].len() - 3
                        && input[y][x + 1] == 'M'
                        && input[y][x + 2] == 'A'
                        && input[y][x + 3] == 'S'
                    {
                        count += 1;
                    }
                }
            }
        }
        count.to_string()
    }

    fn part2(input: Self::Input) -> String {
        let mut count = 0;
        for y in 1..input.len() - 1 {
            for x in 1..input[y].len() - 1 {
                if input[y][x] == 'A' {
                    let tl_br = [input[y - 1][x - 1], input[y + 1][x + 1]];
                    let bl_tr = [input[y + 1][x - 1], input[y - 1][x + 1]];
                    if tl_br.contains(&'M')
                        && tl_br.contains(&'S')
                        && bl_tr.contains(&'M')
                        && bl_tr.contains(&'S')
                    {
                        count += 1;
                    }
                }
            }
        }
        count.to_string()
    }

    fn parse_input(input: &[impl AsRef<str>]) -> Self::Input {
        input
            .iter()
            .map(|line| line.as_ref().chars().collect())
            .collect()
    }
}
