/// Parses the input to a vector of numbers
fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|line| {
            if !line.is_empty() {
                Some(line.parse().unwrap())
            } else {
                None
            }
        })
        .collect()
}

/// Mixes the numbers in the given vector according to the given key and times
fn mix(numbers: &mut Vec<i32>, key: usize, times: usize) {
    // shrink key by numbers.len()-1
    let reduced_key = key % (numbers.len() - 1);
    // multiply each number by key
    (0..numbers.len()).for_each(|i| {
        numbers[i] *= reduced_key as i32;
    });
    // get a vector filled with all indices of input
    let mut orig_to_index: Vec<usize> = (0..numbers.len()).collect();
    let mut index_to_orig: Vec<usize> = (0..numbers.len()).collect();

    for _ in 0..times {
        for i in 0..numbers.len() {
            let start_index = orig_to_index[i];

            let mut steps = numbers[start_index].abs() % (numbers.len() as i32 - 1);

            // go left if the final index is less than the start index
            let left = (numbers[start_index] < 0 && start_index as i32 - steps > 0)
                || (numbers[start_index] >= 0
                    && start_index as i32 + steps > numbers.len() as i32 - 1);

            // we would exceed the left bound, so we need to go right
            if numbers[start_index] < 0 && !left {
                steps = (numbers.len() as i32 - steps - 1) % (numbers.len() as i32 - 1);
            }

            // we would exceed the right bound, so we need to go left
            if numbers[start_index] >= 0 && left {
                steps = (numbers.len() as i32 - steps - 1) % (numbers.len() as i32 - 1);
            }

            if left {
                for count in 1..=steps {
                    swap_right(
                        numbers,
                        &mut orig_to_index,
                        &mut index_to_orig,
                        ((start_index as i32 + numbers.len() as i32 - 1 - count)
                            % (numbers.len() as i32 - 1)) as usize,
                    );
                }
            } else {
                for count in 0..steps {
                    swap_right(
                        numbers,
                        &mut orig_to_index,
                        &mut index_to_orig,
                        ((start_index as i32 + count) % (numbers.len() as i32 - 1)) as usize,
                    );
                }
            }
        }
    }

    (0..numbers.len()).for_each(|i| {
        numbers[i] /= reduced_key as i32;
    });
}

/// Swaps the number at the given index with the number at the index to the right (wrapped around)
fn swap_right(
    numbers: &mut Vec<i32>,
    orig_to_index: &mut [usize],
    index_to_orig: &mut [usize],
    swap_index: usize,
) {
    let number_idx = swap_index;
    let number = numbers[number_idx];
    let number_idx_to_orig = index_to_orig[number_idx];

    let number_to_swap_idx = (number_idx + 1) % numbers.len();
    let number_to_swap = numbers[number_to_swap_idx];

    // move value of number_to_swap to number_idx
    numbers[number_idx] = number_to_swap;
    orig_to_index[index_to_orig[number_to_swap_idx]] = number_idx;
    index_to_orig[number_idx] = index_to_orig[number_to_swap_idx];

    // move value of number to number_to_swap
    numbers[number_to_swap_idx] = number;
    orig_to_index[number_idx_to_orig] = number_to_swap_idx;
    index_to_orig[number_to_swap_idx] = number_idx_to_orig;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers = parse_input(input);

    mix(&mut numbers, 1, 1);

    let value_0_index = numbers.iter().position(|&x| x == 0).unwrap();

    let thousand = numbers[(value_0_index + 1000) % numbers.len()];
    let two_thousand = numbers[(value_0_index + 2000) % numbers.len()];
    let three_thousand = numbers[(value_0_index + 3000) % numbers.len()];

    Some(
        (thousand + two_thousand + three_thousand)
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut numbers = parse_input(input);

    let key = 811589153;

    mix(&mut numbers, key, 10);

    let value_0_index = numbers.iter().position(|&x| x == 0).unwrap();

    let thousand = numbers[(value_0_index + 1000) % numbers.len()];
    let two_thousand = numbers[(value_0_index + 2000) % numbers.len()];
    let three_thousand = numbers[(value_0_index + 3000) % numbers.len()];

    let result: usize = ((thousand + two_thousand + three_thousand) as usize) * key;

    if TryInto::<u32>::try_into(result).is_err() {
        println!("Result is too large! Submit manually: {}", result);
        return None;
    }

    Some(result.try_into().unwrap())
}

fn main() {
    let input = &aoc::read_file("inputs", 20);
    aoc::solve!(20, 1, part_one, input);
    aoc::solve!(20, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
