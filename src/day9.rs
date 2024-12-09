use crate::Day;

pub struct Day9;

#[derive(Copy, Clone, Debug)]
pub enum Fragment {
    File { id: usize, len: usize },
    Free { len: usize },
}

impl Fragment {
    pub fn is_file(&self) -> bool {
        match self {
            Fragment::File { .. } => true,
            Fragment::Free { .. } => false,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            &Fragment::File { len, .. } => len,
            &Fragment::Free { len, .. } => len,
        }
    }
}

impl Day for Day9 {
    type Input = Vec<Fragment>;

    fn part1(mut input: Self::Input) -> String {
        let mut first_free = input
            .iter()
            .enumerate()
            .find(|&(_, f)| !f.is_file())
            .map(|(i, _)| i)
            .unwrap();
        let mut curr;
        let mut curr_index;
        'outer: while {
            while {
                curr = input.pop().unwrap();
                curr_index = input.len();
                if curr_index == first_free {
                    break 'outer;
                }
                !curr.is_file()
            } {}
            true
        } {
            match curr {
                Fragment::File { id, len } => {
                    let file_len = len;
                    match input[first_free] {
                        Fragment::Free { len } => {
                            if len > file_len {
                                input[first_free] = Fragment::File { id, len: file_len };
                                input.insert(
                                    first_free + 1,
                                    Fragment::Free {
                                        len: len - file_len,
                                    },
                                );
                                first_free += 1;
                            } else {
                                if len == file_len {
                                    input[first_free] = Fragment::File { id, len };
                                } else {
                                    input[first_free] = Fragment::File { id, len };
                                    input.push(Fragment::File {
                                        id,
                                        len: file_len - len,
                                    });
                                }
                                if let Some(i) = input
                                    .iter()
                                    .enumerate()
                                    .skip(first_free)
                                    .find(|&(_, f)| !f.is_file())
                                    .map(|(i, _)| i)
                                {
                                    first_free = i;
                                } else {
                                    break 'outer;
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        let mut sum = 0;
        let mut idx = 0;
        for frag in input {
            match frag {
                Fragment::File { id, len } => {
                    for block_pos in idx..(idx + len) {
                        sum += block_pos * id;
                    }
                    idx += len;
                }
                _ => unreachable!(),
            }
        }

        sum.to_string()
    }

    fn part2(mut input: Self::Input) -> String {
        let mut first_free = input
            .iter()
            .enumerate()
            .find(|&(_, f)| !f.is_file())
            .map(|(i, &f)| (i, f));
        while let Some((i, free)) = first_free {
            let fit = input
                .iter()
                .enumerate()
                .skip(i + 1)
                .rfind(|&(_, f)| f.is_file() && f.len() <= free.len())
                .map(|(i, &f)| (i, f));
            if let Some((j, file)) = fit {
                if file.len() == free.len() {
                    input.swap(i, j)
                } else {
                    input[j] = Fragment::Free { len: file.len() };
                    input[i] = file;
                    let left_over = Fragment::Free {
                        len: free.len() - file.len(),
                    };
                    input.insert(i + 1, left_over);
                    first_free = Some((i + 1, left_over));
                    continue;
                }
            }
            first_free = input
                .iter()
                .enumerate()
                .skip(i + 1)
                .find(|&(_, f)| !f.is_file())
                .map(|(i, &f)| (i, f))
        }
        let mut sum = 0;
        let mut idx = 0;
        for frag in input {
            match frag {
                Fragment::File { id, len } => {
                    for block_pos in idx..(idx + len) {
                        sum += block_pos * id;
                    }
                }
                _ => {}
            }
            idx += frag.len()
        }

        sum.to_string()
    }

    fn parse_input(input: &[impl AsRef<str>]) -> Self::Input {
        let input = input[0]
            .as_ref()
            .chars()
            .map(|c| (c as u8 - b'0') as usize)
            .enumerate();
        let mut disk = Vec::new();
        let mut id = 0;
        for (i, len) in input {
            if i % 2 == 0 {
                if len != 0 {
                    disk.push(Fragment::File { id, len });
                    id += 1;
                }
            } else if len != 0 {
                disk.push(Fragment::Free { len });
            }
        }

        disk
    }
}
