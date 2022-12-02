use std::vec;

use itertools::Itertools;

fn get_score_pt_1(A: &str, B: &str) -> u32 {
    match A {
        "A" => match B {
            "X" => 1 + 3,
            "Y" => 2 + 6,
            "Z" => 3 + 0,
            _ => {println!("Action unclear: {}", B); return 0;},
        },
        "B" => match B {
            "X" => 1 + 0,
            "Y" => 2 + 3,
            "Z" => 3 + 6,
            _ => {println!("Action unclear: {}", B); return 0;},
        },
        "C" => match B {
            "X" => 1 + 6,
            "Y" => 2 + 0,
            "Z" => 3 + 3,
            _ => {println!("Action unclear: {}", B); return 0;},
        },
        _ => {println!("Action unclear: {}", A); return 0;},
    }
}

pub fn part_one(input: &str) -> Option<u32> {

    let mut scores: Vec<u32> = vec![];

    for line in input.split("\n") {
        let clean_line = line.replace("\n", "").replace("\r", "");
        if clean_line.len()>0 {
            let actions: Vec<&str> = clean_line.split(" ").collect_vec();
            scores.push(get_score_pt_1(actions[0], actions[1]));
        }
    }

    Some(scores.iter().sum())
}

fn get_score_pt_2(A: &str, B: &str) -> u32 {
    match A {
        "A" => match B {
            "X" => 3 + 0,
            "Y" => 1 + 3,
            "Z" => 2 + 6,
            _ => {println!("Action unclear: {}", B); return 0;},
        },
        "B" => match B {
            "X" => 1 + 0,
            "Y" => 2 + 3,
            "Z" => 3 + 6,
            _ => {println!("Action unclear: {}", B); return 0;},
        },
        "C" => match B {
            "X" => 2 + 0,
            "Y" => 3 + 3,
            "Z" => 1 + 6,
            _ => {println!("Action unclear: {}", B); return 0;},
        },
        _ => {println!("Action unclear: {}", A); return 0;},
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut scores: Vec<u32> = vec![];

    for line in input.split("\n") {
        let clean_line = line.replace("\n", "").replace("\r", "");
        if clean_line.len()>0 {
            let actions: Vec<&str> = clean_line.split(" ").collect_vec();
            scores.push(get_score_pt_2(actions[0], actions[1]));
        }
    }

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
