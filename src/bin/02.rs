use std::vec;

use itertools::Itertools;

/// Rock, paper, scissors game.
/// Rock has score 1, paper score 2, scissors score 3
/// Loosing increases score by 0, draw by 3, win by 6

/// Returns the score for part 1
fn get_score_pt_1(a: &str, b: &str) -> u32 {
    match a {
        "A" => match b {
            "X" => 1 + 3, // rock vs. rock means draw
            "Y" => 2 + 6, // rock vs. paper means win
            "Z" => 3, // rock vs. scissors means loose
            _ => {println!("Action unclear: {}", b); 0},
        },
        "B" => match b {
            "X" => 1, // paper vs. rock means loose
            "Y" => 2 + 3, // paper vs. paper means draw
            "Z" => 3 + 6, // paper vs. scissors means win
            _ => {println!("Action unclear: {}", b); 0},
        },
        "C" => match b {
            "X" => 1 + 6, // scissors vs. rock means win
            "Y" => 2, // scissors vs. paper means loose
            "Z" => 3 + 3, // scissors vs. scissors means draw
            _ => {println!("Action unclear: {}", b); 0},
        },
        _ => {println!("Action unclear: {}", a); 0},
    }
}

pub fn part_one(input: &str) -> Option<u32> {

    let mut scores: Vec<u32> = vec![];

    for line in input.split('\n') {
        // cleanup line
        let clean_line = line.replace(['\n', '\r'], "");
        if !clean_line.is_empty() {
            // get actions
            let actions: Vec<&str> = clean_line.split(' ').collect_vec();
            // add score
            scores.push(get_score_pt_1(actions[0], actions[1]));
        }
    }

    // return sum
    Some(scores.iter().sum())
}

/// Returns the score for part 2
fn get_score_pt_2(a: &str, b: &str) -> u32 {
    match a {
        "A" => match b {
            "X" => 3, // loose against rock means scissors
            "Y" => 1 + 3, // draw against rock means rock
            "Z" => 2 + 6, // win against rock means paper
            _ => {println!("Action unclear: {}", b); 0},
        },
        "B" => match b {
            "X" => 1, // loose against paper means rock
            "Y" => 2 + 3, // draw against paper means paper
            "Z" => 3 + 6, // win against paper means scissors
            _ => {println!("Action unclear: {}", b); 0},
        },
        "C" => match b {
            "X" => 2, // loose against scissors means paper
            "Y" => 3 + 3, // draw against scissors means scissors
            "Z" => 1 + 6, // win against scissors means rock
            _ => {println!("Action unclear: {}", b); 0},
        },
        _ => {println!("Action unclear: {}", a); 0},
    }
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut scores: Vec<u32> = vec![];

    for line in input.split('\n') {
        // cleanup line
        let clean_line = line.replace(['\n', '\r'], "");
        if !clean_line.is_empty() {
            // get actions
            let actions: Vec<&str> = clean_line.split(' ').collect_vec();
            // add score
            scores.push(get_score_pt_2(actions[0], actions[1]));
        }
    }

    // return sum
    Some(scores.iter().sum())
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(2, 1, part_one, input);
    aoc::solve!(2, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
