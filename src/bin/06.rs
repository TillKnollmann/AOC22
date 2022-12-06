use std::collections::HashSet;

/// Returns the first marker position where the last 4 characters are different
fn get_first_marker_pos(input: &str) -> u32 {
    let (_, (_, (_, (idx, _)))) = input.chars().zip(input.chars().skip(1).zip(input.chars().skip(2).zip(input.chars().skip(3).enumerate()))).find(|(c1, (c2, (c3, (_, c4))))| {let set: HashSet<char> = HashSet::from([*c1,*c2,*c3,*c4]); matches!(set.len(),4)}).unwrap();
   (idx + 3 + 1) as u32
}

/// Returns the first packet position where the last 14 characters are different
fn get_first_packet_pos(input: &str) -> u32 {
    let mut index: u32 = 13;
    let mut found = false;
    while !found {
        index += 1;
        if HashSet::<char>::from_iter(input.chars().enumerate().filter_map(|(idx, c)| match index -14 <= idx as u32 && (idx as u32) < index {
            true => Some(c),
            false => None,
        })).len() == 14 {
            found = true;
        }
    }
    index
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(get_first_marker_pos(input.lines().next().unwrap()))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(get_first_packet_pos(input.lines().next().unwrap()))
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(6, 1, part_one, input);
    aoc::solve!(6, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 6);
        let mut lines = input.lines();
        assert_eq!(part_one(lines.next().unwrap()), Some(7));
        assert_eq!(part_one(lines.next().unwrap()), Some(5));
        assert_eq!(part_one(lines.next().unwrap()), Some(6));
        assert_eq!(part_one(lines.next().unwrap()), Some(10));
        assert_eq!(part_one(lines.next().unwrap()), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 6);
        let mut lines = input.lines();
        assert_eq!(part_two(lines.next().unwrap()), Some(19));
        assert_eq!(part_two(lines.next().unwrap()), Some(23));
        assert_eq!(part_two(lines.next().unwrap()), Some(23));
        assert_eq!(part_two(lines.next().unwrap()), Some(29));
        assert_eq!(part_two(lines.next().unwrap()), Some(26));
    }
}
