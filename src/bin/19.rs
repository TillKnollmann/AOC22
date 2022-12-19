use std::{collections::HashSet, default};
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

/// An inventory
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Inventory {
    ore: u32,
    ore_robot: u32,
    clay: u32,
    clay_robot: u32,
    obsidian: u32,
    obsidian_robot: u32,
    geode: u32,
    geode_robot: u32,
}

/// A blueprint
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Blueprint {
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
    max_number_geodes: u32,
    index: u32,
}

/// Parses all blueprints and returns them as a vector
fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    let mut result: Vec<Blueprint> = vec![];
    let mut count = 1;
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let ore_robot_cost = words.nth(6).unwrap().parse().unwrap();
        let clay_robot_cost = words.nth(5).unwrap().parse().unwrap();
        let obsidian_robot_ore_cost = words.nth(5).unwrap().parse().unwrap();
        let obsidian_robot_clay_cost = words.nth(2).unwrap().parse().unwrap();
        let geode_robot_ore_cost = words.nth(5).unwrap().parse().unwrap();
        let geode_robot_obsidian_cost = words.nth(2).unwrap().parse().unwrap();
        let max_number_geodes = 0;
        result.push(Blueprint {
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
            max_number_geodes,
            index: count,
        });
        count += 1;
    }
    result
}

/// Simulates the blueprint for the given amount of minutes and sets the maximum number of possible geodes to get from it
fn simulate_blueprint(blueprint: &mut Blueprint, minutes: u32) {
    let mut inventories: HashSet<Inventory> = HashSet::new();
    inventories.insert(Inventory {
        ore_robot: 1,
        ..default::Default::default()
    });

    for _ in 1..=minutes {
        let mut new_inventories: HashSet<Inventory> = HashSet::new();
        for mut invent in inventories {
            let mut new_invents: Vec<Inventory> = vec![];

            if blueprint.geode_robot_obsidian_cost <= invent.obsidian
                && blueprint.geode_robot_ore_cost <= invent.ore
            {
                new_invents.push(Inventory {
                    geode_robot: invent.geode_robot + 1,
                    obsidian: invent.obsidian - blueprint.geode_robot_obsidian_cost + invent.obsidian_robot,
                    ore: invent.ore - blueprint.geode_robot_ore_cost + invent.ore_robot,
                    ore_robot: invent.ore_robot,
                    clay: invent.clay + invent.clay_robot,
                    clay_robot: invent.clay_robot,
                    obsidian_robot: invent.obsidian_robot,
                    geode: invent.geode + invent.geode_robot,
                });
            }
            if blueprint.obsidian_robot_clay_cost <= invent.clay
                && blueprint.obsidian_robot_ore_cost <= invent.ore
            {
                new_invents.push(Inventory {
                    obsidian_robot: invent.obsidian_robot + 1,
                    clay: invent.clay - blueprint.obsidian_robot_clay_cost + invent.clay_robot,
                    ore: invent.ore - blueprint.obsidian_robot_ore_cost + invent.ore_robot,
                    ore_robot: invent.ore_robot,
                    clay_robot: invent.clay_robot,
                    obsidian: invent.obsidian + invent.obsidian_robot,
                    geode_robot: invent.geode_robot,
                    geode: invent.geode + invent.geode_robot,
                });
            } else {
                if blueprint.clay_robot_cost <= invent.ore {
                    new_invents.push(Inventory {
                        clay_robot: invent.clay_robot + 1,
                        ore: invent.ore - blueprint.clay_robot_cost + invent.ore_robot,
                        ore_robot: invent.ore_robot,
                        clay: invent.clay + invent.clay_robot,
                        obsidian_robot: invent.obsidian_robot,
                        obsidian: invent.obsidian + invent.obsidian_robot,
                        geode_robot: invent.geode_robot,
                        geode: invent.geode + invent.geode_robot,
                    });
                }
                if blueprint.ore_robot_cost <= invent.ore {
                    new_invents.push(Inventory {
                        ore_robot: invent.ore_robot + 1,
                        ore: invent.ore - blueprint.ore_robot_cost + invent.ore_robot,
                        clay_robot: invent.clay_robot,
                        clay: invent.clay+ invent.clay_robot,
                        obsidian_robot: invent.obsidian_robot,
                        obsidian: invent.obsidian + invent.obsidian_robot,
                        geode_robot: invent.geode_robot,
                        geode: invent.geode + invent.geode_robot,
                    });
                }
                invent.ore += invent.ore_robot;
                invent.clay += invent.clay_robot;
                invent.obsidian += invent.obsidian_robot;
                invent.geode += invent.geode_robot;
                new_invents.push(invent);
            }

            let contains_geode = new_invents.iter().any(|invent| invent.geode > 0);

            // get best guesses
            let max_geode_robots = new_invents
                .iter()
                .map(|invent| invent.geode_robot)
                .max()
                .unwrap();

            for new_invent in new_invents {
                if !contains_geode || new_invent.geode_robot == max_geode_robots {
                    new_inventories.insert(new_invent);
                }
            }
        }
        inventories = new_inventories;
    }
    blueprint.max_number_geodes = inventories.iter().map(|invent| invent.geode).max().unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    // get the blueprints
    let mut blueprints = parse_blueprints(input);

    let mut quality_levels = vec![];

    let length = blueprints.len() as u64;

    blueprints.par_iter_mut().progress_count(length).map(|blueprint| {
        simulate_blueprint(blueprint, 24);
        blueprint.max_number_geodes*blueprint.index
    }).collect_into_vec(&mut quality_levels);

    Some(quality_levels.iter().sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    // get the blueprints
    let mut blueprints = parse_blueprints(input);

    let mut quality_levels = vec![];

    blueprints.par_iter_mut().take(3).progress_count(3).map(|blueprint| {
        simulate_blueprint(blueprint, 32);
        blueprint.max_number_geodes
    }).collect_into_vec(&mut quality_levels);

    // print the quality levels
    println!("{:?}", quality_levels);
    Some(quality_levels.iter().product::<u32>())
}

fn main() {
    let input = &aoc::read_file("inputs", 19);
    aoc::solve!(19, 1, part_one, input);
    aoc::solve!(19, 2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }
}
