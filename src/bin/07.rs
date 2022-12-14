use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum FileLeaf {
    File(u32),
    Dir(BTreeMap<String, FileLeaf>),
}

impl FileLeaf {
    pub fn size(&self) -> u32 {
        match self {
            Self::File(size) => *size,
            Self::Dir(map) => map.values().fold(0, |acc, x| acc + x.size()),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct FileSystem {
    head: FileLeaf,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            head: FileLeaf::Dir(BTreeMap::new()),
        }
    }

    fn root_size(&self) -> u32 {
        self.head.size()
    }

    fn get_dirs(&self) -> Vec<(String, FileLeaf)> {
        let mut found = vec![("/".to_string(), self.head.clone())];

        let extracted_map = if let FileLeaf::Dir(map) = &self.head {
            map
        } else {
            panic!("head is not a dir");
        };

        let mut to_plunge: Vec<BTreeMap<String, FileLeaf>> = vec![extracted_map.clone()];

        while !to_plunge.is_empty() {
            let popped = to_plunge.pop().unwrap();

            let mut filtered = vec![];

            for (name, file) in popped.iter() {
                if let FileLeaf::Dir(_) = file {
                    let name = name.clone();
                    let file = file.clone();
                    filtered.push((name, file));
                }
            }

            found.extend(filtered.iter().cloned());

            let eh = filtered
                .iter()
                .map(|(_string, file)| {
                    if let FileLeaf::Dir(map) = file {
                        map
                    } else {
                        panic!("I've been working on this too long please help");
                    }
                })
                .cloned();
            to_plunge.extend(eh);
        }
        found
    }
}

struct FileSystemBuilder {
    file_system: FileSystem,
    current_position: Vec<String>,
}

impl FileSystemBuilder {
    fn new() -> Self {
        let file_system = FileSystem::new();
        FileSystemBuilder {
            file_system,
            current_position: vec!["/".to_string()],
        }
    }

    fn parse_input(&mut self, input: &str) {
        let lines: Vec<&str> = input.lines().collect();
        let mut index = 0;
        while index < lines.len() {
            let line = lines[index];

            let split: Vec<&str> = line.split(' ').collect();

            match split[1] {
                "cd" => {
                    self.change_directory(split[2]);
                    index += 1;
                }
                "ls" => {
                    let mut jindex = index + 1;
                    let mut found_listings = vec![];
                    while jindex < lines.len() && !lines[jindex].starts_with('$') {
                        found_listings.push(lines[jindex]);
                        jindex += 1;
                    }
                    index += found_listings.len() + 1;
                    self.populate_directory(found_listings);
                }
                _ => panic!("Error, invalid command: {}", split[1]),
            }
        }
    }

    fn change_directory(&mut self, input: &str) {
        match input {
            ".." => {
                self.current_position.pop();
            }
            "/" => {
                self.current_position = vec!["/".to_string()];
            }
            _ => self.current_position.push(input.to_string()),
        }
    }

    fn get_current_dir_mut(&mut self) -> &mut BTreeMap<String, FileLeaf> {
        let mut current = &mut self.file_system.head;

        if self.current_position == vec!["/".to_string()] {
            if let FileLeaf::Dir(head_map) = current {
                return head_map;
            }
        }

        let mut reversed_stack = self.current_position.clone();
        reversed_stack.reverse();
        // pop root
        reversed_stack.pop();
        // reverse back
        reversed_stack.reverse();
        for entry in reversed_stack {
            if let FileLeaf::Dir(dir_map) = current {
                let found = dir_map.get_mut(&entry).unwrap();
                current = found;
            }
        }
        if let FileLeaf::Dir(map) = current {
            map
        } else {
            panic!("Fail!")
        }
    }

    fn populate_directory(&mut self, lines: Vec<&str>) {
        fn parse_line(input: &str) -> (String, FileLeaf) {
            let (first, second): (&str, &str) = input.split(' ').collect_tuple().unwrap();
            if first == "dir" {
                (second.to_string(), FileLeaf::Dir(BTreeMap::new()))
            } else {
                (second.to_string(), FileLeaf::File(first.parse().unwrap()))
            }
        }

        let parsed = lines.iter().copied().map(parse_line);
        let current_dir = self.get_current_dir_mut();
        for (name, leaf) in parsed {
            current_dir.insert(name, leaf);
        }
    }

    fn into_file_system(self) -> FileSystem {
        self.file_system
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut builder = FileSystemBuilder::new();
    builder.parse_input(input);
    let file_system = builder.into_file_system();

    let dirs = file_system.get_dirs();

    let sizes: u32 = dirs
        .iter()
        .map(|(_, file)| file)
        .map(FileLeaf::size)
        .filter(|x| *x < 100000)
        .sum();
    Some(sizes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cap: u32 = 70000000;

    let update_size = 30000000;

    let under_size = cap - update_size;

    let mut builder = FileSystemBuilder::new();
    builder.parse_input(input);
    let file_system = builder.into_file_system();

    let dirs = file_system.get_dirs();

    let root_size = file_system.root_size();
    println!("Total Size: {root_size}");

    let minimum_required = root_size - under_size;

    println!("Minimum Required: {minimum_required}");

    let mut large_enough: Vec<(String, FileLeaf)> = dirs
        .iter()
        .filter(|(_, obj)| obj.size() >= minimum_required)
        .cloned()
        .collect();

    large_enough.sort_by(|(_, leaf_a), (_, leaf_b)| leaf_a.size().cmp(&leaf_b.size()));

    let smallest_name = large_enough[0].1.size();

    Some(smallest_name)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some("d".to_string()));
    }
}
