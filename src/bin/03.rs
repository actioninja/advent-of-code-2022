use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::str::Chars;

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

fn get_packs(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let pack = get_packs(input);

    let pack_pouches = pack.iter().map(|line| {
        let split_point = line.len() / 2;
        line.split_at(split_point)
    });

    let sum = pack_pouches
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
    let packs = get_packs(input);

    let groups = packs.iter().chunks(3);

    let result: u32 = groups
        .into_iter()
        .map(|chunk| {
            let mut previous_packs: Vec<HashSet<char>> = vec![];
            for pack in chunk {
                // if a previous entry isn't found, this is the first one, so don't filter at all
                let chars: Vec<char> = if let Some(previous) = previous_packs.last() {
                    pack.chars().filter(|x| previous.contains(x)).collect()
                } else {
                    pack.chars().collect()
                };
                let mut current_set: HashSet<char> = HashSet::new();
                current_set.extend(chars.iter());
                previous_packs.push(current_set);
            }
            let remaining = previous_packs.last().unwrap().iter().last().unwrap();
            *PRIORITIES.get(remaining).unwrap()
        })
        .sum();

    Some(result)
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
        assert_eq!(part_two(&input), Some(70));
    }
}
