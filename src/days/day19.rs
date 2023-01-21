use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

type Blueprint = [[usize; 4]; 4];

fn get_input() -> Vec<Blueprint> {
    let reg =
        Regex::new(r"ore .*?(\d+).*?\s*?.*?(\d+).*?\s*?.*?(\d+).*?(\d+).*?\s*?.*?(\d+).*?(\d+)")
            .unwrap();
    let input = fs::read_to_string(Path::new("./input/day19.input"))
        .expect("Something went wrong with the input");

    let mut blueprints = vec![];

    for recipe in reg.captures_iter(&input) {
        let ore_print = [recipe[1].parse().unwrap(), 0, 0, 0];
        let clay_print = [recipe[2].parse().unwrap(), 0, 0, 0];
        let obsidian_print = [recipe[3].parse().unwrap(), recipe[4].parse().unwrap(), 0, 0];
        let geode_print = [recipe[5].parse().unwrap(), 0, recipe[6].parse().unwrap(), 0];

        let blueprint = [ore_print, clay_print, obsidian_print, geode_print];

        blueprints.push(blueprint);
    }

    blueprints
}

fn min_potential(turn: usize, max_turn: usize) -> usize {
    max_turn.saturating_sub(turn)
}

fn max_potential(turn: usize, max_turn: usize) -> usize {
    let remaining = max_turn.saturating_sub(turn);
    remaining * (remaining + 1) / 2
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let max_turn = 24;
    let blueprints = get_input();
    let mut total_ql = 0;

    for (bp_index, blueprint) in blueprints.iter().enumerate() {
        let mut duplicates: HashSet<String> = HashSet::new();
        let mut ql = 0;
        let mut config: Vec<(usize, [usize; 4], [usize; 4], usize)> =
            vec![(1, [0, 0, 0, 0], [1, 0, 0, 0], 0)];
        let mut min_geodes = 0;

        while !config.is_empty() {
            let mut next_config = vec![];
            for (steps, resources, robots, total_geodes) in config {
                if !duplicates.insert(format!("{}-{:?}-{:?}", steps, resources, robots)) {
                    continue;
                }
                if steps > max_turn {
                    ql = ql.max(resources[3]);
                    continue;
                }

                for (index, robot_cost) in blueprint.iter().enumerate() {
                    if robot_cost
                        .iter()
                        .enumerate()
                        .all(|(i, cost)| cost <= &resources[i])
                    {
                        let next_resources: [usize; 4] = resources
                            .iter()
                            .enumerate()
                            .map(|(i, res)| *res - robot_cost[i] + robots[i])
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap();
                        let mut next_robots = robots.clone();
                        next_robots[index] += 1;

                        let mut next_total = total_geodes;

                        if index == 3 {
                            next_total += min_potential(steps + 1, max_turn);
                            min_geodes = min_geodes.max(next_total);
                        }
                        next_config.push((steps + 1, next_resources, next_robots, next_total));
                    }
                }

                let next_resources: [usize; 4] = resources
                    .iter()
                    .enumerate()
                    .map(|(i, res)| *res + robots[i])
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                next_config.push((steps + 1, next_resources, robots, total_geodes));
            }
            config = next_config
                .into_iter()
                .filter(|(steps, _, _, total_geodes)| {
                    max_potential(steps + 1, max_turn) + total_geodes >= min_geodes
                })
                .collect();
        }

        total_ql += ql * (bp_index + 1);
    }

    println!("Total quality level of blueprints is: {}", total_ql);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let max_turn = 32;
    let blueprints = get_input().into_iter().take(3).collect::<Vec<_>>();

    let mut max_geode = 0;

    for blueprint in blueprints.iter() {
        let mut duplicates: HashSet<String> = HashSet::new();
        let mut ql = 0;
        let mut config: Vec<(usize, [usize; 4], [usize; 4], usize)> =
            vec![(1, [0, 0, 0, 0], [1, 0, 0, 0], 0)];
        let mut min_geodes = 1;

        while !config.is_empty() {
            let mut next_config = vec![];
            for (steps, resources, robots, total_geodes) in config {
                if !duplicates.insert(format!("{}-{:?}-{:?}", steps, resources, robots)) {
                    continue;
                }
                if steps > max_turn {
                    ql = ql.max(resources[3]);
                    continue;
                }

                for (index, robot_cost) in blueprint.iter().enumerate() {
                    if robot_cost
                        .iter()
                        .enumerate()
                        .all(|(i, cost)| cost <= &resources[i])
                    {
                        let next_resources: [usize; 4] = resources
                            .iter()
                            .enumerate()
                            .map(|(i, res)| *res - robot_cost[i] + robots[i])
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap();
                        let mut next_robots = robots;
                        next_robots[index] += 1;

                        let mut next_total = total_geodes;

                        if index == 3 {
                            next_total += min_potential(steps + 1, max_turn);
                            min_geodes = min_geodes.max(next_total);
                        }
                        next_config.push((steps + 1, next_resources, next_robots, next_total));
                    }
                }

                let next_resources: [usize; 4] = resources
                    .iter()
                    .enumerate()
                    .map(|(i, res)| *res + robots[i])
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                next_config.push((steps + 1, next_resources, robots, total_geodes));
            }
            config = next_config
                .into_iter()
                .filter(|(steps, _, _, total_geodes)| {
                    max_potential(steps + 1, max_turn) + total_geodes >= min_geodes
                })
                .collect();
        }

        max_geode *= ql;
    }

    println!(
        "Max geodes for the first 3 blueprints after {} turns is: {}",
        max_turn, max_geode
    );

    Ok(())
}
