use itertools::Itertools;

/// Finds the double items in a rucksack
fn find_double_item(rucksack: &str) -> char {
    let first_half = rucksack.chars().count()/2;
    rucksack.chars().take(first_half).find_or_first(|item| rucksack.chars().skip(first_half).contains(item)).unwrap()
}

/// returns the priority of a char
fn get_priority(item: char) -> u32 {
    if item.is_lowercase() {
        // Lower case characters start at 97
        item as u32 - 96
    } else {
        // Upper case characters start at 65 and have an offset of 26
        item as u32 - 64 + 26
    }
}

/// returns the priority of a rucksack
fn get_rucksack_priority(rucksack: &str) -> u32 {
    get_priority(find_double_item(rucksack))
}

/// returns the group badge of three elves
fn get_group_badge(elve_a: &str, elve_b: &str, elve_c: &str) -> char {
    let match_a_b = elve_a.chars().filter(|item| elve_b.contains(*item)).collect_vec();
    elve_c.chars().find_or_first(|item| match_a_b.contains(item)).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(get_rucksack_priority).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().chunks(3).into_iter().map(|mut elves| get_priority(get_group_badge(elves.next().unwrap(), elves.next().unwrap(), elves.next().unwrap()))).sum())
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(3, 1, part_one, input);
    aoc::solve!(3, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_item() {
        let input = aoc::read_file("examples", 3);
        let mut lines = input.lines();
        assert_eq!(find_double_item(lines.next().unwrap()), 'p');
        assert_eq!(find_double_item(lines.next().unwrap()), 'L');
        assert_eq!(find_double_item(lines.next().unwrap()), 'P');
        assert_eq!(find_double_item(lines.next().unwrap()), 'v');
        assert_eq!(find_double_item(lines.next().unwrap()), 't');
        assert_eq!(find_double_item(lines.next().unwrap()), 's');
    }

    #[test]
    fn test_char_priority() {
        let input = aoc::read_file("examples", 3);
        let mut lines = input.lines();
        assert_eq!(get_rucksack_priority(lines.next().unwrap()), 16);
        assert_eq!(get_rucksack_priority(lines.next().unwrap()), 38);
        assert_eq!(get_rucksack_priority(lines.next().unwrap()), 42);
        assert_eq!(get_rucksack_priority(lines.next().unwrap()), 22);
        assert_eq!(get_rucksack_priority(lines.next().unwrap()), 20);
        assert_eq!(get_rucksack_priority(lines.next().unwrap()), 19);
    }

    #[test]
    fn test_badge_find() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(get_group_badge(input.lines().next().unwrap(), input.lines().nth(1).unwrap(), input.lines().nth(2).unwrap()), 'r');
        assert_eq!(get_group_badge(input.lines().nth(3).unwrap(), input.lines().nth(4).unwrap(), input.lines().nth(5).unwrap()), 'Z');
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
