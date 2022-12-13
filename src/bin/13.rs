use itertools::{Itertools, partition};

/// Returns true if left and right are correctly ordered.
fn are_correct(left: &[String], right: &[String]) -> bool {

    if left.is_empty() {
        return true;
    }

    if !left.is_empty() && right.is_empty() {
        return false;
    }

    match left[0].as_str() {
        "[" => {
            match right[0].as_str() {
                "[" => are_correct(&left[1..left.len()], &right[1..right.len()]),
                "]" => false,
                s2 => {
                    // append matching closing bracket
                    let mut new_right = vec![ s2.to_string(), "]".to_string() ];
                    new_right.append(&mut right.iter().skip(1).map(String::from).collect_vec());
                    are_correct(&left[1..left.len()], &new_right)
                }
            }
        },
        "]" => {
            // All right, shorten the list_2 (if needed)
            if right[0] == "]" {
                are_correct(&left[1..left.len()], &right[1..right.len()])
            } else {
                true
            }

        },
        s1 => {
            match right[0].as_str() {
                "[" => {
                    // append matching closing bracket
                    let mut new_left = vec![ s1.to_string(), "]".to_string()];

                    new_left.append(&mut left.iter().skip(1).map(String::from).collect_vec());
                    are_correct(&new_left, &right[1..right.len()])
                },
                "]" => {
                    // right ended
                    false
                },
                s2 if s1.parse::<i32>().unwrap() == s2.parse::<i32>().unwrap() => are_correct(&left[1..left.len()], &right[1..right.len()]),
                s2 if s1.parse::<i32>().unwrap() < s2.parse::<i32>().unwrap() => {
                    true
                },
                _ => {
                    false
                },
            }
        },
    }
}

/// Returns the packet as a vector of string vector tuples
fn get_lists(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let mut result:Vec<(Vec<String>, Vec<String>)> = vec![];

    // filter empty lines
    let relevant_lines = input.lines().filter(|line| {
        !line.is_empty()
    }).collect_vec();

    for i in (0..relevant_lines.len()).step_by(2) {
        result.push((relevant_lines[i].replace('[', "[,").replace(']', ",]").replace(",,", ",").split(',').map(|s| s.to_string()).collect_vec(), relevant_lines[i+1].replace('[', "[,").replace(']', ",]").replace(",,", ",").split(',').map(|s| s.to_string()).collect_vec()));
    }

    result
}

/// Returns the two delimiters for part 2
fn get_delimiters() -> (Vec<String>, Vec<String>) {
    (vec!["[".to_string(), "[".to_string(), "2".to_string(), "]".to_string(), "]".to_string()], vec!["[".to_string(), "[".to_string(), "6".to_string(), "]".to_string(), "]".to_string()])
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(get_lists(input).into_iter().enumerate().filter_map(|(idx, (list_1, list_2))| if are_correct(&list_1, &list_2) {Some(idx as u32 + 1)} else {None}).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first, second) = get_delimiters();
    let mut all: Vec<Vec<String>> = vec![];
    for (left, right) in get_lists(input) {
        all.push(left);
        all.push(right);
    }
    Some((partition(all.iter_mut(), |string| are_correct(string, &first)) as u32 + 1)*(partition(all.iter_mut(), |string| are_correct(string, &second)) as u32 + 2))
}

fn main() {
    let input = &aoc::read_file("inputs", 13);
    aoc::solve!(13, 1, part_one, input);
    aoc::solve!(13, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_compare() {
        let input = aoc::read_file("examples", 13);
        let parsed = get_lists(&input);
        assert!(are_correct(&parsed[0].0, &parsed[0].1));
        assert!(are_correct(&parsed[1].0, &parsed[1].1));
        assert!(!are_correct(&parsed[2].0, &parsed[2].1));
        assert!(are_correct(&parsed[3].0, &parsed[3].1));
        assert!(!are_correct(&parsed[4].0, &parsed[4].1));
        assert!(are_correct(&parsed[5].0, &parsed[5].1));
        assert!(!are_correct(&parsed[6].0, &parsed[6].1));
        assert!(!are_correct(&parsed[7].0, &parsed[7].1));
    }
    #[test]
    fn test_sample() {
        let parsed = get_lists("[[[[],[2,1,4,2],3],[1,9,[8,2,4,10]],[5]],[[1,2,[10],7],10]]\n[[7,[],[[10,5],5,1],[5]],[3,3],[1,7],[[[1],9],1],[9,[[7,2,6,6,0],[]],[],[]]]");
        assert!(are_correct(&parsed[0].0, &parsed[0].1));
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
