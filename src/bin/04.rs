
/// Returns the 4 u32 values defining the two sections of a pair
fn parse_sections(input: &str) -> (u32, u32, u32, u32) {
    let split = input.split(|c| c == ',' || c == '-').collect::<Vec<&str>>();
    (split[0].parse::<u32>().unwrap(), split[1].parse::<u32>().unwrap(), split[2].parse::<u32>().unwrap(), split[3].parse::<u32>().unwrap())
}

/// Returns true iff for [a1, a2] and [b1, b2], one is contained fully in the other
fn is_contained((a1, a2, b1, b2): (u32, u32, u32, u32)) -> bool {
    (a1 <= b1 && b2 <= a2) || (b1 <= a1 && a2 <= b2)
}

/// Returns true iff [a1, a2] and [b1, b2] are overlapping
fn is_overlapping((a1, a2, b1, b2): (u32, u32, u32, u32)) -> bool {
    a1 <= b2 && a2 >= b1
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| { if is_contained(parse_sections(line)) { return 1; } 0}).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| { if is_overlapping(parse_sections(line)) { return 1; } 0}).sum())
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(4, 1, part_one, input);
    aoc::solve!(4, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = aoc::read_file("examples", 4);
        let mut lines = input.lines();
        assert_eq!(parse_sections(lines.next().unwrap()), (2, 4, 6, 8));
        assert_eq!(parse_sections(lines.next().unwrap()), (2, 3, 4, 5));
        assert_eq!(parse_sections(lines.next().unwrap()), (5, 7, 7, 9));
        assert_eq!(parse_sections(lines.next().unwrap()), (2, 8, 3, 7));
        assert_eq!(parse_sections(lines.next().unwrap()), (6, 6, 4, 6));
        assert_eq!(parse_sections(lines.next().unwrap()), (2, 6, 4, 8));
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
