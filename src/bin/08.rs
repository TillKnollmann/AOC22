use itertools::Itertools;
use std::cmp::max;

/// Returns all trees as a matrix
fn get_trees(input: &str) -> Vec<Vec<u32>> {
    let mut trees:Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        if !line.is_empty() {
            trees.push(line.chars().map(|c| c.to_string().parse::<u32>().unwrap()).collect_vec());
        }
    }

    trees
}

/// Returns matrix containing all max_heights to the left
fn get_max_left(trees: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut left: Vec<Vec<u32>> = vec![];

    for row in trees {
        let mut left_row: Vec<u32> = vec![0];
        for i in 1..row.len() {
            left_row.push(max(*left_row.last().unwrap(), row[i-1]));
        }
        left.push(left_row);
    }

    left
}

/// Returns matrix containing all max_heights to the right
fn get_max_right(trees: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut right: Vec<Vec<u32>> = vec![];

    for row in trees {
        let mut right_row: Vec<u32> = vec![0];
        for i in (0..row.len()-1).rev() {
            right_row.push(max(*right_row.last().unwrap(), row[i+1]));
        };

        right_row.reverse();
        right.push(right_row);
    }

    right
}


/// Returns matrix containing all max_heights to the top (careful: dimensions are switched)
fn get_max_top(trees: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut top: Vec<Vec<u32>> = vec![];

    for i in 0..trees[0].len() {
        let mut top_col: Vec<u32> = vec![0];

        for j in 1..trees.len() {
            top_col.push(max(*top_col.last().unwrap(), trees[j-1][i]));
        }

        top.push(top_col);
    }

    top
}

/// Returns matrix containing all max_heights to the bottom (careful: dimensions are switched)
fn get_max_bottom(trees: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut bot: Vec<Vec<u32>> = vec![];

    for i in 0..trees[0].len() {
        let mut bot_col: Vec<u32> = vec![0];

        for j in (0..trees.len()-1).rev() {
            bot_col.push(max(*bot_col.last().unwrap(), trees[j+1][i]));
        }

        bot_col.reverse();
        bot.push(bot_col);
    }

    bot
}

/// Returns true iff the tree at (i,j) is visible
fn is_tree_visible(trees: &[Vec<u32>], (i, j): (usize, usize), left: &[Vec<u32>], right: &[Vec<u32>], top: &[Vec<u32>], bot: &[Vec<u32>]) -> bool {
    (left[i][j] < trees[i][j]) || (right[i][j] < trees[i][j]) || (top[j][i] < trees[i][j]) || (bot[j][i] < trees[i][j]) || (i==0) || (j==0) || (i == trees.len()-1) || (j == trees[i].len()-1)
}

/// Returns the scenic score for a tree at (i,j)
fn get_scenic_score(trees: &[Vec<u32>], (i, j): (usize, usize)) -> u32 {
    let mut sc_left = 1;
    let mut sc_right = 1;
    let mut sc_top = 1;
    let mut sc_bot = 1;

    if i == 0 {
        sc_top = 0;
    }

    if i == trees.len()-1 {
        sc_bot = 0;
    }

    if j == 0 {
        sc_left = 0;
    }

    if j == trees[i].len()-1 {
        sc_right = 0;
    }


    for left in (1..j).rev() {
        if trees[i][left] < trees[i][j] {
            sc_left += 1;
        } else {
            break;
        }
    }

    for right in j+1..trees[i].len()-1 {
        if trees[i][right] < trees[i][j] {
            sc_right += 1;
        } else {
            break;
        }
    }

    for top in (1..i).rev() {
        if trees[top][j] < trees[i][j] {
            sc_top += 1;
        } else {
            break;
        }
    }

    for bot in i+1..trees.len()-1 {
        if trees[bot][j] < trees[i][j] {
            sc_bot += 1;
        } else {
            break;
        }
    }
    sc_left * sc_right * sc_top * sc_bot
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees = get_trees(input);

    let left = get_max_left(&trees);
    let right = get_max_right(&trees);
    let top = get_max_top(&trees);
    let bot = get_max_bottom(&trees);

    let mut score = 0;

    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            if is_tree_visible(&trees, (i,j), &left, &right, &top, &bot) {
                score+=1;
            }
        }
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let trees = get_trees(input);

    let mut max_val = 0;

    for i in 0..trees.len() {
        for j in 0..trees[0].len() {
            max_val = max(max_val, get_scenic_score(&trees, (i,j)));
        }
    }

    Some(max_val)
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(8, 1, part_one, input);
    aoc::solve!(8, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_scenic_score() {
        let input = aoc::read_file("examples", 8);
        let trees = get_trees(&input);
        assert_eq!(get_scenic_score(&trees, (1,2)), 4);
        assert_eq!(get_scenic_score(&trees, (3,2)), 8);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
