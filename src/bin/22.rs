
/// Expresses a player state
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Player {
    x: i32,
    y: i32,
    direction: u8,
}

/// Parses the input and returns a map and a string of commands
fn parse_input(input: &str) -> (Vec<Vec<char>>, &str) {
    let mut lines = input.lines();

    let mut map: Vec<Vec<char>> = Vec::new();

    let mut max_length = 0;

    // while the next line is not empty, parse the line
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        //
        let row: Vec<char> = line.chars().collect();
        map.push(row);
        max_length = max_length.max(line.chars().count());
    }

    // pad the map with empty spaces
    for row in map.iter_mut() {
        while row.len() < max_length {
            row.push(' ');
        }
    }

    let commands = lines.next().unwrap();

    (map, commands)
}

/// Returns the initial player state
fn get_player(map: &[Vec<char>]) -> Player {
    let mut player = Player {
        x: 0,
        y: 0,
        direction: 1,
    };

    // find the first position
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '.' {
                player.y = y as i32;
                player.x = x as i32;
                return player;
            }
        }
    }

    panic!("Invalid map");
}

/// Processes one command
fn process_command(player: &mut Player, map: &mut [Vec<char>], command: &str, cube: bool) {
    match command {
        "R" => {
            player.direction = (player.direction + 1) % 4;
        }
        "L" => {
            player.direction = (player.direction + 3) % 4;
        }
        _ => {
            let steps = command.parse::<u32>().unwrap();
            simulate_steps(player, map, steps, cube);
        }
    }
}

/// Simulates the steps into one direction
fn simulate_steps(player: &mut Player, map: &mut [Vec<char>], steps: u32, cube: bool) {
    for _ in 0..steps {
        let (new_x, new_y, new_dir) = if !cube {
            get_wrapping_classic(player.x, player.y, map, player.direction)
        } else {
            get_wrapping_cube(player.x, player.y, map, player.direction)
        };
        // check if a jump happened
        if (new_x - player.x).abs() + (new_y - player.y).abs() > 1 {
            // check if the jump is valid
            if !is_border(new_x, new_y, map) {
                panic!("Player did not move to a border from {:?} to {:?}", (player.x, player.y), (new_x, new_y));
            }
        }

        player.x = new_x;
        player.y = new_y;
        player.direction = new_dir;
    }
}

