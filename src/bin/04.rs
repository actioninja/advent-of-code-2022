use itertools::Itertools;

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Seating {
    pub low: u32,
    pub high: u32,
}

impl Seating {
    fn contains(self, second: &Seating) -> bool {
        second.low >= self.low && second.high <= self.high
    }

    fn overlaps(self, second: &Seating) -> bool {
        self.low >= second.low && self.low <= second.high
            || self.high >= second.low && self.high <= second.high
    }

    fn parse(input: &str) -> Self {
        let seating: (&str, &str) = input.split('-').collect_tuple().unwrap();

        Seating {
            low: seating.0.parse().unwrap(),
            high: seating.1.parse().unwrap(),
        }
    }
}

pub type SeatingPair = (Seating, Seating);

fn parse_pairs(input: &str) -> Vec<SeatingPair> {
    input
        .lines()
        .map(|line| {
            let pair: (&str, &str) = line.split(',').collect_tuple().unwrap();
            (Seating::parse(pair.0), Seating::parse(pair.1))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let seating_pairs = parse_pairs(input);

    let number = seating_pairs
        .into_iter()
        .filter(|(first, second)| first.contains(second) || second.contains(first))
        .count() as u32;

    Some(number)
}

pub fn part_two(input: &str) -> Option<u32> {
    let seating_pairs = parse_pairs(input);

    let number = seating_pairs
        .into_iter()
        .filter(|(first, second)| first.overlaps(second) || second.overlaps(first))
        .count() as u32;

    Some(number)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
