/// Returns the rock as a vector of vectors of booleans which has its turn for the given number
fn get_rock(i: u8) -> Vec<Vec<bool>> {
    let idx = i % 5;
    match idx {
        0 => vec![vec![true], vec![true], vec![true], vec![true]],
        1 => vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        2 => vec![
            vec![true, false, false],
            vec![true, false, false],
            vec![true, true, true],
        ],
        3 => vec![vec![true, true, true, true]],
        4 => vec![vec![true, true], vec![true, true]],
        _ => vec![],
    }
}

/// Returns true if the rock can be placed at the given position without colliding with the cave
fn can_be_placed(cave: &[Vec<bool>], rock: &[Vec<bool>], x: usize, y: usize) -> bool {
    let mut can_be_placed = true;
    for (i, row) in rock.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell && cave[x + i][y + j] {
                can_be_placed = false;
            }
        }
    }
    can_be_placed
}

/// Places the rock at the given position and returns the new cave
fn place_rock(cave: &[Vec<bool>], rock: &[Vec<bool>], x: usize, y: usize) -> Vec<Vec<bool>> {
    let mut new_cave = cave.to_vec();
    for (i, row) in rock.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell {
                new_cave[x + i][y + j] = true;
            }
        }
    }
    new_cave
}

/// Simulates the rock falling down and returns the new cave, the new height and the remaining push commands
fn simulate_rock(
    cave: &mut [Vec<bool>],
    height: usize,
    number: usize,
    push: Vec<char>,
) -> (Vec<Vec<bool>>, usize, Vec<char>) {
    let rock = get_rock((number % 5).try_into().unwrap());

    // println!("Rock {} falls with push ", number);
    // print_rock(&rock);

    let rock_width = rock.len() as u32;
    let rock_height = rock[0].len() as u32;

    // get rock's starting position
    let mut x: usize = 2;
    let mut y = height + 3;

    // extend the cave if necessary
    if cave[0].len() < (y + rock_height as usize) {
        for row in cave.iter_mut() {
            row.extend(vec![false; (y + rock_height as usize) - row.len()]);
        }
    }

    let mut command_idx = 0;

    // initial push to the rock
    (x, y) = process_command(&mut x, &mut y, rock_width, push[command_idx]);

    let mut last_x = x;
    let mut last_y = y;

    // check if rock can be placed in a loop
    while can_be_placed(cave, &rock, x, y) {
        last_x = x;
        last_y = y;

        // let the rock fall
        if y == 0 {
            break;
        } else {
            y -= 1;
        }

        if can_be_placed(cave, &rock, x, y) {
            // if the rock can be placed, check if there is a push command
            last_x = x;
            last_y = y;
            (x, y) = process_command(&mut x, &mut y, rock_width, push[command_idx + 1]);
            command_idx += 1;
            if !can_be_placed(cave, &rock, x, y) {
                // if the rock cannot be placed after the push, revert the last push
                x = last_x;
                y = last_y;
            }
        }
    }

    // apply the rock to the cave
    let cave = place_rock(cave, &rock, last_x, last_y);

    // calculate the new height of the cave
    let new_height: usize = cave
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(idx, elem)| if *elem { idx + 1 } else { 0 })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    (cave, new_height, push[command_idx + 1..].to_vec())
}

/// Processes the given command and returns the new position of the rock
fn process_command(x: &mut usize, y: &mut usize, rock_width: u32, command: char) -> (usize, usize) {
    match command {
        '>' => {
            if *x + (rock_width as usize) < 7_usize {
                *x += 1;
            }
        }
        '<' => {
            if *x > 0 {
                *x -= 1;
            }
        }
        _ => {}
    };
    (*x, *y)
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.lines().next().unwrap().to_string();

    let mut cave = vec![vec![false; 20]; 7];

    let mut height = 0;
    let mut commands = input.chars().collect::<Vec<char>>();

    for i in 0..2022 {
        (cave, height, commands) = simulate_rock(&mut cave, height, i, commands);
        if commands.len() <= input.len() {
            commands.append(input.chars().collect::<Vec<char>>().as_mut());
        }
    }
    Some(height as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.lines().next().unwrap().to_string();

    let mut cave = vec![vec![false; 20]; 7];

    let mut height: usize = 0;
    let mut commands = input.chars().collect::<Vec<char>>();

    let mut limit: usize = 1_000_000_000_000;
    let mut count = 0;

    let mut heights: Vec<usize> = vec![0];
    let mut rock_number: Vec<usize> = vec![0];
    let mut delta_heights: Vec<usize> = vec![];
    let mut delta_rocks: Vec<usize> = vec![];

    let mut height_offset: usize = 0;

    while count < limit {
        if commands[0..input.chars().count()] == input.chars().collect::<Vec<char>>() {
            // store the height and the delta
            heights.push(height);
            rock_number.push(count);
            delta_heights.push(heights[heights.len()-1] - heights[heights.len()-2]);
            delta_rocks.push(rock_number[rock_number.len()-1] - rock_number[rock_number.len()-2]);

            if delta_heights.len() > 2 && delta_heights[delta_heights.len()-1] == delta_heights[delta_heights.len()-2] && delta_rocks[delta_rocks.len()-1] == delta_rocks[delta_rocks.len()-2] {
                // we found the delta, so we can advance the calculation
                let delta_height = delta_heights[delta_heights.len()-1];
                let delta_rock = delta_rocks[delta_rocks.len()-1];
                let remaining_rocks = limit - count;
                let steps = remaining_rocks/delta_rock;
                // store height that will be generated by repeated rocks
                height_offset = steps*delta_height;
                // shrink the limit by the number of skipped calculations
                limit -= steps*delta_rock;
            }
        }
        (cave, height, commands) = simulate_rock(&mut cave, height, count, commands);
        if commands.len() <= input.len() {
            commands.append(input.chars().collect::<Vec<char>>().as_mut());
        }
        count += 1;
    }

    // The result is too large to be cast to u32 again :D
    println!("The height is: {}", height  + height_offset);
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 17);
    aoc::solve!(17, 1, part_one, input);
    aoc::solve!(17, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }
}
