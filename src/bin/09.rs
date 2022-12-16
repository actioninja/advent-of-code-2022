use advent_of_code::helpers::Direction;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Rope {
    pub head: (isize, isize),
    pub tail: Vec<(isize, isize)>,
    pub visited: HashSet<(isize, isize)>,
}

impl Rope {
    pub fn new(length: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        let tail = vec![(0, 0); length - 1];
        Self {
            head: (0, 0),
            tail,
            visited,
        }
    }

    pub fn update_chain(&mut self) {
        let mut full_chain = vec![self.head];
        full_chain.extend(self.tail.clone());

        let mut next = self.head;
        for i in 0..self.tail.len() {
            let current = self.tail[i];
            let result = Rope::move_knot_towards_next(current, next);
            self.tail[i] = result;
            next = result;
        }
    }

    pub fn move_knot_towards_next(knot: (isize, isize), next: (isize, isize)) -> (isize, isize) {
        let difference = (next.0 - knot.0, next.1 - knot.1);

        let abs = (difference.0.abs(), difference.1.abs());

        let should_move = abs.0 > 1 || abs.1 > 1;

        if should_move {
            let move_amount = (difference.0.clamp(-1, 1), difference.1.clamp(-1, 1));
            (knot.0 + move_amount.0, knot.1 + move_amount.1)
        } else {
            knot
        }
    }

    pub fn move_in_direction(&mut self, direction: Direction) {
        self.head = direction.step(self.head);
        self.update_chain();
        let new_tail_loc = *self.tail.last().unwrap();
        self.visited.insert(new_tail_loc);
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
    let mut rope = Rope::new(2);

    for command in input.lines() {
        rope.run_command(command);
    }

    Some(rope.num_visited() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = Rope::new(10);

    for command in input.lines() {
        rope.run_command(command);
    }

    Some(rope.num_visited() as u32)
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
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part_two(input), Some(36));
    }
}
