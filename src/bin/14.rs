use std::cmp::{min, max};

use itertools::Itertools;

#[derive(Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

/// Prints a cave
fn print_cave(cave:  &Vec<Vec<u8>>) {
    for y in 0..cave[0].len() {
        (0..cave.len()).for_each(|x| {
            if cave[x][y] == 1 {
                print!("#");
            } else if cave[x][y] == 2 {
                print!("o");
            } else {
                print!(".");
            }
        });
        println!();
    }
}

/// Returns the initial cave without sand
fn get_cave(input: &str, floor: bool) -> (Vec<Vec<u8>>, u32) {
    let mut cave = vec![];

    // gather all x and y values
    let mut all_x = vec![];
    let mut all_y = vec![];
    for line in input.lines() {
        let points_str = line.split(" -> ").collect_vec();
        for point in points_str.iter().map(|s| s.split(',').map(|s| s.parse::<u32>().unwrap()).collect_vec()) {
            all_x.push(point[0]);
            all_y.push(point[1]);
        }
    }

    // initialize the cave
    let y_max = *all_y.iter().max().unwrap();

    // find bounds for the floor
    let floor_depth = y_max + 2;
    let offset = 500 - (floor_depth+10);
    let x_max = 500 + (floor_depth+10);

    for _ in 0..(x_max - offset + 1) {
        cave.push(vec![0; (floor_depth + 1) as usize]);
    }

    // fill the cave with paths
    input.lines().for_each(|line| add_path(line, &mut cave, offset));

    // Add the floor
    if floor {
        add_simple_path(Point{x:offset, y:floor_depth}, Point{x:x_max, y:floor_depth}, &mut cave, offset);
    }

    (cave, offset)
}

/// Adds a path given by an input string to the cave
fn add_path(input: &str, cave:  &mut [Vec<u8>], offset: u32) {
    let points_str = input.split(" -> ").collect_vec();
    for i in 1..points_str.len() {
        let first = points_str[i-1].split(',').map(|s| s.parse::<u32>().unwrap()).collect_vec();
        let second = points_str[i].split(',').map(|s| s.parse::<u32>().unwrap()).collect_vec();
        add_simple_path(Point { x:first[0], y:first[1]}, Point { x:second[0], y:second[1]}, cave, offset);
    }
}

/// Adds a path between two points to the cave
fn add_simple_path(start: Point, dest: Point, cave:  &mut [Vec<u8>], offset: u32) {
    if start.x == dest.x {
        for i in min(start.y, dest.y)..=max(start.y, dest.y) {
            cave[(start.x - offset) as usize][i as usize] = 1;
        }
    } else if start.y == dest.y {
        for i in min(start.x, dest.x)..=max(start.x, dest.x) {
            cave[(i-offset) as usize][start.y as usize] = 1;
        }
    }
}

/// Simulates one sand grain falling down. Returns the place where it comes to rest (if any)
fn simulate_sand_grain(cave:  &mut Vec<Vec<u8>>, offset: u32) -> Option<Point> {
    let mut sand = Point {x: 500, y: 0};
    let rest = find_rest_point(&mut sand, cave, offset);
    if let Some(unwr) = rest  {
        cave[(unwr.x - offset) as usize][unwr.y as usize] = 2;
    }
    rest
}

/// Simulates one fall step of a sand grain. Returns the place where it comes to rest (if any)
fn find_rest_point(p: &mut Point, cave:  &Vec<Vec<u8>>, offset: u32) -> Option<Point> {

    if p.y as usize >= cave[0].len() - 1 || (p.x - offset) as usize <= 0 || (p.x - offset) as usize >= cave.len()-1 {
        return None;
    }

    // check if fall down is possible
    if cave[(p.x - offset) as usize][p.y as usize + 1] == 0 {
        p.y+=1;
        return find_rest_point(p, cave, offset);
    }

    // Can we fall left?
    if cave[(p.x - offset) as usize - 1][p.y as usize + 1] == 0 {
        p.x-=1;
        p.y+=1;
        return find_rest_point(p, cave, offset);
    }

    // Can we fall right?
    if cave[(p.x - offset) as usize + 1][p.y as usize + 1] == 0 {
        p.x+=1;
        p.y+=1;
        return find_rest_point(p, cave, offset);
    }

    // found a rest point
    Some(*p)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut cave, offset) = get_cave(input, false);
    let mut counter = 0;
    let mut last_rest = simulate_sand_grain(&mut cave, offset);
    while last_rest.is_some() {
        last_rest = simulate_sand_grain(&mut cave, offset);
        counter += 1;
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut cave, offset) = get_cave(input, true);
    let mut counter = 0;
    let mut last_rest = None;
    let mut stop = false;
    while !stop {
        last_rest = simulate_sand_grain(&mut cave, offset);
        counter += 1;
        match last_rest {
            Some(rest) => if rest.x == 500 && rest.y==0 {stop = true;},
            None => {println!("Sand fell out of cave!"); stop=true; print_cave(&cave)},
        };
    }
    last_rest.map(|_| counter)
}

fn main() {
    let input = &aoc::read_file("inputs", 14);
    aoc::solve!(14, 1, part_one, input);
    aoc::solve!(14, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