/// Gets the wrapping position in the classic mode
fn get_wrapping_classic(x: i32, y: i32, map: &[Vec<char>], direction: u8) -> (i32, i32, u8) {
    let old_x = x;
    let old_y = y;
    let mut new_x: i32;
    let mut new_y: i32;
    match direction {
        0 => {
            new_y = y - 1;
            if new_y < 0 {
                new_y = map.len() as i32 - 1;
            }
            match map[new_y as usize][old_x as usize] {
                '#' => (old_x, old_y, direction),
                '.' => (old_x, new_y, direction),
                ' ' => {
                    // find the next position
                    match map
                        .iter()
                        .enumerate()
                        .skip(old_y as usize)
                        .rev()
                        .find(|(_, row)| row[old_x as usize] == '#' || row[old_x as usize] == '.')
                    {
                        Some((y, _)) => {
                            if map[y][old_x as usize] == '#' {
                                (old_x, old_y, direction)
                            } else {
                                (old_x, y as i32, direction)
                            }
                        }
                        None => {
                            panic!("Invalid map");
                        }
                    }
                }
                _ => panic!("Invalid character"),
            }
        }
        1 => {
            new_x = x + 1;
            if new_x >= map[old_y as usize].len() as i32 {
                new_x = 0;
            }
            match map[old_y as usize][new_x as usize] {
                '#' => (old_x, old_y, direction),
                '.' => (new_x, old_y, direction),
                ' ' => {
                    // find the next position
                    match map[old_y as usize]
                        .iter()
                        .enumerate()
                        .find(|(_, c)| **c == '#' || **c == '.')
                    {
                        Some((x, _)) => {
                            if map[old_y as usize][x] == '#' {
                                (old_x, old_y, direction)
                            } else {
                                (x as i32, old_y, direction)
                            }
                        }
                        None => {
                            panic!("Invalid map");
                        }
                    }
                }
                _ => panic!("Invalid character"),
            }
        }
        2 => {
            new_y = y + 1;
            if new_y >= map.len() as i32 {
                new_y = 0;
            }
            match map[new_y as usize][old_x as usize] {
                '#' => (old_x, old_y, direction),
                '.' => (old_x, new_y, direction),
                ' ' => {
                    // find the next position
                    match map
                        .iter()
                        .enumerate()
                        .find(|(_, row)| row[old_x as usize] == '#' || row[old_x as usize] == '.')
                    {
                        Some((y, _)) => {
                            if map[y][old_x as usize] == '#' {
                                (old_x, old_y, direction)
                            } else {
                                (old_x, y as i32, direction)
                            }
                        }
                        None => {
                            panic!("Invalid map");
                        }
                    }
                }
                _ => panic!("Invalid character"),
            }
        }
        3 => {
            new_x = x - 1;
            if new_x < 0 {
                new_x = map[old_y as usize].len() as i32 - 1;
            }
            match map[old_y as usize][new_x as usize] {
                '#' => (old_x, old_y, direction),
                '.' => (new_x, old_y, direction),
                ' ' => {
                    // find the next position
                    match map[old_y as usize]
                        .iter()
                        .enumerate()
                        .skip(old_x as usize)
                        .rev()
                        .find(|(_, c)| **c == '#' || **c == '.')
                    {
                        Some((x, _)) => {
                            if map[old_y as usize][x] == '#' {
                                (old_x, old_y, direction)
                            } else {
                                (x as i32, old_y, direction)
                            }
                        }
                        None => {
                            panic!("Invalid map");
                        }
                    }
                }
                _ => panic!("Invalid character"),
            }
        }
        _ => panic!("Invalid direction"),
    }
}

/// Gets the wrapping position in the cube mode
fn get_wrapping_cube(x: i32, y: i32, map: &[Vec<char>], direction: u8) -> (i32, i32, u8) {
    let old_x = x;
    let old_y = y;
    let mut new_x: i32;
    let mut new_y: i32;
    match direction {
        0 => {
            new_y = y - 1;
            if new_y < 0 {
                new_y = map.len() as i32 - 1;
            }
            match map[new_y as usize][old_x as usize] {
                '#' => (old_x, old_y, direction),
                '.' => (old_x, new_y, direction),
                ' ' => {
                    // find the wrapping position
                    // we move to the top of the map
                    if old_x < 50 {
                        // we are in the left part
                        new_y = 50 + old_x;
                        new_x = 50;
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 1),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }
                    } else if old_x < 100 {
                        // we are in the middle part
                        new_y = 150 + old_x - 50;
                        new_x = 0;
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 1),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }

                    } else {
                        // we are in the right part
                        new_x = old_x - 100;
                        new_y = map.len() as i32 - 1;
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 0),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }
                    }
                }
                _ => panic!("Invalid character"),
            }
        }
        1 => {
            new_x = x + 1;
            if new_x >= map[old_y as usize].len() as i32 {
                new_x = 0;
            }
            match map[old_y as usize][new_x as usize] {
                '#' => (old_x, old_y, direction),
                '.' => (new_x, old_y, direction),
                ' ' => {
                    // find the wrapping position
                    if y < 50 {
                        // we are in the top part
                        new_x = 99;
                        new_y = 149 - old_y;
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 3),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }

                    } else if y < 100 {
                        // we are in the middle part
                        new_y = 49;
                        new_x = 100 + old_y - 50;
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 0),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }

                    } else if y < 150{
                        // we are in the bottom part
                        new_x = 149;
                        new_y = 49 - (old_y - 100);
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 3),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }
                    } else {
                        // we are in the lowest part
                        new_y = 149;
                        new_x = 49 + old_y - 149;
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 0),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }
                    }
                }
                _ => panic!("Invalid character"),
            }
        }
        2 => {
            new_y = y + 1;
            if new_y >= map.len() as i32 {
                new_y = 0;
            }
            match map[new_y as usize][old_x as usize] {
                '#' => (old_x, old_y, direction),
                '.' => (old_x, new_y, direction),
                ' ' => {
                    // find the wrapping position
                    if x < 50 {
                        // we are in the left part
                        new_x = 100 + old_x;
                        new_y = 0;
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 2),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }
                    } else if x < 100 {
                        // we are in the middle part
                        new_x = 49;
                        new_y = 149 + (old_x - 49);
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 3),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }
                    } else {
                        // we are in the right part
                        new_x = 99;
                        new_y = 49 + (old_x - 99);
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 3),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }
                    }
                }
                _ => panic!("Invalid character"),
            }
        }
        3 => {
            new_x = x - 1;
            if new_x < 0 {
                new_x = map[old_y as usize].len() as i32 - 1;
            }
            match map[old_y as usize][new_x as usize] {
                '#' => (old_x, old_y, direction),
                '.' => (new_x, old_y, direction),
                ' ' => {
                    // find the wrapping position
                    if y < 50 {
                        // top part
                        new_x = 0;
                        new_y = 100 + (49 - old_y);
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 1),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }
                    } else if y < 100 {
                        // middle top part
                        new_y = 100;
                        new_x = old_y - 50;
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 2),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }
                    } else if y < 150{
                        // middle bot part
                        new_x = 50;
                        new_y = 149 - old_y;
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 1),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }
                    } else {
                        // bot part
                        new_y = 0;
                        new_x = 50 + old_y - 150;
                        match map[new_y as usize][new_x as usize] {
                            '#' => (old_x, old_y, direction),
                            '.' => (new_x, new_y, 2),
                            ' ' => panic!("Invalid map"),
                            _ => panic!("Invalid character"),
                        }
                    }
                }
                _ => panic!("Invalid character"),
            }
        }
        _ => panic!("Invalid direction"),
    }
}

