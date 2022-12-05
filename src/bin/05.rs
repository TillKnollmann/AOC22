use itertools::Itertools;

/// Returns the puzzle input for today
fn get_puzzle_crates() -> Vec<Vec<char>> {
    let crates: Vec<Vec<char>> = vec![
        vec!['B', 'G', 'S', 'C'],
        vec!['T', 'M', 'W', 'H', 'J', 'N', 'V', 'G'],
        vec!['M', 'Q', 'S'],
        vec!['B', 'S', 'L', 'T', 'W', 'N', 'M'],
        vec!['J', 'Z', 'F', 'T', 'V', 'G', 'W', 'P'],
        vec!['C', 'T', 'B', 'G', 'Q', 'H', 'S'],
        vec!['T', 'J', 'P', 'B', 'W'],
        vec!['G', 'D', 'C', 'Z', 'F', 'T', 'Q', 'M'],
        vec!['N', 'S', 'H', 'B', 'P', 'F'],
    ];

    crates
}

/// Interprets a command for part 1 and executes the movements on crates
fn do_command_pt1(command: &str, crates: &mut [Vec<char>]) {
    let split = command.split_whitespace().collect_vec();
    if split[0] =="move" {
        let (num, src, dst) = (split[1].parse::<i32>().unwrap(), split[3].parse::<i32>().unwrap()-1, split[5].parse::<i32>().unwrap()-1);
        for _ in 0..num {
            let item = crates.get_mut(src as usize).unwrap().pop().unwrap();
            crates.get_mut(dst as usize).unwrap().push(item);
        }
    }
}

/// Interprets a command for part 2 and executes the movements on crates
fn do_command_pt2(command: &str, crates: &mut [Vec<char>]) {
    let split = command.split_whitespace().collect_vec();
    if split[0] =="move" {
        let (num, src, dst) = (split[1].parse::<i32>().unwrap(), split[3].parse::<i32>().unwrap()-1, split[5].parse::<i32>().unwrap()-1);
        let length = crates.get(src as usize).unwrap().len() - (num as usize);
        let mut items = crates.get_mut(src as usize).unwrap().drain(length..).collect_vec();
        crates.get_mut(dst as usize).unwrap().append(&mut items);
    }
}

/// Returns a string concatenating all top crates
fn get_top_crates(crates: &[Vec<char>]) -> String {
    crates.iter().map(|vec| vec.last().unwrap()).join("")
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut crates = get_puzzle_crates();

    for line in input.lines() {
        if !line.is_empty() {
            do_command_pt1(line, &mut crates);
        }
    }

    println!("Result is {}", get_top_crates(&crates));
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut crates = get_puzzle_crates();

    for line in input.lines() {
        if !line.is_empty() {
            do_command_pt2(line, &mut crates);
        }
    }

    println!("Result is {}", get_top_crates(&crates));
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(5, 1, part_one, input);
    aoc::solve!(5, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
