use crate::Day;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct Day5;

static RULES: OnceLock<HashMap<u8, Vec<u8>>> = OnceLock::new();

#[derive(Copy, Clone)]
pub struct Id(u8);
#[derive(Clone)]
pub struct Update(Vec<Id>);
#[derive(Clone)]
pub struct Updates(Vec<Update>);

impl PartialEq<Self> for Id {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Id {}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> Ordering {
        let rules = RULES.get().unwrap();
        if rules
            .get(&self.0)
            .is_some_and(|rules| rules.contains(&other.0))
        {
            Ordering::Less
        } else if rules
            .get(&other.0)
            .is_some_and(|rules| rules.contains(&self.0))
        {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Day for Day5 {
    type Input = Updates;

    fn part1(input: Self::Input) -> String {
        input
            .0
            .into_iter()
            .filter_map(|update| {
                if update.0.is_sorted() {
                    Some(update.0[update.0.len() / 2].0 as u32)
                } else {
                    None
                }
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(input: Self::Input) -> String {
        input
            .0
            .into_iter()
            .filter_map(|mut update| {
                if !update.0.is_sorted() {
                    update.0.sort_unstable();
                    Some(update.0[update.0.len() / 2].0 as u32)
                } else {
                    None
                }
            })
            .sum::<u32>()
            .to_string()
    }

    fn parse_input(input: &[impl AsRef<str>]) -> Self::Input {
        let mut rules = HashMap::new();

        for line in input
            .iter()
            .map(AsRef::as_ref)
            .take_while(|line| !line.is_empty())
        {
            let (first, second) = line
                .split_once("|")
                .map(|(a, b)| (a.parse::<u8>().unwrap(), b.parse::<u8>().unwrap()))
                .unwrap();
            rules.entry(first).or_insert_with(Vec::new).push(second);
        }

        RULES.get_or_init(|| rules);

        let updates = input
            .iter()
            .skip_while(|line| !line.as_ref().is_empty())
            .skip(1)
            .map(|line| {
                Update(
                    line.as_ref()
                        .split(',')
                        .map(|s| Id(s.parse::<u8>().unwrap()))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        Updates(updates)
    }
}
