#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleCPU {
    register_x: isize,
    history: Vec<isize>,
}

impl SimpleCPU {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn noop(&mut self) {
        self.history.push(self.register_x);
    }

    pub fn addx(&mut self, amount: isize) {
        self.history.push(self.register_x);
        self.register_x += amount;
        self.history.push(self.register_x);
    }

    pub fn run_instruction(&mut self, input: &str) {
        let split: Vec<&str> = input.split(' ').collect();

        match split[0] {
            "noop" => self.noop(),
            "addx" => self.addx(split[1].parse().unwrap()),
            _ => panic!("Invalid Instruction"),
        }
    }

    pub fn get_strength_at_cycle(&self, cycle_num: usize) -> isize {
        self.history[cycle_num - 1] * (cycle_num as isize)
    }
}

impl Default for SimpleCPU {
    fn default() -> Self {
        Self {
            register_x: 1,
            history: vec![1],
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cpu = SimpleCPU::new();

    for line in input.lines() {
        cpu.run_instruction(line);
    }

    let wanted_cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];

    let sum: isize = wanted_cycles
        .iter()
        .map(|x| cpu.get_strength_at_cycle(*x))
        .sum();

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
