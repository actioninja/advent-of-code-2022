use advent_of_code::helpers::{display_bool_grid, Direction, Vec2d};
use lazy_static::lazy_static;
use priority_queue::DoublePriorityQueue;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::Mutex;

type Coord = (usize, usize);

fn distance(a: Coord, b: Coord) -> u32 {
    let a = (a.0 as i32, a.1 as i32);
    let b = (b.0 as i32, b.1 as i32);
    let diff = (b.0 - a.0, b.1 - a.1);
    diff.0.unsigned_abs() + diff.1.unsigned_abs()
}

fn valid_move(grid: &Vec2d<u32>, from: Coord, to: Coord) -> bool {
    let from = *grid.get(from).unwrap();
    let to = *grid.get(to).unwrap();

    let gap = to as i32 - from as i32;

    gap <= 1
}

fn valid_moves(grid: &Vec2d<u32>, coord: Coord) -> Vec<Coord> {
    let steps = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    let mut out = vec![];
    let cast = (coord.0 as i32, coord.1 as i32);
    for dir in steps {
        let stepped = dir.step(cast);

        if stepped.0 < 0
            || stepped.1 < 0
            || stepped.0 >= grid.x as i32
            || stepped.1 >= grid.y as i32
        {
            continue;
        }

        let stepped = (stepped.0 as usize, stepped.1 as usize);

        if valid_move(grid, coord, stepped) {
            out.push(stepped);
        }
    }

    out
}

fn reconstruct_path(came_from: HashMap<Coord, Coord>, current: Coord) -> Vec<Coord> {
    let mut path = vec![current];
    let mut current = current;
    while came_from.contains_key(&current) {
        current = *came_from.get(&current).unwrap();
        path.push(current);
    }
    path.reverse();
    path
}

// busted
fn astar(grid: &Vec2d<u32>, start: Coord, goal: Coord) -> Option<Vec<Coord>> {
    //let heuristic = |coord| distance(coord, goal);
    let heuristic = |_| 0;

    let mut came_from: HashMap<Coord, Coord> = HashMap::new();

    let mut g_scores: HashMap<Coord, u32> = HashMap::new();
    g_scores.insert(start, 0);

    let mut f_scores: HashMap<Coord, u32> = HashMap::new();
    let start_f_score = heuristic(start);
    f_scores.insert(start, start_f_score);

    let mut open_set = DoublePriorityQueue::new();
    open_set.push(start, start_f_score);

    while !open_set.is_empty() {
        let (current, _) = open_set.pop_min().unwrap();

        if current == goal {
            return Some(reconstruct_path(came_from, current));
        }

        //display_char_vis_grid(&came_from.values().copied().collect());
        //std::io::stdout().flush().unwrap();

        let neighbors = valid_moves(grid, current);
        for neighbor in neighbors {
            let tentative_g_score = *g_scores.get(&current).unwrap_or(&u32::MAX); // ADD EDGE WEIGHT HERE IF THAT'S PART 2
            if tentative_g_score < *g_scores.get(&neighbor).unwrap_or(&u32::MAX) {
                // hot path! record it
                came_from.insert(neighbor, current);
                g_scores.insert(neighbor, tentative_g_score);
                let calced_fscore = tentative_g_score + heuristic(neighbor);
                f_scores.insert(neighbor, calced_fscore);
                open_set.push(neighbor, calced_fscore);
            }
        }
    }

    None
}

fn dumbjikstra(grid: &Vec2d<u32>, start: Coord, goal: Coord) -> Option<Vec<Coord>> {
    let mut unvisited = BTreeSet::new();
    unvisited.insert(start);

    let mut distances = HashMap::new();
    distances.insert(start, 0);

    while !unvisited.is_empty() {
        let next = unvisited.pop_first().unwrap();
    }
    None
}

lazy_static! {
    static ref HEIGHTS: HashMap<char, u32> = {
        let mut m = HashMap::new();
        let mut counter = 0;
        for c in 'a'..='z' {
            counter += 1;
            m.insert(c, counter);
        }
        m.insert('S', 1);
        m.insert('E', 26);
        m
    };
}

lazy_static! {
    static ref CHAR_MAP: Mutex<Vec2d<char>> = {
        let mut m = Vec2d::new(1, 1);
        Mutex::new(m)
    };
}

fn map_grid(input: &char) -> u32 {
    *HEIGHTS.get(input).unwrap()
}

fn generate_vis_grid(grid: &Vec2d<u32>, path: &Vec<Coord>) -> String {
    let mut vis_grid = Vec2d::new_filled(grid.x, grid.y, false);

    for coord in path {
        vis_grid.put(*coord, true)
    }

    display_bool_grid(&vis_grid)
}

fn display_char_vis_grid(path: &Vec<Coord>) {
    let mut lock = CHAR_MAP.lock().unwrap();

    for coord in path {
        lock.put(*coord, '█');
    }

    println!("{}", *lock);
    println!("--");
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Vec2d::<char>::parse(input);

    {
        let mut data = CHAR_MAP.lock().unwrap();
        data.resize(grid.x, grid.y);
        data.replace_vec(&grid.vec);
    }

    let start_index = grid.backing_iter().position(|x| x == &'S').unwrap();
    let goal_index = grid.backing_iter().position(|x| x == &'E').unwrap();

    let start = grid.index_to_coord(start_index);
    println!("Start: {start:?}");

    let goal = grid.index_to_coord(goal_index);
    println!("Goal: {goal:?}");

    let mapped_backing: Vec<u32> = grid.backing_iter().map(map_grid).collect();

    let grid = Vec2d::from_vec(grid.x, grid.y, mapped_backing);

    let path = astar(&grid, start, goal).expect("Failed to find path");

    println!("{path:?}");

    let displayed = generate_vis_grid(&grid, &path);

    println!("{displayed}");

    display_char_vis_grid(&path);

    // subtract end
    let path_length = path.len() - 1;
    Some(path_length as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
