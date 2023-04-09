use core::time;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::path::Path;
use std::{fs, path};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: usize,
    leads: Vec<String>,
}

type Network = HashMap<String, Valve>;
type Memoize = HashMap<String, HashMap<String, usize>>;

fn get_input() -> Network {
    fs::read_to_string(Path::new("./input/day16.input"))
        .expect("Something went wrong with the input")
        .trim()
        .lines()
        .map(|line| {
            let infos = line.split_whitespace().collect::<Vec<_>>();
            let name = infos[1].to_string();
            let flow_rate = infos[4].split('=').collect::<Vec<_>>()[1]
                .split(';')
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let leads = infos
                .iter()
                .skip(9)
                .map(|tunnel| tunnel.split(',').next().unwrap().to_string())
                .collect::<Vec<_>>();
            let valve = Valve { flow_rate, leads };
            (name, valve)
        })
        .collect()
}

fn get_score(
    start: String,
    end: String,
    remaining_time: usize,
    network: &Network,
    memoize: &mut Memoize,
) -> Option<(String, usize, usize)> {
    let flow = network.get(&end).unwrap().flow_rate;
    if flow == 0 {
        return None;
    }
    let mut distance = 0;
    let mut memoized = false;

    if let Some(dest) = memoize.get(&start) {
        if let Some(dist) = dest.get(&end) {
            distance = *dist;
            memoized = true;
        }
    }

    if !memoized {
        let mut explore = VecDeque::from(vec![(start.clone(), 0)]);
        let dist_to = memoize.entry(start.clone()).or_default();

        while !explore.is_empty() {
            let (current_valve, dist) = explore.pop_front().unwrap();
            if current_valve == end {
                distance = dist;
                break;
            }
            if let Some(valves) = network.get(&current_valve) {
                let mut valve_dist: VecDeque<_> = valves
                    .leads
                    .iter()
                    .filter(|dest_valve| *dest_valve != &start)
                    .map(|dest_valve| {
                        if !dist_to.contains_key(dest_valve) {
                            dist_to.insert(dest_valve.clone(), dist + 1);
                        }
                        (dest_valve.clone(), dist + 1)
                    })
                    .collect();
                explore.append(&mut valve_dist);
            }
        }
    }
    let time_after_move = remaining_time.saturating_sub(distance);
    if time_after_move != 0 {
        let total_score = (time_after_move - 1) * flow;
        Some((end, time_after_move - 1, total_score))
    } else {
        None
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let valves = get_input();
    let mut states: VecDeque<(String, usize, usize, Vec<String>)> = VecDeque::new();
    states.push_front((
        String::from("AA"),
        30,
        0,
        valves.keys().cloned().filter(|key| key != "AA").collect(),
    ));
    let mut max_pressure = 0;
    let mut memoized = HashMap::new();
    while !states.is_empty() {
        let (current_valve, remaining_time, score, remaining) = states.pop_front().unwrap();
        max_pressure = max_pressure.max(score);
        let candidates = remaining.iter().flat_map(|target| {
            get_score(
                current_valve.clone(),
                target.clone(),
                remaining_time,
                &valves,
                &mut memoized,
            )
        });
        let mut appendable = candidates
            .map(|(target, next_time, target_score)| {
                (
                    target.clone(),
                    next_time,
                    score + target_score,
                    remaining
                        .iter()
                        .cloned()
                        .filter(|elem| elem != &target)
                        .collect::<Vec<String>>(),
                )
            })
            .collect::<VecDeque<_>>();
        states.append(&mut appendable);
    }
    println!(
        "At most, in 30 minutes, you can free {} pressure",
        max_pressure
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let network = get_input();

    let reachables = network
        .iter()
        .filter_map(|(key, valve)| {
            if valve.flow_rate > 0 {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut time: Vec<Vec<(String, Vec<String>, usize)>> = vec![];

    time[0] = vec![(String::from("AA"), reachables, 0)];

    for i in 0..30 {
        if let Some(cancidates) = time.get(i) {
            for 
        } else {
            continue;
        }
    }

    Ok(())
}
