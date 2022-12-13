use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref PRIORITIES: HashMap<char, u32> = {
        let mut m = HashMap::new();
        let mut counter = 0;
        for c in 'a'..='z' {
            counter += 1;
            m.insert(c, counter);
        }
        for c in 'A'..='Z' {
            counter += 1;
            m.insert(c, counter);
        }
        m
    };
}

fn get_packs(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let split_point = line.len() / 2;
            line.split_at(split_point)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let packs = get_packs(input);

    let sum = packs
        .iter()
        .map(|pack| {
            let mut found = HashSet::new();
            found.extend(pack.0.chars());
            let dupe = pack.1.chars().find(|c| found.contains(c)).unwrap();
            PRIORITIES.get(&dupe).unwrap()
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
