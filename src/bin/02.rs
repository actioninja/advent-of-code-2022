use advent_of_code::helpers::strip_newline;
use itertools::Itertools;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    pub fn value(self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    pub fn play_from(self, result: GameResult) -> Self {
        match result {
            GameResult::Draw => self,
            GameResult::Win => match self {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            },
            GameResult::Loss => match self {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            },
        }
    }

    pub fn result_against(self, opponent: Play) -> GameResult {
        match self {
            Play::Rock => match opponent {
                Play::Rock => GameResult::Draw,
                Play::Paper => GameResult::Loss,
                Play::Scissors => GameResult::Win,
            },
            Play::Paper => match opponent {
                Play::Rock => GameResult::Win,
                Play::Paper => GameResult::Draw,
                Play::Scissors => GameResult::Loss,
            },
            Play::Scissors => match opponent {
                Play::Rock => GameResult::Loss,
                Play::Paper => GameResult::Win,
                Play::Scissors => GameResult::Draw,
            },
        }
    }

    pub fn parse_opponent_part1(opponent_play: &str) -> Self {
        match opponent_play {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!("Invalid opponent play: {opponent_play}"),
        }
    }

    pub fn parse_my_play_part1(my_play: &str) -> Self {
        match my_play {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => panic!("Invalid my play: {my_play}"),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum GameResult {
    Loss,
    Draw,
    Win,
}

impl GameResult {
    pub fn get_score(self) -> u32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }

    pub fn parse_result(result: &str) -> Self {
        match result {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid result: {result}"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let stripped = strip_newline(input);

    let lines = stripped.lines();

    let sum = lines
        .map(|line| line.split(' ').collect_tuple())
        .map(|option| option.unwrap())
        .map(|(opponent, mine)| {
            (
                Play::parse_opponent_part1(opponent),
                Play::parse_my_play_part1(mine),
            )
        })
        .map(|(opponent, mine)| mine.value() + mine.result_against(opponent).get_score())
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let stripped = strip_newline(input);

    let lines = stripped.lines();

    let sum = lines
        .map(|line| line.split(' ').collect_tuple())
        .map(|option| option.unwrap())
        .map(|(opponent, result)| {
            (
                Play::parse_opponent_part1(opponent),
                GameResult::parse_result(result),
            )
        })
        .map(|(opponent, result)| {
            let my_play = opponent.play_from(result);
            my_play.value() + result.get_score()
        })
        .sum();

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
