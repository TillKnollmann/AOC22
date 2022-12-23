use std::collections::HashMap;

/// parses the input and returns the map as a vector of vectors of chars
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Simulates a single round and returns the number of movements
fn simulate_round(map: &mut Vec<Vec<char>>, start_dir: u8) -> u32 {
    // extend map is needed
    let mut extend = false;
    let (min_x, max_x, min_y, max_y) = get_range(map);
    if min_x == 0 || max_x == map[0].len() - 1 || min_y == 0 || max_y == map.len() - 1 {
        extend = true;
    }
    if extend {
        let mut new_map = vec![vec!['.'; map[0].len() + 2]; map.len() + 2];
        for (y, line) in map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                new_map[y + 1][x + 1] = *c;
            }
        }
        *map = new_map;
    }

    // track proposed movements
    let mut proposed: Vec<Vec<u8>> = vec![vec![0; map[0].len()]; map.len()];
    let mut movements: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    // for each # in the map check the 8 neighbors
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '#' {
                // if there are no neighbors, add 1 to current position
                let mut count = 0;
                for i in 0..8 {
                    let (dx, dy) = match i {
                        0 => (1, 0),
                        1 => (1, 1),
                        2 => (0, 1),
                        3 => (-1, 1),
                        4 => (-1, 0),
                        5 => (-1, -1),
                        6 => (0, -1),
                        7 => (1, -1),
                        _ => unreachable!(),
                    };
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                    if nx >= 0
                        && nx < map[0].len() as i32
                        && ny >= 0
                        && ny < map.len() as i32
                        && map[ny as usize][nx as usize] == '#'
                    {
                        count += 1;
                    }
                }
                if count == 0 {
                    proposed[y][x] += 1;
                } else {
                    for i in 0..4 {
                        let dir = (start_dir + i) % 4;
                        match dir {
                            0 => {
                                // North
                                // check if there is a # in the three positions above
                                if map[y - 1][x - 1] != '#'
                                    && map[y - 1][x] != '#'
                                    && map[y - 1][x + 1] != '#'
                                {
                                    proposed[y - 1][x] += 1;
                                    movements.insert((y, x), (y - 1, x));
                                    break;
                                }
                            }
                            3 => {
                                // East
                                // check if there is a # in the three positions to the right
                                if map[y - 1][x + 1] != '#'
                                    && map[y][x + 1] != '#'
                                    && map[y + 1][x + 1] != '#'
                                {
                                    proposed[y][x + 1] += 1;
                                    movements.insert((y, x), (y, x + 1));
                                    break;
                                }
                            }
                            1 => {
                                // South
                                // check if there is a # in the three positions below
                                if map[y + 1][x - 1] != '#'
                                    && map[y + 1][x] != '#'
                                    && map[y + 1][x + 1] != '#'
                                {
                                    proposed[y + 1][x] += 1;
                                    movements.insert((y, x), (y + 1, x));
                                    break;
                                }
                            }
                            2 => {
                                // West
                                // check if there is a # in the three positions to the left
                                if map[y - 1][x - 1] != '#'
                                    && map[y][x - 1] != '#'
                                    && map[y + 1][x - 1] != '#'
                                {
                                    proposed[y][x - 1] += 1;
                                    movements.insert((y, x), (y, x - 1));
                                    break;
                                }
                            }
                            _ => unreachable!(),
                        };
                        // if we get here, we didn't find a place to move
                        // so we add 1 to the current position
                        proposed[y][x] += 1;
                    }
                }
            }
        }
    }

    let mut movement_count = 0;

    // potentially apply the movements
    for (y, x) in movements.keys() {
        let (ny, nx) = movements.get(&(*y, *x)).unwrap();
        if proposed[*ny][*nx] == 1 {
            map[*y][*x] = '.';
            map[*ny][*nx] = '#';
            movement_count += 1;
        }
    }

    movement_count
}

/// Returns the min and max x and y values where there is a #
fn get_range(map: &[Vec<char>]) -> (usize, usize, usize, usize) {
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '#' {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }
    (min_x, max_x, min_y, max_y)
}

/// Returns the number of . in the map within the range of #
fn get_uncovered(map: &[Vec<char>]) -> usize {
    // get the ranges
    let (min_x, max_x, min_y, max_y) = get_range(map);
    // find within the range the number of .
    let mut count = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '.' && x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                count += 1;
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse_input(input);

    let mut dir: u8 = 0;

    for _ in 0..10 {
        simulate_round(&mut map, dir);
        dir = (dir + 1) % 4;
    }
    let result = get_uncovered(&map);

    if let Ok(val) = TryInto::<u32>::try_into(result) {
        Some(val)
    } else {
        println!("Could not convert {} to u32", result);
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse_input(input);

    let mut round_number: usize = 0;
    let mut last_movement_count = 1;

    let mut dir: u8 = 0;

    while last_movement_count != 0 {
        last_movement_count = simulate_round(&mut map, dir);
        round_number += 1;
        dir = (dir + 1) % 4;
    }

    if let Ok(val) = TryInto::<u32>::try_into(round_number) {
        Some(val)
    } else {
        println!("Could not convert {} to u32", round_number);
        None
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 23);
    aoc::solve!(23, 1, part_one, input);
    aoc::solve!(23, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
