use itertools::Itertools;

#[derive(Eq, PartialEq, Clone, Debug)]
struct Stacks {
    pub stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn parse(input: &str) -> Self {
        let mut lines: Vec<&str> = input.lines().collect();

        let stack_labels = lines.pop().unwrap();
        lines.reverse();
        let mut out = vec![];

        for (x, char) in stack_labels.chars().enumerate() {
            if char == ' ' {
                continue;
            }

            let mut stack = vec![];
            for line in &lines {
                let target = line.chars().collect::<Vec<char>>()[x];
                if target == ' ' {
                    break;
                }
                stack.push(target);
            }
            out.push(stack);
        }

        Stacks { stacks: out }
    }

    pub fn place_on_stack(&mut self, position: usize, character: char) {
        self.stacks[position - 1].push(character)
    }

    pub fn place_multiple_on_stack(&mut self, position: usize, chars: Vec<char>) {
        self.stacks[position - 1].extend(chars);
    }

    pub fn pop_from_stack(&mut self, position: usize) -> char {
        self.stacks[position - 1].pop().unwrap()
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        let Instruction { count, from, to } = instruction;
        for _ in 1..=count {
            let pulled = self.pop_from_stack(from);
            self.place_on_stack(to, pulled);
        }
    }

    pub fn execute_instruction_9001(&mut self, instruction: Instruction) {
        let Instruction { count, from, to } = instruction;
        // silly way of doing this but it made me laugh
        let mut pulled = vec![];
        for _ in 1..=count {
            pulled.push(self.pop_from_stack(from));
        }
        pulled.reverse();
        self.place_multiple_on_stack(to, pulled);
    }

    pub fn top_of_stacks(&self) -> String {
        self.stacks
            .iter()
            .map(|x| x.iter().last().unwrap())
            .collect()
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
struct Instruction {
    pub count: u32,
    pub from: usize,
    pub to: usize,
}

impl Instruction {
    pub fn parse(input: &str) -> Self {
        let split: Vec<&str> = input.split(' ').collect();

        let count = split[1].parse().unwrap();
        let from = split[3].parse().unwrap();
        let to = split[5].parse().unwrap();

        Instruction { count, from, to }
    }

    pub fn parse_block(input: &str) -> Vec<Self> {
        input.lines().map(Self::parse).collect()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut stacks = Stacks::parse(split[0]);
    let instructions = Instruction::parse_block(split[1]);

    for instruction in instructions {
        stacks.execute_instruction(instruction);
    }

    let tops = stacks.top_of_stacks();

    Some(tops)
}

pub fn part_two(input: &str) -> Option<String> {
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut stacks = Stacks::parse(split[0]);
    let instructions = Instruction::parse_block(split[1]);

    for instruction in instructions {
        stacks.execute_instruction_9001(instruction);
    }

    let tops = stacks.top_of_stacks();

    Some(tops)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
