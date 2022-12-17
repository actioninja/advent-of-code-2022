#[derive(PartialEq, Eq, Clone, Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    // input should be everything after "Operation: new = "
    fn parse(input: &str) -> Self {
        let split: Vec<&str> = input.split(' ').collect();
        let operand = split[1];
        match operand {
            "+" => Self::Add(split[2].parse().unwrap()),
            "*" => {
                if split[2] == "old" {
                    Self::Square
                } else {
                    Self::Multiply(split[2].parse().unwrap())
                }
            }
            _ => panic!("Invalid Operand"),
        }
    }

    fn do_operation(self, x: u64) -> u64 {
        match self {
            Self::Add(y) => x + y,
            Self::Multiply(y) => x * y,
            Self::Square => x.clone() * x,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_divisor: u64,
    target_true: usize,
    target_false: usize,
    pub inspection_count: u128,
}

impl Monkey {
    fn parse(input: Vec<&str>) -> Self {
        let starting_items: Vec<u64> = input[0]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let operation = Operation::parse(input[1].strip_prefix("  Operation: new = ").unwrap());

        let divisor = input[2]
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();

        let target_true = input[3]
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let target_false = input[4]
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        Self {
            items: starting_items,
            operation,
            test_divisor: divisor,
            target_true,
            target_false,
            inspection_count: 0,
        }
    }

    fn inspect_and_throw(&mut self, item: u64, lcm: Option<u64>) -> (usize, u64) {
        let inspection = self.operation.clone().do_operation(item);
        self.inspection_count += 1;

        let result = if let Some(lcm) = lcm {
            inspection % lcm
        } else {
            inspection / 3
        };
        let target = if result % self.test_divisor == 0 {
            self.target_true
        } else {
            self.target_false
        };

        (target, result)
    }

    // result is (target, item)
    fn take_turn(&mut self, lcm: Option<u64>) -> Vec<(usize, u64)> {
        let mut out = vec![];
        for i in 0..self.items.len() {
            let item = self.items[i];
            out.push(self.inspect_and_throw(item, lcm))
        }
        self.items.clear();
        out
    }

    fn catch_item(&mut self, item: u64) {
        self.items.push(item)
    }
}

fn play_round(monkeys: &mut Vec<Monkey>, lcm: Option<u64>) {
    for i in 0..monkeys.len() {
        let results = monkeys[i].take_turn(lcm);
        for (target, item) in results {
            monkeys[target].items.push(item);
        }
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut out = vec![];

    let lines: Vec<&str> = input.lines().collect();

    // there's probably a funky way of doing this with iterators but aoc is already turning
    // my brain in to mush
    let mut i = 0;
    while i < lines.len() {
        let current_line = lines[i];
        let monkey_prefix = "Monkey ";
        if current_line.starts_with(monkey_prefix) {
            let mut args = vec![];
            for j in 1..=5 {
                args.push(lines[i + j]);
            }
            let parsed = Monkey::parse(args);
            out.push(parsed);
            i += 5;
        } else {
            //infinite loop escape plan
            i += 1;
        }
    }

    out
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut monkeys = parse_monkeys(input);

    for _ in 1..=20 {
        play_round(&mut monkeys, None);
    }

    let mut monkey_business: Vec<u128> = monkeys.iter().map(|m| m.inspection_count).collect();

    println!("{:#?}", monkey_business);

    monkey_business.sort();

    let highest = monkey_business.pop().unwrap();
    let second_highest = monkey_business.pop().unwrap();

    Some(highest * second_highest)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut monkeys = parse_monkeys(input);

    let lcm = monkeys.iter().map(|x| x.test_divisor).product();

    for i in 1..=10000 {
        play_round(&mut monkeys, Some(lcm));
    }

    let mut monkey_business: Vec<u128> = monkeys.iter().map(|m| m.inspection_count).collect();

    println!("{:#?}", monkey_business);

    monkey_business.sort();

    let highest = monkey_business.pop().unwrap();
    let second_highest = monkey_business.pop().unwrap();

    Some(highest * second_highest)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
