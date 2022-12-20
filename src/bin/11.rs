use itertools::Itertools;

/// Represents a monkey. A monkey has a set of items, an operation it does for inspecting an item, a modification for the stress level, an action telling where to throw the item, and a count of inspections it made.
struct Monkey {
    items: Vec<usize>,
    pub operation: Box<dyn Fn(usize) -> usize>,
    pub modification: Box<dyn Fn(usize) -> usize>,
    pub throw: Box<dyn Fn(usize) -> usize>,
    inspection_count: u32,
}

/// Returns the puzzle monkeys where func is a function for the relief one experiences
fn get_puzzle_monkeys(func: fn(usize) -> usize) -> Vec<Monkey> {
    let relief = Box::new(func);
    vec![
        Monkey {
            items: vec![50, 70, 54, 83, 52, 78],
            inspection_count: 0,
            operation: Box::new(
                |item: usize| {
                    item * 3
                }
            ),
            modification: relief.clone(),
            throw: Box::new(
                |item: usize| {
                    if item % 11 == 0 {
                        2
                    } else {
                        7
                    }
                }
            ),
        },
        Monkey {
            items: vec![71, 52, 58, 60, 71],
            inspection_count: 0,
            operation: Box::new(
                |item: usize| {
                    item * item
                }
            ),
            modification: relief.clone(),
            throw: Box::new(
                |item: usize| {
                    if item % 7 == 0 {
                        0
                    } else {
                        2
                    }
                }
            ),
        },
        Monkey {
            items: vec![66, 56, 56, 94, 60, 86, 73],
            inspection_count: 0,
            operation: Box::new(
                |item: usize| {
                    item + 1
                }
            ),
            modification: relief.clone(),
            throw: Box::new(
                |item: usize| {
                    if item % 3 == 0 {
                        7
                    } else {
                        5
                    }
                }
            ),
        },
        Monkey {
            items: vec![83, 99],
            inspection_count: 0,
            operation: Box::new(
                |item: usize| {
                    item + 8
                }
            ),
            modification: relief.clone(),
            throw: Box::new(
                |item: usize| {
                    if item % 5 == 0 {
                        6
                    } else {
                        4
                    }
                }
            ),
        },
        Monkey {
            items: vec![98, 98, 79],
            inspection_count: 0,
            operation: Box::new(
                |item: usize| {
                    item + 3
                }
            ),
            modification: relief.clone(),
            throw: Box::new(
                |item: usize| {
                    usize::from(item % 17 == 0)
                }
            ),
        },
        Monkey {
            items: vec![76],
            inspection_count: 0,
            operation: Box::new(
                |item: usize| {
                    item + 4
                }
            ),
            modification: relief.clone(),
            throw: Box::new(
                |item: usize| {
                    if item % 13 == 0 {
                        6
                    } else {
                        3
                    }
                }
            ),
        },
        Monkey {
            items: vec![52, 51, 84, 54],
            inspection_count: 0,
            operation: Box::new(
                |item: usize| {
                    item * 17
                }
            ),
            modification: relief.clone(),
            throw: Box::new(
                |item: usize| {
                    if item % 19 == 0 {
                        4
                    } else {
                        1
                    }
                }
            ),
        },
        Monkey {
            items: vec![82, 86, 91, 79, 94, 92, 59, 94],
            inspection_count: 0,
            operation: Box::new(
                |item: usize| {
                    item + 7
                }
            ),
            modification: relief,
            throw: Box::new(
                |item: usize| {
                    if item % 2 == 0 {
                        5
                    } else {
                        3
                    }
                }
            ),
        },
    ]
}


/// Returns the example monkeys where func is a function for the relief one experiences
fn get_example_monkeys(func: fn(usize) -> usize) -> Vec<Monkey> {
    let relief = Box::new(func);
    vec![
        Monkey {items: vec![79, 98],
            operation: Box::new(|item: usize| {
                item * 19
            }),
            modification: relief.clone(),
            throw: Box::new(|item: usize| {
                if item % 23 == 0 {
                    2
                } else {
                    3
                }
            }),
            inspection_count: 0,
        },
        Monkey {items: vec![54, 65, 75, 74],
            operation: Box::new(|item: usize| {
                item + 6
            }),
            modification: relief.clone(),
             throw: Box::new(|item: usize| {
                if item % 19 == 0 {
                    2
                } else {
                    0
                }
            }),
            inspection_count: 0,
        },
        Monkey {items: vec![79, 60, 97],
            operation: Box::new(|item: usize| {
                item * item
            }),
            modification: relief.clone(),
             throw: Box::new(|item: usize| {
                if item % 13 == 0 {
                    1
                } else {
                    3
                }
            }),
            inspection_count: 0,
        },
        Monkey {items: vec![74],
            operation: Box::new(|item: usize| {
                item + 3
            }),
            modification: relief,
             throw: Box::new(|item: usize| {
                usize::from(item % 17 != 0)
            }),
            inspection_count: 0,
        },
    ]
}

fn simulate_rounds(number: u32, monkeys: &mut Vec<Monkey>, reduce: fn(usize) -> usize) {
    for _ in 0..number {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let value: usize;
                let destin: usize;
                {
                    let monkey = &mut monkeys[i];
                    value = (monkey.modification)((monkey.operation)(monkey.items.remove(0)));
                    destin = (monkey.throw)(value);
                }
                monkeys[destin].items.push(reduce(value));
                monkeys[i].inspection_count+=1;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys = match input {
        "ex" => get_example_monkeys(|item: usize| -> usize {item/3}),
        "in" => get_puzzle_monkeys(|item: usize| -> usize {item/3}),
        _ => vec![]
    };
    simulate_rounds(20, &mut monkeys, |item| -> usize {item});
    Some(monkeys.iter().map(|monkey| monkey.inspection_count).sorted().rev().take(2).product())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut monkeys = match input {
        "ex" => get_example_monkeys(|item: usize| -> usize {item}),
        "in" => get_puzzle_monkeys(|item: usize| -> usize {item}),
        _ => vec![]
    };
    match input {
        "ex" => simulate_rounds(10000, &mut monkeys, |item| -> usize {item % 96577}),
        "in" => simulate_rounds(10000, &mut monkeys, |item| -> usize {item % 9699690}),
        _ => {},
    };
    match input {
        "ex" => Some(monkeys.iter().map(|monkey| monkey.inspection_count).sorted().rev().take(2).product()),
        "in" => { println!("The two values are {:?}. Go multiply yourself!", monkeys.iter().map(|monkey| monkey.inspection_count).sorted().rev().take(2).collect_vec()); None },
        _ => None
    }
}

fn main() {
    let _input = &aoc::read_file("inputs", 11);
    aoc::solve!(11, 1, part_one, "in");
    aoc::solve!(11, 2, part_two, "in");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let _input = aoc::read_file("examples", 11);
        assert_eq!(part_one("ex"), Some(10605));
        assert_eq!(part_one("in"), Some(102399));
    }

    #[test]
    fn test_part_two() {
        let _input = aoc::read_file("examples", 11);
        assert_eq!(part_two("ex"), Some(2713310158));
        assert_eq!(part_two("in"), None);
    }
}
