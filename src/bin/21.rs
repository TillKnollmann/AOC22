use std::collections::HashMap;

/// Represents a monkey
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Monkey {
    name: String,
    operation: String,
    val_1: String,
    val_2: String,
}

/// Parses the input into a map of monkeys
fn parse_input(input: &str) -> HashMap<String, Monkey> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap().replace(':', "");
        let mut operation= "*";
        let val_1 = parts.next().unwrap();
        let mut val_2 = "1";
        if let Some(val) = parts.next() {
            operation = val;
            val_2 = parts.next().unwrap();
        }
        map.insert(
            name.clone(),
            Monkey {
                name: name.to_string(),
                operation: operation.to_string(),
                val_1: val_1.to_string(),
                val_2: val_2.to_string(),
            },
        );
    }
    map
}

/// Calculates the value of the given monkey recursively
fn calculate(monkeys: &HashMap<String, Monkey>, monkey: &Monkey) -> isize {
    let val_1 = match monkeys.get(&monkey.val_1) {
        Some(monkey) => calculate(monkeys, monkey),
        None => monkey.val_1.parse().unwrap(),
    };
    let val_2 = match monkeys.get(&monkey.val_2) {
        Some(monkey) => calculate(monkeys, monkey),
        None => monkey.val_2.parse().unwrap(),
    };
    evaluate_calculation(val_1, val_2, monkey.operation.as_str())
}

/// Evaluates the given calculation
fn evaluate_calculation(val_1: isize, val_2:isize, operation: &str) -> isize {
    match operation {
        "+" => val_1 + val_2,
        "*" => val_1 * val_2,
        "-" => val_1 - val_2,
        "/" => val_1 / val_2,
        _ => panic!("Unknown operation {}", operation),
    }
}

/// Gets the human readable equation dependent on "x" which is the monkey we are looking for
fn get_human_equation(monkeys: &HashMap<String, Monkey>, monkey: &Monkey) -> String {
    if monkey.name == "humn" {
        return "x".to_string();
    }
    let val_1 = match monkeys.get(&monkey.val_1) {
        Some(monkey) => get_human_equation(monkeys, monkey),
        None => monkey.val_1.to_string(),
    };
    let val_2 = match monkeys.get(&monkey.val_2) {
        Some(monkey) => get_human_equation(monkeys, monkey),
        None => monkey.val_2.to_string(),
    };
    if !val_1.contains('x') && !val_2.contains('x') {
        return evaluate_calculation(val_1.parse().unwrap(), val_2.parse().unwrap(), monkey.operation.as_str()).to_string();
    }
    format!("({} {} {})", val_1, monkey.operation, val_2)
}

/// Returns the equation for the given monkeys where "x" is the human monkey
fn get_equation(monkeys: &HashMap<String, Monkey>) -> String {
    let root = monkeys.get("root").unwrap();
    let left_child = monkeys.get(&root.val_1).unwrap();
    let right_child = monkeys.get(&root.val_2).unwrap();
    let left_val = get_human_equation(monkeys, left_child);
    let right_val = get_human_equation(monkeys, right_child);

    let equation = if left_val.contains("x") {left_val.clone()} else {right_val};

    let value = if left_val.contains("x") {calculate(monkeys, right_child)} else {calculate(monkeys, left_child)};

    format!("{} = {}", equation, value)
}

/// Solves the equation for "x"
fn solve_equation(equation: &str) -> isize {
    let mut parts = equation.split('=');
    let left = parts.next().unwrap().trim();
    let right = parts.next().unwrap().trim().parse().unwrap();
    solve_for_x(left, right)
}

/// Recursively solves the equation for "x". Left is an expression dependent on "x" and right is the value of the expression
fn solve_for_x(left: &str, right: isize) -> isize {
    let mut lefthand = left.trim();
    // remove brackets
    if lefthand.starts_with('(') && lefthand.ends_with(')') {
        lefthand = &lefthand[1..lefthand.len()-1];
    }

    if lefthand.trim() == "x" {
        return right;
    }

    let mut is_left_expression = false;

    // assume that the expression is right of the operation
    let mut parts = lefthand.split_whitespace();
    let mut val_1 = parts.next().unwrap();
    let mut operation = parts.next().unwrap();
    let mut val_2 = &lefthand[val_1.len()+operation.len()+1..lefthand.len()];

    if val_1.contains([')', '(', 'x']) {
        // the expression is left of the operation
        is_left_expression = true;
        let mut parts = lefthand.split_whitespace().rev();
        val_1 = parts.next().unwrap();
        operation = parts.next().unwrap();
        val_2 = &lefthand[0..lefthand.len()-val_1.len()-operation.len()-1];
    }

    // split expression and number
    let mut number: isize = 0;
    let mut expression = "";
    if let Ok(val) = val_1.parse() {
        number = val;
        expression = val_2;
    } else if let Ok(val) = val_2.parse() {
        number = val;
        expression = val_1;
    }

    // calculate new right
    let new_right = match operation {
        "+" => right - number,
        "-" => if is_left_expression {right + number} else {-right + number},
        "*" => right / number,
        "/" => if is_left_expression {right * number} else {(1/right) * number},
        _ => panic!("Unknown operation {}", operation),
    };

    // proceed recursively
    solve_for_x(expression, new_right)

}

/// Checks if the equation given by the monkeys is correct
fn check_equation(monkeys: &HashMap<String, Monkey>, root: &Monkey) -> bool {
    let left_child = monkeys.get(&root.val_1).unwrap();
    let right_child = monkeys.get(&root.val_2).unwrap();
    let left_val = calculate(monkeys, left_child);
    let right_val = calculate(monkeys, right_child);
    left_val == right_val
}

pub fn part_one(input: &str) -> Option<u32> {
    // parse input
    let monkeys = parse_input(input);
    // print map
    let result = calculate(&monkeys, monkeys.get("root").unwrap());
    // print result
    if TryInto::<u32>::try_into(result).is_err() {
        println!("Result is too big: {}", result);
        return None;
    }
    Some(result.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    // parse input
    let monkeys = parse_input(input);
    // get equation
    let human_equation = get_equation(&monkeys);

    // solve equation
    let result = solve_equation(human_equation.as_str());

    // check the result
    let mut monkeys = parse_input(input);
    monkeys.insert("humn".to_string(), Monkey { name: "humn".to_string(), operation:"*".to_string(),  val_1: result.to_string(), val_2: "1".to_string()});
    let is_correct = check_equation(&monkeys, monkeys.get("root").unwrap());
    if ! is_correct {
        println!("Solution is not correct: {}", result);
        return None;
    }

    // print result
    if TryInto::<u32>::try_into(result).is_err() {
        println!("Result is too big: {}", result);
        return None;
    }

    Some(result.try_into().unwrap())
}

fn main() {
    let input = &aoc::read_file("inputs", 21);
    aoc::solve!(21, 1, part_one, input);
    aoc::solve!(21, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
