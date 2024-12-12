use crate::Day;
use std::collections::HashSet;
use std::time::Instant;

pub struct Day6;

impl Day for Day6 {
    type Input = ((usize, usize), Vec<Vec<char>>);

    fn part1(input: Self::Input) -> String {
        let mut positions = HashSet::new();
        let ((y, x), grid) = input;
        _ = walk_grid(y, x, &grid, &mut positions, false);

        positions
            .into_iter()
            .map(|(y, x, _)| (y, x))
            .collect::<HashSet<_>>()
            .len()
            .to_string()
    }

    fn part2(input: Self::Input) -> String {
        let start = Instant::now();
        let mut count = 0;
        let ((y, x), mut grid) = input;

        let mut positions = HashSet::new();
        _ = walk_grid(y, x, &grid, &mut positions, false);
        let original_path = positions
            .into_iter()
            .filter_map(|(y2, x2, _)| {
                if x == x2 && y == y2 {
                    None
                } else {
                    Some((y2, x2))
                }
            })
            .collect::<HashSet<_>>();

        // zug zug
        for (y2, x2) in original_path {
            let mut positions = HashSet::new();
            grid[y2][x2] = '#';
            if !walk_grid(y, x, &grid, &mut positions, true) {
                count += 1;
            }
            grid[y2][x2] = '.';
        }
        count.to_string()
    }

    fn parse_input(input: &[impl AsRef<str>]) -> Self::Input {
        let mut start_x = 0;
        let mut start_y = 0;

        let grid = input
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.as_ref()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '^' {
                            start_y = y;
                            start_x = x;
                        }
                        c
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        ((start_y, start_x), grid)
    }
}

fn walk_grid(
    mut y: usize,
    mut x: usize,
    grid: &[Vec<char>],
    positions: &mut HashSet<(usize, usize, Direction)>,
    check: bool,
) -> bool {
    let mut direction = Direction::North;
    positions.insert((y, x, direction));
    while (1..grid.len() - 1).contains(&y) && (1..grid[0].len() - 1).contains(&x) {
        match direction {
            Direction::North => {
                if y > 0 {
                    if grid[y - 1][x] == '#' {
                        direction = Direction::East;
                    } else {
                        y -= 1;
                    }
                }
            }
            Direction::South => {
                if y < grid.len() - 1 {
                    if grid[y + 1][x] == '#' {
                        direction = Direction::West;
                    } else {
                        y += 1;
                    }
                }
            }
            Direction::East => {
                if x < grid[0].len() - 1 {
                    if grid[y][x + 1] == '#' {
                        direction = Direction::South;
                    } else {
                        x += 1;
                    }
                }
            }
            Direction::West => {
                if x > 0 {
                    if grid[y][x - 1] == '#' {
                        direction = Direction::North;
                    } else {
                        x -= 1;
                    }
                }
            }
        }
        if !positions.insert((y, x, direction)) && check {
            return false;
        }
    }
    true
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}
