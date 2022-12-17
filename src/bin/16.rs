use std::collections::{HashMap, HashSet};
use indicatif::ProgressStyle;
use itertools::Itertools;
use petgraph::prelude::{DiGraphMap, GraphMap};

/// Represents a Valve having an index, a value for the flow and a list of connections to other valves
#[derive(Debug, Hash)]
struct Valve {
    index: u32,
    flow: u32,
    connections: Vec<(String, u32)>,
}

/// Parses the input and returns a HashMap of all valves mapped by their name
fn parse_input(input: &str) -> HashMap<String, Valve> {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    for line in input.lines() {
        if !line.is_empty() {
            let mut parts = line.split(' ');
            // name is word 2
            // flow is word 5 without "rate=" and ";"
            let name = parts.nth(1).unwrap().to_string();
            let flow = parts
                .nth(2)
                .unwrap()
                .replace("rate=", "")
                .replace(';', "")
                .parse()
                .unwrap();
            // connections are all words after word 4 without ","
            let mut connections = Vec::new();
            let parts = parts.skip(4);
            for part in parts {
                connections.push((part.to_string().replace(',', ""), 1));
            }
            valves.insert(
                name,
                Valve {
                    index: valves.len() as u32,
                    flow,
                    connections,
                },
            );
        }
    }
    valves
}

/// Parses the valves to a GraphMap
fn get_graph(valves: &HashMap<String, Valve>) -> GraphMap<&str, u32, petgraph::Directed> {
    // make every valve a node in the graph
    let mut graph = DiGraphMap::<&str, u32>::new();
    valves.iter().for_each(|(name, _)| {
        graph.add_node(name.as_str());
    });
    // make every connection a edge in the graph
    for (name, valve) in valves {
        for connection in &valve.connections {
            graph.add_edge(name.as_str(), connection.0.as_str(), connection.1);
        }
    }

    graph
}

/// Returns a HashSet of all relevant nodes (where the flow is greater than 0 or the node is "AA")
fn get_relevant_nodes(valves: &HashMap<String, Valve>) -> HashSet<&str> {
    let mut relevant_nodes = HashSet::new();
    for (name, valve) in valves {
        if valve.flow > 0 || name == "AA" {
            relevant_nodes.insert(name.as_str());
        }
    }
    relevant_nodes
}

/// Returns true iff all open valves of state 1 are closed in state 2 and vice versa
fn states_are_compatible(state1: &[u32], state2: &[u32]) -> bool {
    !state1.iter().zip(state2).any(|(a, b)| a == b)
}

