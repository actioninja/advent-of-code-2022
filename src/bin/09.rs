use advent_of_code::helpers::Direction;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Rope {
    pub head: (isize, isize),
    pub tail: (isize, isize),
    pub visited: HashSet<(isize, isize)>,
}

impl Rope {
    pub fn new() -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        Self {
            head: (0, 0),
            tail: (0, 0),
            visited,
        }
    }

    pub fn move_tail_towards_head(&mut self) {
        let difference = (self.head.0 - self.tail.0, self.head.1 - self.tail.1);

        let abs = (difference.0.abs(), difference.1.abs());

        let should_move = abs.0 > 1 || abs.1 > 1;

        if should_move {
            let move_amount = (difference.0.clamp(-1, 1), difference.1.clamp(-1, 1));
            self.tail = (self.tail.0 + move_amount.0, self.tail.1 + move_amount.1);
        }

        self.visited.insert(self.tail);
    }

    pub fn move_in_direction(&mut self, direction: Direction) {
        self.head = direction.step(self.head);
        self.move_tail_towards_head();
    }

    pub fn move_in_direction_multiple(&mut self, direction: Direction, count: usize) {
        for _ in 0..count {
            self.move_in_direction(direction);
        }
    }

    pub fn run_command(&mut self, input: &str) {
        let (dir, count): (&str, &str) = input.split(' ').collect_tuple().unwrap();

        let dir = match dir {
            "U" => Direction::North,
            "D" => Direction::South,
            "R" => Direction::East,
            "L" => Direction::West,
            _ => panic!("Invalid Dir Parsed"),
        };

        let count: usize = count.parse().unwrap();

        self.move_in_direction_multiple(dir, count);
    }

    pub fn num_visited(&self) -> usize {
        self.visited.len()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rope = Rope::new();

    for command in input.lines() {
        rope.run_command(command);
    }

    Some(rope.num_visited() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
