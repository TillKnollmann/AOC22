use itertools::Itertools;

/// Represents a file system objects
enum FSObject {
    File {
        size: u32,
    },
    Directory {
        objects: Vec<FSObject>,
    }
}

impl FSObject {

    /// Returns the total size of the object
    fn get_size(&self) -> u32 {
        match self {
            FSObject::File { size } => *size,
            FSObject::Directory { objects } => objects.iter().map(|obj| obj.get_size()).sum(),
        }
    }
}

/// Recursively builds up the file system
fn build_filesystem(input: &mut Vec<&str>) -> FSObject {
    let mut current = FSObject::Directory { objects: vec![] };

    while let Some(line) = input.pop() {
        let mut words = line.split_whitespace();

        match words.next().unwrap() {
            "$" => {
                if let "cd" = words.next().unwrap() {
                    match words.next().unwrap() {
                        ".." => {
                            return current;
                        },
                        _ => {
                            match current {
                                FSObject::File { size:_ } => continue,
                                FSObject::Directory { ref mut objects } => objects.push(build_filesystem(input)),
                            }
                        }
                    }
                }
            },
            "dir" => {},
            value => {
                match current {
                    FSObject::File { size:_ } => continue,
                    FSObject::Directory { ref mut objects } => objects.push(FSObject::File { size: value.parse::<u32>().unwrap() }),
                }
            },
        }
    }

    current


}

/// Returns the sum of all sizes of dirs that have a size under a certain threshold
fn get_size_of_dir_smaller(current: &FSObject, threshold: u32) -> u32 {
    let score_current = match current {
        FSObject::File { size: _ } => 0,
        FSObject::Directory { objects: _ } => {let size = current.get_size(); if size <= threshold {size} else {0}},
    };
    let score_below = match current {
        FSObject::File { size: _ } => 0,
        FSObject::Directory { ref objects } => objects.iter().map(|obj| {get_size_of_dir_smaller(obj, threshold)}).sum(),
    };
    score_current + score_below
}

/// Returns all directories
fn get_all_dirs(current: &FSObject) -> Vec<&FSObject> {
    let mut result: Vec<&FSObject> = vec![];
    match current {
        FSObject::File { size: _ } => {},
        FSObject::Directory { objects } => { result.push(current); for dir in objects {
            result.append(&mut get_all_dirs(dir));
        }},
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let root_of_all_evil = build_filesystem(&mut input.lines().skip(1).collect_vec().into_iter().rev().collect_vec());

    Some(get_size_of_dir_smaller(&root_of_all_evil, 100000))
}

pub fn part_two(input: &str) -> Option<u32> {
    let root_of_all_evil = build_filesystem(&mut input.lines().skip(1).collect_vec().into_iter().rev().collect_vec());

    let min = 30000000 - (70000000 - root_of_all_evil.get_size());

    let all_dirs = get_all_dirs(&root_of_all_evil);
    Some(all_dirs.into_iter().filter_map(|dir| {let size = dir.get_size(); if size >= min {Some(size)} else {None}}).min().unwrap())


}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(7, 1, part_one, input);
    aoc::solve!(7, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