/// Returns true if the position is on the border of the map
fn is_border(x: i32, y: i32, map: &[Vec<char>]) -> bool {
    if x == 0 || x == map[0].len() as i32 - 1 {
        return true;
    }
    if y == 0 || y == map.len() as i32 - 1 {
        return true;
    }
    // true if map at surrounding positions is a space
    if map[y as usize - 1][x as usize] == ' ' {
        return true;
    }
    if map[y as usize + 1][x as usize] == ' ' {
        return true;
    }
    if map[y as usize][x as usize - 1] == ' ' {
        return true;
    }
    if map[y as usize][x as usize + 1] == ' ' {
        return true;
    }
    false
}


pub fn part_one(input: &str) -> Option<u32> {
    // parse the input
    let (mut map, commands) = parse_input(input);

    // get player
    let mut player = get_player(&map);

    // split the commands into a vector. A command is either R, L or a number
    let binding = commands.replace('R', ",R,").replace('L', ",L,");
    let commands = binding.split(',').collect::<Vec<&str>>();

    for command in commands {
        if command.is_empty() {
            continue;
        }
        process_command(&mut player, &mut map, command, false);
        // print_map(map.as_slice(), &player);
    }

    let result = ((player.direction + 3) % 4) as u32
    + (player.y as u32 + 1) * 1000
    + (player.x as u32 + 1) * 4;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    // parse the input
    let (mut map, commands) = parse_input(input);

    // get player
    let mut player = get_player(&map);

    // split the commands into a vector. A command is either R, L or a number
    let binding = commands.replace('R', ",R,").replace('L', ",L,");
    let commands = binding.split(',').collect::<Vec<&str>>();

    for command in commands {
        if command.is_empty() {
            continue;
        }
        process_command(&mut player, &mut map, command, true);
        // print_map(map.as_slice(), &player);
    }

    let result = ((player.direction + 3) % 4) as u32
    + (player.y as u32 + 1) * 1000
    + (player.x as u32 + 1) * 4;

    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 22);
    aoc::solve!(22, 1, part_one, input);
    aoc::solve!(22, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }
}
