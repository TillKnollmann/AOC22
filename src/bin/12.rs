use itertools::Itertools;
use petgraph::{
    algo,
    prelude::{DiGraphMap, GraphMap},
};

/// Builds a directed graph out of the input nodes
fn build_graph(
    input: &Vec<Vec<char>>,
) -> GraphMap<(usize, usize), u32, petgraph::Directed> {
    let mut graph = DiGraphMap::<(usize, usize), u32>::new();

    // Add nodes
    (0..input.len()).for_each(|i| {
        (0..input[i].len()).for_each(|j| {
            graph.add_node((i, j));
        });
    });

    // Add edges
    (0..input.len()).for_each(|i| {
        (0..input[i].len()).for_each(|j| {
            // left
            if j > 0 && get_ele(input[i][j - 1]) <= get_ele(input[i][j]) + 1 {
                graph.add_edge((i, j), (i, j - 1), 1);
            }
            // right
            if j < input[i].len() - 1 && get_ele(input[i][j + 1]) <= get_ele(input[i][j]) + 1 {
                graph.add_edge((i, j), (i, j + 1), 1);
            }
            // top
            if i > 0 && get_ele(input[i-1][j]) <= get_ele(input[i][j]) + 1 {
                graph.add_edge((i,j), (i-1,j), 1);
            }
            // down
            if i < input.len() - 1 && get_ele(input[i + 1][j]) <= get_ele(input[i][j]) + 1 {
                graph.add_edge((i, j), (i + 1, j), 1);
            }
        });
    });

    graph
}

/// Returns the elevation for a given char c
fn get_ele(c: char) -> u32 {
    match c {
        'S' => 'a' as u32,
        'E' => 'z' as u32,
        _ => c as u32,
    }
}

/// Returns a vector of nodes that match the given pattern
fn get_nodes(nodes: &Vec<Vec<char>>, pat: &[char]) -> Vec<(usize, usize)> {
    let mut result = vec![];
    (0..nodes.len()).for_each(|i| {
        (0..nodes[i].len()).for_each(|j| {
            if pat.contains(&nodes[i][j])  {
                result.push((i,j));
            }
        });
    });
   result
}

/// Transforms the input to a 2d vector of chars
fn in_to_vec(input: &str) -> Vec<Vec<char>> {
    let mut res = vec![];

    for line in input.lines() {
        if !line.is_empty() {
            res.push(line.chars().collect_vec());
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let nodes = in_to_vec(input);
    let graph = build_graph(&nodes);
    let start = get_nodes(&nodes, &['S'])[0];
    let dst = get_nodes(&nodes, &['E'])[0];
    let length = algo::dijkstra(&graph, start, Some(dst), |_| 1);

    Some(*length.get(&dst).unwrap() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let nodes = in_to_vec(input);
    let graph = build_graph(&nodes);
    let dst = get_nodes(&nodes, &['E'])[0];

    Some(get_nodes(&nodes, &['S', 'a']).iter().filter_map(|start| -> Option<u32> {
        algo::dijkstra(&graph, *start, Some(dst), |_| 1).get(&dst).copied()
    }).min().unwrap())
}

fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(12, 1, part_one, input);
    aoc::solve!(12, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_ex = aoc::read_file("examples", 12);
        let input_in = aoc::read_file("inputs", 12);
        assert_eq!(part_one(&input_ex), Some(31));
        assert_eq!(part_one(&input_in), Some(456));
    }

    #[test]
    fn test_part_two() {
        let input_ex = aoc::read_file("examples", 12);
        let input_in = aoc::read_file("inputs", 12);
        assert_eq!(part_two(&input_ex), Some(29));
        assert_eq!(part_two(&input_in), Some(454));
    }
}
