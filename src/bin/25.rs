use itertools::Itertools;

/// Converts a snafu number to a decimal number
fn _snafu_to_decimal(snafu_number: &str) -> usize {
    let mut decimal_number: i64 = 0;
    let mut power = 0;
    for c in snafu_number.chars().rev() {
        match c {
            '2' => decimal_number += 5_i64.pow(power)*2,
            '1' => decimal_number += 5_i64.pow(power)*1,
            '0' => {},
            '-' => decimal_number -= 5_i64.pow(power)*1,
            '=' => decimal_number -= 5_i64.pow(power)*2,
            _ => panic!("Invalid snafu number"),
        }
        power += 1;
    }
    decimal_number as usize
}

/// adds two snafu numbers and returns the result as a snafu number
fn add_snafus(snafu_1: String, snafu_2: String) -> String {
    // add both snafu numbers and return the result without converting to decimal
    let mut result = String::new();
    let mut carry = 0;
    let mut i = 0;

    let mut snafu_1 = snafu_1.chars().rev().collect_vec();
    let mut snafu_2 = snafu_2.chars().rev().collect_vec();

    // make both snafu numbers the same length
    while snafu_1.len() < snafu_2.len() {
        snafu_1.push('0');
    }
    while snafu_2.len() < snafu_1.len() {
        snafu_2.push('0');
    }

    while i < snafu_1.len() {
        let mut sum = carry;
        match snafu_1[i] {
            '2' => sum += 2,
            '1' => sum += 1,
            '0' => {},
            '-' => sum -= 1,
            '=' => sum -= 2,
            _ => panic!("Invalid snafu number"),
        }
        match snafu_2[i] {
            '2' => sum += 2,
            '1' => sum += 1,
            '0' => {},
            '-' => sum -= 1,
            '=' => sum -= 2,
            _ => panic!("Invalid snafu number"),
        }

        match sum {
            5 => {
                result.push('0');
                carry = 1;
            },
            4 => {
                result.push('-');
                carry = 1;
            },
            3 => {
                result.push('=');
                carry = 1;
            },
            2 => {
                result.push('2');
                carry = 0;
            },
            1 => {
                result.push('1');
                carry = 0;
            },
            0 => {
                result.push('0');
                carry = 0;
            },
            -1 => {
                result.push('-');
                carry = 0;
            },
            -2 => {
                result.push('=');
                carry = 0;
            },
            -3 => {
                result.push('2');
                carry = -1;
            },
            -4 => {
                result.push('1');
                carry = -1;
            },
            -5 => {
                result.push('0');
                carry = -1;
            },
            _ => panic!("Invalid sum"),
        }
        i += 1;
    }
    if carry == 1 {
        result.push('1');
    } else if carry == -1 {
        result.push('-');
    }
    result = result.chars().rev().collect();

    result

}

pub fn part_one(input: &str) -> Option<u32> {
    let snafus = input.lines().collect_vec();

    // add all snafus
    let mut sum = String::new();
    for snafu in snafus {
        sum = add_snafus(sum, snafu.to_string());
    }

    println!("{}", sum);
    None
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 25);
    aoc::solve!(25, 1, part_one, input);
    aoc::solve!(25, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 25);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
