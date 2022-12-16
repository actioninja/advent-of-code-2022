use advent_of_code::helpers::{Direction, Vec2d};

fn search(grid: &mut Vec2d<u32>, coord: (usize, usize), direction: Direction) -> bool {
    let reference_height = *grid.get(coord).unwrap();

    let tallest_tree_in_dir = grid
        .iter_direction(coord, direction)
        .reduce(|acc, x| if x > acc { x } else { acc })
        .unwrap_or(0);

    reference_height > tallest_tree_in_dir
}

fn visible(grid: &mut Vec2d<u32>, coord: (usize, usize)) -> bool {
    search(grid, coord, Direction::North)
        || search(grid, coord, Direction::South)
        || search(grid, coord, Direction::West)
        || search(grid, coord, Direction::East)
}

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = Vec2d::<char>::parse(input);

    let mapped: Vec<u32> = parsed
        .backing_iter()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let mut numbers = Vec2d::from_vec(parsed.x, parsed.y, mapped);

    let mut visibility_grid = Vec2d::new_filled(numbers.x, numbers.y, true);

    for x in 1..(numbers.x - 1) {
        for y in 1..(numbers.y - 1) {
            let coord = (x, y);

            visibility_grid.put(coord, visible(&mut numbers, coord));
        }
    }

    let num_visible: u32 = visibility_grid.backing_iter().filter(|x| **x).count() as u32;

    Some(num_visible)
}

fn score_in_direction(grid: &mut Vec2d<u32>, coord: (usize, usize), direction: Direction) -> u32 {
    let reference_height = *grid.get(coord).unwrap();

    let direction_trees: Vec<u32> = grid.iter_direction(coord, direction).collect();

    let mut found_in_dir = 0;
    for tree in direction_trees {
        found_in_dir += 1;
        if tree >= reference_height {
            break;
        }
    }

    found_in_dir
}

fn calc_score(grid: &mut Vec2d<u32>, coord: (usize, usize)) -> u32 {
    score_in_direction(grid, coord, Direction::North)
        * score_in_direction(grid, coord, Direction::South)
        * score_in_direction(grid, coord, Direction::East)
        * score_in_direction(grid, coord, Direction::West)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parsed = Vec2d::<char>::parse(input);

    let mapped: Vec<u32> = parsed
        .backing_iter()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let mut numbers = Vec2d::from_vec(parsed.x, parsed.y, mapped);

    let mut score_grid = Vec2d::new_filled(numbers.x, numbers.y, 0);

    for x in 1..(numbers.x - 1) {
        for y in 1..(numbers.y - 1) {
            let coord = (x, y);

            score_grid.put(coord, calc_score(&mut numbers, coord));
        }
    }

    let mut score_grid: Vec<u32> = score_grid.backing_iter().copied().collect();

    score_grid.sort();

    let highest_score = score_grid.pop().unwrap();

    Some(highest_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
