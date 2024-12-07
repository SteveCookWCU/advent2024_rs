use crate::Day;
use std::collections::VecDeque;

pub struct Day7;

impl Day for Day7 {
    type Input = (Vec<Vec<u64>>, Vec<u64>);

    fn part1(input: Self::Input) -> String {
        let mut sum = 0;
        for (operands, test) in input.0.into_iter().zip(input.1.into_iter()) {
            let num_operators = operands.len() - 1;
            if (0..(2 << num_operators)).any(|i| {
                let val = operands
                    .iter()
                    .copied()
                    .enumerate()
                    .reduce(|(curr_index, num1), (j, num2)| {
                        if (2 << curr_index) & i == 0 {
                            (j, num1 * num2)
                        } else {
                            (j, num1 + num2)
                        }
                    })
                    .unwrap()
                    .1;
                test == val
            }) {
                sum += test;
            }
        }
        sum.to_string()
    }

    fn part2(input: Self::Input) -> String {
        let mut sum = 0;
        for (operands, test) in input.0.into_iter().zip(input.1.into_iter()) {
            let mut queue = VecDeque::new();
            queue.push_back(operands[0]);
            (1..operands.len()).for_each(|i| {
                let len = queue.len();
                (0..len).for_each(|_| {
                    let curr = queue.pop_front().unwrap();
                    queue.push_back(curr + operands[i]);
                    queue.push_back(curr * operands[i]);
                    queue.push_back(format!("{curr}{}", operands[i]).parse().unwrap())
                })
            });
            if queue.contains(&test) {
                sum += test;
            }
        }
        sum.to_string()
    }

    fn parse_input(input: &[impl AsRef<str>]) -> Self::Input {
        let mut operand_vec = Vec::with_capacity(input.len());
        let mut test_value_vec = Vec::with_capacity(input.len());
        for line in input.iter().map(AsRef::as_ref) {
            let (test_val, operands) = line.split_once(": ").unwrap();
            let test_val = test_val.parse::<u64>().unwrap();
            let operands = operands
                .split(' ')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            test_value_vec.push(test_val);
            operand_vec.push(operands);
        }
        (operand_vec, test_value_vec)
    }
}