/// Simulates how valves can be opened in limit many minutes. Returns the maximum number of release possible.
fn simulate_valve_run(input: &str, limit: usize, use_elephant: bool) -> Option<u32> {

    // get parsed input
    let valves = parse_input(input);

    // get graph of valves
    let graph = get_graph(&valves);

    // get relevant nodes
    let relevant_nodes = get_relevant_nodes(&valves);

    // get solution to flyod warshall on graph
    let solution = petgraph::algo::floyd_warshall(&graph, |_| 1).unwrap();

    // state_space is a vector of size 31 x relevant_nodes.len() x 2^relevant_nodes.len()
    let mut state_space: Vec<Vec<HashMap<Vec<u32>,u32>>> = vec![vec![HashMap::new(); relevant_nodes.len()]; limit +1];

    // fill state_space with zero for all subsets of relevant_nodes for all i in 0..=1
    (0..=1).for_each(|i| {
        for (j, _node) in relevant_nodes.iter().enumerate() {
            for subset in (0..=relevant_nodes.len()).flat_map(|count| (0..relevant_nodes.len()).combinations(count)) {
                // create a vector of size relevant_nodes.len() with all values 0
                let mut vec = vec![0; relevant_nodes.len()];
                // set all values in vec to 1 where the index is in subset
                for k in subset {
                    vec[k] = 1;
                }

                state_space[i][j].insert(vec, 0);
            }
        }
    });

    // print for how many relevant nodes we have to calculate
    println!("Calculating for {} relevant nodes", relevant_nodes.len());

    // get Progress Bar
    let pb = indicatif::ProgressBar::new((limit as u64 -1)*relevant_nodes.len() as u64 * 2u64.pow(relevant_nodes.len() as u32));

    pb.set_style(ProgressStyle::with_template("[{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap()
    .progress_chars("##-"));

    for i in 2..=limit {
        for (j, node) in relevant_nodes.iter().enumerate() {
            for subset in (0..=relevant_nodes.len()).flat_map(|count| (0..relevant_nodes.len()).combinations(count)) {
                // create a vector of size relevant_nodes.len() with all values 0
                let mut vec = vec![0; relevant_nodes.len()];
                // set all values in vec to 1 where the index is in subset
                for k in subset {
                    vec[k] = 1;
                }

                let value: u32;

                if vec[j] == 1 {
                    // current valve was opened, can only get the best by moving
                    value = relevant_nodes.iter().enumerate().map( |(idx, dst)| {
                        if idx != j && solution.get(&(*node, *dst)).unwrap() <= &(i as u32) {
                            *state_space[i as usize - *solution.get(&(*node, *dst)).unwrap() as usize][idx].get(&vec).unwrap()
                        } else {
                            0
                        }
                    }).max().unwrap();
                } else {
                    let not_opened = relevant_nodes.iter().enumerate().map( |(idx, dst)| {
                        if idx != j && solution.get(&(*node, *dst)).unwrap() <= &(i as u32) {
                            *state_space[i as usize - *solution.get(&(*node, *dst)).unwrap() as usize][idx].get(&vec).unwrap()
                        } else {
                            0
                        }
                    }).max().unwrap();
                    let mut opened_state = vec.clone();
                    opened_state[j] = 1;
                    let opened = relevant_nodes.iter().enumerate().map( |(idx, dst)| {
                        if idx != j && *solution.get(&(*node, *dst)).unwrap() < (i as u32) {
                            *state_space[i as usize - 1 - *solution.get(&(*node, *dst)).unwrap() as usize][idx].get(&opened_state).unwrap()
                        } else {
                            0
                        }
                    }).max().unwrap() + valves.get(*node).unwrap().flow * (i as u32 -1);
                    value = not_opened.max(opened);
                }
                // insert vec into state_space[i] with value
                state_space[i][j].insert(vec, value);
                pb.inc(1);
            }
        }
    }
    if ! use_elephant {
        // when no elephant is used, we can just return the maximum value in the last state for valve AA
        Some(*(state_space[30][relevant_nodes.iter().enumerate().find_map(|(idx, name)| if *name == "AA" {Some(idx)} else {None}).unwrap() as usize].values().max().unwrap()))
    } else {
        // when an elephant is used, we have to check all possible combinations of final states and add the release flow if the elephant opened disjoint valves
        let to_check = state_space[limit][relevant_nodes.iter().enumerate().find_map(|(idx, name)| if *name == "AA" {Some(idx)} else {None}).unwrap() as usize].len().pow(2) as u64;
        println!("\nChecking {} state combinations", to_check);
        let pb = indicatif::ProgressBar::new(to_check);
            pb.set_style(ProgressStyle::with_template("[{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));

        state_space[limit][relevant_nodes.iter().enumerate().find_map(|(idx, name)| if *name == "AA" {Some(idx)} else {None}).unwrap() as usize].iter().cartesian_product(state_space[limit][relevant_nodes.iter().enumerate().find_map(|(idx, name)| if *name == "AA" {Some(idx)} else {None}).unwrap() as usize].iter()).filter_map(|((state_1, val1), (state_2, val2))| {
            pb.inc(1);
            if states_are_compatible(state_1, state_2) {
                Some(val1 + val2)
            } else {
                None
            }
         }).max()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    simulate_valve_run(input, 30, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    simulate_valve_run(input, 26, true)
}

fn main() {
    let input = &aoc::read_file("inputs", 16);
    aoc::solve!(16, 1, part_one, input);
    aoc::solve!(16, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
