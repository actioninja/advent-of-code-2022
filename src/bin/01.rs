use advent_of_code::helpers::strip_newline;

fn elves(in_str: String) -> Vec<u32> {
    in_str
        .split("\n\n")
        .map(|x| x.lines().map(|str| str.parse::<u32>().unwrap()).sum())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let elves = elves(strip_newline(input));

    let highest = elves.iter().fold(0u32, |acc, x| {
        if *x > acc {
            *x
        } else {
            acc
        }
    });

    Some(highest)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = elves(strip_newline(input));

    elves.sort();
    elves.reverse();

    Some(elves.iter().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
