use std::collections::HashSet;

fn window_only_unique(window: &[char]) -> bool {
    let mut found = HashSet::new();
    for char in window {
        if found.contains(char) {
            return false;
        }
        found.insert(char);
    }
    true
}

pub fn find_with_window_size(input: &str, window_size: usize) -> u32 {
    let char_vec: Vec<char> = input.chars().collect();
    let windows = char_vec.windows(window_size);

    let mut found = 0;
    for (pos, window) in windows.enumerate() {
        if window_only_unique(window) {
            found = pos as u32;
            break;
        }
    }

    found + window_size as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_with_window_size(input, 4))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_with_window_size(input, 14))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(23));
    }
}
