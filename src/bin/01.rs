use itertools::Itertools;

struct Elve {
    calories: Vec<i32>,
}

impl Elve {
    fn get_sum(&self) -> i32 {
        self.calories.iter().sum()
    }
}

fn parse_elve (input: &str) -> Elve {
    let mut result = Elve {calories : vec![]};
    for line in input.split("\n") {
        // println!("Line is {:#?}", line.replace("\n", ""));
        result.calories.push(line.replace("\n", "").parse::<i32>().unwrap());
    }

    return result;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut elves: Vec<Elve> = vec![];

    for block in input.replace("\r", "").split("\n\n") {
        // println!("Block is {:#?}", block.replace("\n\n", "").as_str());
        elves.push(parse_elve(block.replace("\n\n", "").as_str()));
    }

    return Some(elves.iter().map(|elve| elve.get_sum()).max().unwrap() as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves: Vec<Elve> = vec![];

    for block in input.replace("\r", "").split("\n\n") {
        // println!("Block is {:#?}", block.replace("\n\n", "").as_str());
        elves.push(parse_elve(block.replace("\n\n", "").as_str()));
    }

    return Some(elves.iter().map(|elve| elve.get_sum()).sorted().skip(elves.len()-3).sum::<i32>() as u32);
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
