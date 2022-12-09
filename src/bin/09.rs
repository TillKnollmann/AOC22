use std::collections::HashSet;

/// Describes a 2d position
struct Point {
    x: i32,
    y: i32,
}

/// Returns true iff the tail is not adjacent to the head
fn move_nec(head: &Point, tail: &Point) -> bool {
    !(head.x.abs_diff(tail.x) <= 1 && head.y.abs_diff(tail.y) <= 1)
}

/// Simulate one step (several movements) of a rope with the given knots
fn sim_rope_step(command: &str, knots: &mut Vec<Point>, pos_count: &mut HashSet<(i32, i32)>) {
    let mut split = command.split_whitespace();

    let dir = split.next().unwrap();
    let count = split.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..count {
        match dir {
            "R" => {
                knots[0].x += 1;
            }
            "U" => {
                knots[0].y += 1;
            }
            "L" => {
                knots[0].x -= 1;
            }
            "D" => {
                knots[0].y -= 1;
            }
            _ => {
                println!("Unknown movement {}", command);
            }
        }
        // move every knot
        for i in 1..knots.len() {
            if move_nec(&knots[i - 1], &knots[i]) {
                if knots[i - 1].y != knots[i].y {
                    // vertical move
                    if knots[i].y > knots[i - 1].y {
                        knots[i].y -= 1;
                    } else {
                        knots[i].y += 1;
                    }
                }
                if knots[i - 1].x != knots[i].x {
                    // horizontal move
                    if knots[i].x > knots[i - 1].x {
                        knots[i].x -= 1;
                    } else {
                        knots[i].x += 1;
                    }
                }
                if i == knots.len()-1 {
                    pos_count.insert((knots[i].x, knots[i].y));
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut knots: Vec<Point> = vec![];
    let mut pos_count: HashSet<(i32, i32)> = HashSet::new();

    pos_count.insert((0, 0));

    for _ in 0..2 {
        knots.push(Point { x: 0, y: 0 });
    }

    for line in input.lines() {
        sim_rope_step(line, &mut knots, &mut pos_count);
    }

    Some(pos_count.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut knots: Vec<Point> = vec![];
    let mut pos_count: HashSet<(i32, i32)> = HashSet::new();

    pos_count.insert((0, 0));

    for _ in 0..10 {
        knots.push(Point { x: 0, y: 0 });
    }

    for line in input.lines() {
        sim_rope_step(line, &mut knots, &mut pos_count);
    }

    Some(pos_count.len() as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(9, 1, part_one, input);
    aoc::solve!(9, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
