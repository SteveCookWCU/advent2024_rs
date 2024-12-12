use crate::Day;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub struct Day12;

#[derive(Clone)]
pub struct Region {
    area: u64,
    perimeter: u64,
    pos: HashSet<(usize, usize)>,
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Area: {}, Perimeter: {}", self.area, self.perimeter)
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Day for Day12 {
    type Input = Vec<Region>;

    fn part1(input: Self::Input) -> String {
        input
            .into_iter()
            .map(|r| r.perimeter * r.area)
            .sum::<u64>()
            .to_string()
    }

    fn part2(input: Self::Input) -> String {
        let mut total = 0;
        for mut region in input.into_iter() {
            let mut wall_locations = Vec::new();
            region.pos = region
                .pos
                .into_iter()
                .map(|(x, y)| (x + 1, y + 1))
                .collect();
            for (x, y) in region.pos.iter().copied() {
                if !region.pos.contains(&(x - 1, y)) {
                    wall_locations.push((x - 1, y, Direction::Left));
                }
                if !region.pos.contains(&(x + 1, y)) {
                    wall_locations.push((x + 1, y, Direction::Right));
                }
                if !region.pos.contains(&(x, y - 1)) {
                    wall_locations.push((x, y - 1, Direction::Up));
                }
                if !region.pos.contains(&(x, y + 1)) {
                    wall_locations.push((x, y + 1, Direction::Down));
                }
            }

            let mut walls = 0;

            while let Some((x, y, dir)) = wall_locations.pop() {
                if [Direction::Down, Direction::Up].contains(&dir) {
                    let mut rx = x + 1;

                    while let Some(i) = wall_locations.iter().enumerate().find_map(|(i, e)| {
                        if e == &(rx, y, dir) {
                            Some(i)
                        } else {
                            None
                        }
                    }) {
                        wall_locations.remove(i);
                        rx += 1;
                    }
                    if x != 0 {
                        let mut lx = x - 1;
                        while let Some(i) = wall_locations.iter().enumerate().find_map(|(i, e)| {
                            if e == &(lx, y, dir) {
                                Some(i)
                            } else {
                                None
                            }
                        }) {
                            wall_locations.remove(i);
                            if lx == 0 {
                                break;
                            } else {
                                lx -= 1;
                            }
                        }
                    }
                } else {
                    let mut dy = y + 1;

                    while let Some(i) = wall_locations.iter().enumerate().find_map(|(i, e)| {
                        if e == &(x, dy, dir) {
                            Some(i)
                        } else {
                            None
                        }
                    }) {
                        wall_locations.remove(i);
                        dy += 1;
                    }
                    if y != 0 {
                        let mut uy = y - 1;
                        while let Some(i) = wall_locations.iter().enumerate().find_map(|(i, e)| {
                            if e == &(x, uy, dir) {
                                Some(i)
                            } else {
                                None
                            }
                        }) {
                            wall_locations.remove(i);
                            if uy == 0 {
                                break;
                            } else {
                                uy -= 1;
                            }
                        }
                    }
                }
                walls += 1;
            }
            total += walls * region.area;
        }
        total.to_string()
    }

    fn parse_input(input: &[impl AsRef<str>]) -> Self::Input {
        let mut regions = Vec::new();
        let mut counted = HashSet::new();
        let input = input
            .iter()
            .map(|line| line.as_ref().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        for (y, row) in input.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if counted.contains(&(x, y)) {
                    continue;
                }
                let mut stack = Vec::new();
                stack.push((x, y));
                let mut region = Region {
                    area: 0,
                    perimeter: 0,
                    pos: HashSet::new(),
                };
                region.pos.insert((x, y));
                counted.insert((x, y));
                while let Some((x, y)) = stack.pop() {
                    if regions.iter().any(|r: &Region| r.pos.contains(&(x, y))) {
                        continue;
                    }
                    region.pos.insert((x, y));
                    region.area += 1;
                    if y != 0 {
                        if input[y - 1][x] == c {
                            if !region.pos.contains(&(x, y - 1)) {
                                region.pos.insert((x, y - 1));
                                stack.push((x, y - 1));
                            }
                        } else {
                            region.perimeter += 1;
                        }
                    } else {
                        region.perimeter += 1;
                    }
                    if x != 0 {
                        if input[y][x - 1] == c {
                            if !region.pos.contains(&(x - 1, y)) {
                                region.pos.insert((x - 1, y));
                                stack.push((x - 1, y));
                            }
                        } else {
                            region.perimeter += 1;
                        }
                    } else {
                        region.perimeter += 1;
                    }
                    if y < input.len() - 1 {
                        if input[y + 1][x] == c {
                            if !region.pos.contains(&(x, y + 1)) {
                                region.pos.insert((x, y + 1));
                                stack.push((x, y + 1));
                            }
                        } else {
                            region.perimeter += 1;
                        }
                    } else {
                        region.perimeter += 1;
                    }
                    if x < input[y].len() - 1 {
                        if input[y][x + 1] == c {
                            if !region.pos.contains(&(x + 1, y)) {
                                region.pos.insert((x + 1, y));
                                stack.push((x + 1, y));
                            }
                        } else {
                            region.perimeter += 1;
                        }
                    } else {
                        region.perimeter += 1;
                    }
                }
                if region.area != 0 && region.perimeter != 0 {
                    regions.push(region);
                }
            }
        }

        regions
    }
}
