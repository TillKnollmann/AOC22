use itertools::Itertools;
use std::vec;

/// Simulates all commands and returns a vector of x values during each cycle
fn simulate_commands(input: &str) -> Vec<i32> {
    let mut x = 1;
    let mut cycle = 0;
    let mut x_values: Vec<i32> = vec![];
    for command in input.lines() {
        if !command.is_empty() {
            let (res_cycle, res_x) = simulate_one_command(command, cycle, x);
            for _ in 0..res_cycle - cycle {
                x_values.push(x);
            }
            x = res_x;
            cycle = res_cycle;
        }
    }
    x_values.push(x);

    x_values
}

/// returns the signal strength
fn get_signal_strength(init: u32, step: u32, x_values: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    (0..x_values.len()).for_each(|i| {
        if i + 1 == init as usize
            || (i + 1 > init as usize && (i + 1 - init as usize) % step as usize == 0)
        {
            res.push(x_values[i] * (i as i32 + 1));
        }
    });

    res
}

/// Simulates one command and returns the cycle number after the command and the x value after the command
fn simulate_one_command(command: &str, cycle: u32, x: i32) -> (u32, i32) {
    let split = command.split_whitespace().collect_vec();
    match split[0] {
        "noop" => (cycle + 1, x),
        _ => (cycle + 2, x + split[1].parse::<i32>().unwrap()),
    }
}

/// returns (and draws if wanted) the CRT output
fn draw(x_values: Vec<i32>, draw: bool) -> Vec<Vec<char>> {
    let mut lines: Vec<Vec<char>> = vec![];
    let mut line = vec![];

    (0..x_values.len()).for_each(|i| {
        if i % 40 == 0 && i > 0 {
            lines.push(line.clone());
            line = vec![];
        }
        let pos = i % 40; // position of the CRT
        let value = x_values[i]; // x during the i+1 st cycle
        let mut c = ' ';
        if value - 1 <= pos as i32 && pos as i32 <= value + 1 {
            c = '#';
        }
        line.push(c);
    });

    if draw {
        for line in lines.clone() {
            println!("{}", line.iter().cloned().collect::<String>());
        }
    }

    lines
}

/// Returns the example output for part 2
fn _get_example_pt2_sol() -> Vec<Vec<char>> {
    vec![
        "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  "
            .chars()
            .collect_vec(),
        "###   ###   ###   ###   ###   ###   ### "
            .chars()
            .collect_vec(),
        "####    ####    ####    ####    ####    "
            .chars()
            .collect_vec(),
        "#####     #####     #####     #####     "
            .chars()
            .collect_vec(),
        "######      ######      ######      ####"
            .chars()
            .collect_vec(),
        "#######       #######       #######     "
            .chars()
            .collect_vec(),
    ]
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        get_signal_strength(20, 40, simulate_commands(input))
            .iter()
            .sum::<i32>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    draw(simulate_commands(input), true);
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(10, 1, part_one, input);
    aoc::solve!(10, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(
            draw(simulate_commands(&input), false),
            _get_example_pt2_sol()
        );
    }
}
