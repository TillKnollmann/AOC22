use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

/// Parses the input and returns a vector of points
fn parse_input(input: &str) -> Vec<Point> {
    // get all points by lines
    let mut points = Vec::new();
    for line in input.lines() {
        // each line is one point separated by commas
        let mut point = Point { x: 0, y: 0, z: 0 };
        for (i, coord) in line.split(',').enumerate() {
            match i {
                0 => point.x = coord.parse().unwrap(),
                1 => point.y = coord.parse().unwrap(),
                2 => point.z = coord.parse().unwrap(),
                _ => panic!("Invalid input"),
            }
        }
        points.push(point);
    }
    points
}

/// Calculates the surface area of the given points
fn get_surface_area(points: Vec<Point>) -> u32 {
    // Idea: Sort the points by x, y, z then y, z, x then z, x, y and always check if according to plane given by the first two axes points are adjacent. Add up the resulting areas along all dimensions

    let mut surface_area = 0;

    // sort the points by x, y, z
    let mut sorted_points = points.clone();
    sorted_points.sort_by(|a, b| {
        if a.x != b.x {
            a.x.cmp(&b.x)
        } else if a.y != b.y {
            a.y.cmp(&b.y)
        } else {
            a.z.cmp(&b.z)
        }
    });

    // loop through all points
    for (i, point) in sorted_points.iter().enumerate() {
        // get the next point
        if i == 0 {
            // first point has no next point
            surface_area += 1;
        } else {
            let last_point = sorted_points
                .get(i - 1)
                .unwrap_or(&Point { x: 0, y: 0, z: 0 });

            // if the next point is on the same x, y plane, they might be adjacent
            if point.x != last_point.x || point.y != last_point.y || point.z != last_point.z + 1 {
                // we are on the same plane, but not adjacent
                surface_area += 2;
            }

            // we are the last point
            if i == sorted_points.len() - 1 {
                surface_area += 1;
            }
        }
    }
    // sort the points by y, z, x
    let mut sorted_points = points.clone();
    sorted_points.sort_by(|a, b| {
        if a.y != b.y {
            a.y.cmp(&b.y)
        } else if a.z != b.z {
            a.z.cmp(&b.z)
        } else {
            a.x.cmp(&b.x)
        }
    });

    // loop through all points
    for (i, point) in sorted_points.iter().enumerate() {
        // get the next point
        if i == 0 {
            // first point has no next point
            surface_area += 1;
        } else {
            let last_point = sorted_points
                .get(i - 1)
                .unwrap_or(&Point { x: 0, y: 0, z: 0 });

            // if the next point is on the same y, z plane, they might be adjacent
            if point.y != last_point.y || point.z != last_point.z || point.x != last_point.x + 1 {
                // we are on the same plane, but not adjacent
                surface_area += 2;
            }

            // we are the last point
            if i == sorted_points.len() - 1 {
                surface_area += 1;
            }
        }
    }

    // sort the points by z, x, y
    let mut sorted_points = points;
    sorted_points.sort_by(|a, b| {
        if a.z != b.z {
            a.z.cmp(&b.z)
        } else if a.x != b.x {
            a.x.cmp(&b.x)
        } else {
            a.y.cmp(&b.y)
        }
    });

    // loop through all points
    for (i, point) in sorted_points.iter().enumerate() {
        // get the next point
        if i == 0 {
            // first point has no next point
            surface_area += 1;
        } else {
            let last_point = sorted_points
                .get(i - 1)
                .unwrap_or(&Point { x: 0, y: 0, z: 0 });

            // if the next point is on the same z, x plane, they might be adjacent
            if point.z != last_point.z || point.x != last_point.x || point.y != last_point.y + 1 {
                // we are on the same plane, but not adjacent
                surface_area += 2;
            }

            // we are the last point
            if i == sorted_points.len() - 1 {
                surface_area += 1;
            }
        }
    }

    surface_area
}

/// Returns the minimum and maximum x, y, z values of the given points
fn get_range(points: &[Point]) -> (i32, i32, i32, i32, i32, i32) {
    // get min and may x, y, z values
    let min_x = points.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
    let max_x = points.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;

    let min_y = points.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
    let max_y = points.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

    let min_z = points.iter().min_by(|a, b| a.z.cmp(&b.z)).unwrap().z;
    let max_z = points.iter().max_by(|a, b| a.z.cmp(&b.z)).unwrap().z;

    (min_x, max_x, min_y, max_y, min_z, max_z)
}

pub fn part_one(input: &str) -> Option<u32> {
    // parse input and get surface area
    let points = parse_input(input);

    Some(get_surface_area(points))
}

pub fn part_two(input: &str) -> Option<u32> {
    // Idea: First get all points. Then flood all points from the outside with a queue in an area as small as possible. Afterward, get all points which were not flooded, add them to the point set and calculate the surface area.

    let mut points = parse_input(input);

    // print range
    let (x_min, x_max, y_min, y_max, z_min, z_max) = get_range(&points);

    // create a 3d vector of points with the same size as the range
    // In the grid each point with 0 is not occupied, each point with 1 is occupied by a rock and each point with 2 is occupied by steam.
    let mut grid: Vec<Vec<Vec<u8>>> =
        vec![
            vec![vec![0; (z_max - z_min + 1) as usize]; (y_max - y_min + 1) as usize];
            (x_max - x_min + 1) as usize
        ];

    // set for each point the grid to 1
    for point in points.iter() {
        grid[(point.x - x_min) as usize][(point.y - y_min) as usize][(point.z - z_min) as usize] =
            1;
    }

    let mut queue = VecDeque::new();

    // set all points at the border which are not 1 to 2
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            for z in 0..grid[x][y].len() {
                if (x == 0
                    || x == grid.len() - 1
                    || y == 0
                    || y == grid[x].len() - 1
                    || z == 0
                    || z == grid[x][y].len() - 1)
                    && grid[x][y][z] != 1
                {
                    grid[x][y][z] = 2;
                    queue.push_back((x, y, z));
                }
            }
        }
    }

    // for each point in the queue, set all adjacent points to 2
    while let Some((x, y, z)) = queue.pop_front() {
        // check all adjacent points
        for (x, y, z) in &[
            (x as i32 - 1, y as i32, z as i32),
            (x as i32 + 1, y as i32, z as i32),
            (x as i32, y as i32 - 1, z as i32),
            (x as i32, y as i32 + 1, z as i32),
            (x as i32, y as i32, z as i32 - 1),
            (x as i32, y as i32, z as i32 + 1),
        ] {
            if *x >= 0
                && *x < grid.len() as i32
                && *y >= 0
                && *y < grid[*x as usize].len() as i32
                && *z >= 0
                && *z < grid[*x as usize][*y as usize].len() as i32
                && grid[*x as usize][*y as usize][*z as usize] == 0
            {
                grid[*x as usize][*y as usize][*z as usize] = 2;
                queue.push_back((*x as usize, *y as usize, *z as usize));
            }
        }
    }

    // get all points that are 0 and add them to points
    (0..grid.len()).for_each(|x| {
        for y in 0..grid[x].len() {
            for z in 0..grid[x][y].len() {
                if grid[x][y][z] == 0 {
                    points.push(Point {
                        x: x as i32 + x_min,
                        y: y as i32 + y_min,
                        z: z as i32 + z_min,
                    });
                }
            }
        }
    });

    // calculate surface area
    Some(get_surface_area(points))
}

fn main() {
    let input = &aoc::read_file("inputs", 18);
    aoc::solve!(18, 1, part_one, input);
    aoc::solve!(18, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
