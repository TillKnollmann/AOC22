use std::cmp::min;

use itertools::Itertools;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

/// Returns the Manhattan distance between two points
fn manhattan_distance(p1: Point, p2: Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

/// Returns a vector of a sensor and a beacon
fn parse_input(input: &str) -> Vec<Point> {
    // point 1 is 3 and 4 word
    // point 2 is 9 and 10 word
    input
        .lines().filter(|line| !line.is_empty())
        .flat_map(|line| {
            let mut words = line.split_whitespace();
            let p1 = Point {
                x: words
                    .nth(2)
                    .unwrap()
                    .replace("x=", "")
                    .replace(',', "")
                    .parse()
                    .unwrap(),
                y: words.next()
                    .unwrap()
                    .replace("y=", "")
                    .replace(':', "")
                    .parse()
                    .unwrap(),
            };
            let p2 = Point {
                x: words
                    .nth(4)
                    .unwrap()
                    .replace("x=", "")
                    .replace(',', "")
                    .parse()
                    .unwrap(),
                y: words.next()
                    .unwrap()
                    .replace("y=", "")
                    .replace(':', "")
                    .parse()
                    .unwrap(),
            };
            vec![p1, p2]
        })
        .collect()
}

/// Returns the min and max x and y
fn get_range(sensors_beacons: &[Vec<Point>]) -> (i32, i32, i32, i32) {
    // get all points
    let points: Vec<Point> = sensors_beacons
        .iter()
        .flat_map(|sb| {
            vec![sb[0], sb[1]]
        })
        .collect();
    // get min and max x and y
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    (min_x, max_x, min_y, max_y)
}

/// Returns true if the point cannot contain a beacon
fn cannot_contain_beacon(p: &Point, sensors_beacons: &[Vec<Point>]) -> bool {
    sensors_beacons.iter().any(|sb| {
        let d1 = manhattan_distance(*p, sb[0]);
        let d2 = manhattan_distance(sb[0], sb[1]);
        d1 <= d2 && sb[1] != *p
    })
}

/// Returns true if the point is an unknown beacon
fn is_unknown_beacon(p: &Point, sensors_beacons: &[Vec<Point>]) -> bool {
    !sensors_beacons.iter().any(|sb| {
        let d1 = manhattan_distance(*p, sb[0]);
        let d2 = manhattan_distance(sb[0], sb[1]);
        d1 <= d2 || sb[1] == *p
    })
}

/// Returns a vector of all points that could potentially be the one unknown beacon
fn get_surrounding_points(p: &Point, dist: i32) -> Vec<Point> {
    let mut points = Vec::new();
    // for i from 0 to dist
    for i in 0..dist {
        // get points with distance dist
        let p1 = Point {
            // above p to right of p
            x: p.x + i,
            y: p.y - dist + i,
        };
        let p2 = Point {
            // right of p to below p
            x: p.x + dist - i,
            y: p.y + i,
        };
        let p3 = Point {
            // below p to left of p
            x: p.x - i,
            y: p.y + dist - i,
        };
        let p4 = Point {
            // left of p to above p
            x: p.x - dist + i,
            y: p.y - i,
        };
        // add points to vector
        points.push(p1);
        points.push(p2);
        points.push(p3);
        points.push(p4);
    }
    points
}

pub fn part_one(input: &str) -> Option<u32> {
    // iterate over all lines and collect the points
    let sensors_beacons: Vec<Vec<Point>> = input.lines().map(parse_input).collect_vec();

    let (min_x, max_x, _min_y,_max_yy) = get_range(&sensors_beacons);

    let mut y_coord = 2_000_000;
    if input.lines().count() == 14 {
        y_coord = 10;
    }

    // generate all points where x is between min_x and max_x and y is y_coord
    let candidates: Vec<Point> = (min_x..=max_x)
        .map(|x| Point { x, y: y_coord })
        .collect();

    // remove all points where y is not y_coord and that cannot be a beacon
    Some(candidates
        .iter()
        .filter(|p| cannot_contain_beacon(p, &sensors_beacons)).copied().count() as u32)

}

pub fn part_two(input: &str) -> Option<u32> {
    // same as part_one but check every possible point
    let sensors_beacons: Vec<Vec<Point>> = input.lines().map(parse_input).collect_vec();

    // print sensors and beacons
    // println!("sensors_beacons: {:?}", sensors_beacons);

    let (_, mut max_x, _, mut max_y) = get_range(&sensors_beacons);


    // redefine max_x and max_y
    max_x = min(max_x, 4_000_000);
    max_y = min(max_y, 4_000_000);

    // adapt for example input
    if input.lines().count() == 14 {
        max_x = 20;
        max_y = 20;
    }

    // get surrounding points of all sensors in distances manhattan_distance(sensor, beacon) + 1 and check them
    let result_points = sensors_beacons.iter().flat_map(|sb| {
        let dist = manhattan_distance(sb[0], sb[1]) + 1;
        let mut points = get_surrounding_points(&sb[0], dist);
        points.retain(|p| {
            p.x >= 0 && p.x <= max_x && p.y >= 0 && p.y <= max_y && is_unknown_beacon(p, &sensors_beacons)
        });
        points
    }).collect_vec();

    // result is first point
    let result = result_points.first().unwrap();

    let calculated = result.x as usize*4_000_000_usize + result.y as usize;

    match std::convert::TryInto::<u32>::try_into(calculated) {
        Ok(v) => Some(v),
        Err(_) => {
            println!("Value is to big for u32: {}", calculated);
            None
        }
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 15);
    aoc::solve!(15, 1, part_one, input);
    aoc::solve!(15, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
