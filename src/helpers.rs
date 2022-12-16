/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};

pub fn strip_newline(in_str: &str) -> String {
    in_str.replace('\r', "")
}

// I know that with a fixed size arrays would probably be better but vecs mean I don't have to think
// too hard about the compile time lengths
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec2d<T> {
    pub vec: Vec<T>,
    pub x: usize,
    pub y: usize,
}

impl<T: Clone> Vec2d<T> {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Self {
            vec: Vec::with_capacity(x_size * y_size),
            x: x_size,
            y: y_size,
        }
    }

    pub fn new_filled(x_size: usize, y_size: usize, filled_value: T) -> Vec2d<T> {
        Vec2d {
            x: x_size,
            y: y_size,
            vec: vec![filled_value; x_size * y_size],
        }
    }

    pub fn from_vec(x_size: usize, y_size: usize, vec: Vec<T>) -> Self {
        Self {
            vec,
            x: x_size,
            y: y_size,
        }
    }

    pub fn parse(input: &str) -> Vec2d<char> {
        if input.is_empty() {
            panic!("Empty Input");
        }

        let split: Vec<&str> = input.lines().collect();

        let x = split[0].len();
        let y = split.len();

        let vec: Vec<char> = split.join("").chars().collect();

        Vec2d { x, y, vec }
    }

    // panics if the vec isn't populated properly
    pub fn put(&mut self, position: (usize, usize), t: T) {
        let index = position.1 * self.x + position.0;

        self.vec[index] = t;
    }

    pub fn get(&self, position: (usize, usize)) -> Option<&T> {
        self.vec.get(position.1 * self.x + position.0)
    }

    pub fn get_mut(&mut self, position: (usize, usize)) -> Option<&mut T> {
        self.vec.get_mut(position.1 * self.x + position.0)
    }

    pub fn backing_iter(&self) -> std::slice::Iter<T> {
        self.vec.iter()
    }

    pub fn iter_direction(
        &mut self,
        coordinate: (usize, usize),
        direction: Direction,
    ) -> DirectionIter<'_, T> {
        DirectionIter {
            backing_vec: self,
            direction,
            current: coordinate,
        }
    }
}

impl<T: Debug> Display for Vec2d<T> {
    // doesn't need to be fast, just work
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let to_strings: Vec<String> = self.vec.iter().map(|x| format!("{:?}", x)).collect();
        let mut sorted = to_strings.clone();
        sorted.sort_by_key(|a| a.len());
        sorted.reverse();

        let longest = to_strings[0].clone();

        let pad_len = longest.len();

        let formatted = to_strings
            .iter()
            .map(|x| format!(" {x:width$} ", width = pad_len))
            .chunks(self.x)
            .into_iter()
            .map(|mut chunk| chunk.join(""))
            .join("\n");

        write!(f, "{formatted}")
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct DirectionIter<'a, T> {
    backing_vec: &'a mut Vec2d<T>,
    direction: Direction,
    current: (usize, usize),
}

impl<T: Clone + Debug> Iterator for DirectionIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_pos = match self.direction {
            Direction::North => (self.current.0 as isize, self.current.1 as isize - 1),
            Direction::South => (self.current.0 as isize, self.current.1 as isize + 1),
            Direction::East => (self.current.0 as isize + 1, self.current.1 as isize),
            Direction::West => (self.current.0 as isize - 1, self.current.1 as isize),
        };

        if next_pos.0 >= 0
            && next_pos.1 >= 0
            && next_pos.0 < self.backing_vec.x as isize
            && next_pos.1 < self.backing_vec.y as isize
        {
            let cast = (next_pos.0 as usize, next_pos.1 as usize);
            self.current = cast;
            Some(self.backing_vec.get(cast).unwrap().clone())
        } else {
            None
        }
    }
}
