use std::{collections::HashSet, hash::Hash};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Blizzard {
    x: u32,
    y: u32,
    dir: u8,
}

/// Parses the input and returns the maximum x and y and a set of blizzards
fn parse_input(input: &str) -> (u32, u32, HashSet<Blizzard>) {
    // get number of lines as y and number of chars in first line as x
    let mut y: u32 = 0;
    let mut x = 0;
    for line in input.lines() {
        y += 1;
        x = line.len() as u32;
    }
    y -= 3;
    x -= 3;

    // create a set of blizzards
    let mut blizzards = HashSet::new();
    // every position with <, >, ^, v is a blizzard
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '<' || c == '>' || c == '^' || c == 'v' {
                blizzards.insert(Blizzard {
                    x: (x - 1) as u32,
                    y: (y - 1) as u32,
                    dir: match c {
                        '<' => 0,
                        '>' => 1,
                        '^' => 2,
                        'v' => 3,
                        _ => 0,
                    },
                });
            }
        }
    }

    (x, y, blizzards)
}

/// Moves a blizzard and returns the new blizzard
fn move_blizzard(blizzard: Blizzard, x: u32, y: u32) -> Blizzard {
    let mut blizzard = blizzard;
    match blizzard.dir {
        0 => {
            if blizzard.x == 0 {
                blizzard.x = x;
            } else {
                blizzard.x -= 1;
            }
        }
        1 => {
            if blizzard.x == x {
                blizzard.x = 0;
            } else {
                blizzard.x += 1;
            }
        }
        2 => {
            if blizzard.y == 0 {
                blizzard.y = y;
            } else {
                blizzard.y -= 1;
            }
        }
        3 => {
            if blizzard.y == y {
                blizzard.y = 0;
            } else {
                blizzard.y += 1;
            }
        }
        _ => {}
    }
    blizzard
}

/// Moves all blizzards and returns the new blizzards
fn move_all_blizzards(blizzards: HashSet<Blizzard>, x: u32, y: u32) -> HashSet<Blizzard> {
    let mut new_blizzards = HashSet::new();
    for blizzard in blizzards {
        new_blizzards.insert(move_blizzard(blizzard, x, y));
    }
    new_blizzards
}

/// Simulates one round of blizzards moving and returns the new blizzards, the possible positions to go to and the minute
fn simulate_round(
    blizzards: HashSet<Blizzard>,
    max_x: u32,
    max_y: u32,
    visited: HashSet<(u32, u32)>, (initial_x, initial_y): (u32, u32), minute: u32,
) -> (HashSet<Blizzard>, HashSet<(u32, u32)>, u32) {
    let new_blizzards = move_all_blizzards(blizzards, max_x, max_y);
    let mut possible = HashSet::new();
    // if no blizzard is at the starting position, add it to the possible set
    if !new_blizzards.contains(&Blizzard { x: initial_x, y: initial_y, dir: 0 })
        && !new_blizzards.contains(&Blizzard { x: initial_x, y: initial_y, dir: 1 })
        && !new_blizzards.contains(&Blizzard { x: initial_x, y: initial_y, dir: 2 })
        && !new_blizzards.contains(&Blizzard { x: initial_x, y: initial_y, dir: 3 })
    {
        possible.insert((initial_x, initial_y));
    }
    // consider all positions of visited
    for (x, y) in visited {
        // add all possible positions to go to from the current position if they are not out of bounds
        if x > 0 {
            possible.insert((x - 1, y));
        }
        if x < max_x {
            possible.insert((x + 1, y));
        }
        if y > 0 {
            possible.insert((x, y - 1));
        }
        if y < max_y {
            possible.insert((x, y + 1));
        }
        possible.insert((x, y));
    }

    // remove all positions in possible where a blizzard is
    for blizzard in new_blizzards.clone() {
        possible.remove(&(blizzard.x, blizzard.y));
    }

    (new_blizzards, possible, minute+1)
}

pub fn part_one(input: &str) -> Option<u32> {
    // parse the input
    let (max_x, max_y, mut blizzards) = parse_input(input);

    // create a set of visited positions
    let mut visited = HashSet::new();

    let mut minute = 0;

    let initial = (0,0);

    // loop until visited contains x, y
    while !visited.contains(&(max_x, max_y)) {
        (blizzards, visited, minute) = simulate_round(blizzards, max_x, max_y, visited, initial, minute);
    }

    // simulate one more round
    (_, _, minute) = simulate_round(blizzards, max_x, max_y, visited, initial, minute);

    Some(minute)
}

pub fn part_two(input: &str) -> Option<u32> {
    // parse the input
    let (max_x, max_y, mut blizzards) = parse_input(input);

    // create a set of visited positions
    let mut visited = HashSet::new();

    let mut minute = 0;

    let initial = (0,0);

    // loop until visited contains x_max, y_max
    while !visited.contains(&(max_x, max_y)) {
        // simulate a round
        (blizzards, visited, minute) = simulate_round(blizzards, max_x, max_y, visited, initial, minute);
    }

    (blizzards, visited, minute) = simulate_round(blizzards, max_x, max_y, visited, initial, minute);

    // clear the visited set
    visited.clear();

    // set initial
    let initial = (max_x, max_y);

    // loop until visited contains 0, 0
    while !visited.contains(&(0, 0)) {
        // simulate a round
        (blizzards, visited, minute) = simulate_round(blizzards, max_x, max_y, visited, initial, minute);
    }

    // add 1 minute for the last move
    (blizzards, visited, minute) = simulate_round(blizzards, max_x, max_y, visited, initial, minute);
    // clear the visited set
    visited.clear();

    // set initial
    let initial = (0, 0);

    // loop until visited contains x_max, y_max
    while !visited.contains(&(max_x, max_y)) {
        // simulate a round
        (blizzards, visited, minute) = simulate_round(blizzards, max_x, max_y, visited, initial, minute);
    }

    // add 1 minute for the last move
    (_, _, minute) = simulate_round(blizzards, max_x, max_y, visited, initial, minute);

    Some(minute)
}

fn main() {
    let input = &aoc::read_file("inputs", 24);
    aoc::solve!(24, 1, part_one, input);
    aoc::solve!(24, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
