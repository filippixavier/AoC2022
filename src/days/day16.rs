use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    leads: Vec<String>,
}

type Network = HashMap<String, Valve>;
type Distances = HashMap<String, HashMap<String, usize>>;

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

fn get_distance(valves: &Network) -> Distances {
    let mut distances = HashMap::new();

    for source in valves.keys() {
        let mut distance = HashMap::new();
        distance.insert(source.to_string(), 0);
        let mut to_explore = vec![source];
        while !to_explore.is_empty() {
            let destination = to_explore.pop().unwrap();
            let valve = valves.get(destination).unwrap();
            if !distance.contains_key(destination) {
                distance.insert(destination.clone(), 0);
            }
            let next_step = *distance.get(destination).unwrap();

            for next_dest in valve.leads.iter() {
                if !distance.contains_key(next_dest) {
                    distance.insert(next_dest.clone(), next_step + 1);
                    to_explore.push(next_dest);
                }
            }
        }

        distances.insert(source.to_string(), distance);
    }

    distances
}

fn reverse_flow(end: String, time: usize, valves: &Network) -> (usize, Vec<String>) {
    let mut max_flow = 0;
    let mut max_path = vec![];

    let origins = valves
        .keys()
        .cloned()
        .filter(|key| key != &end)
        .collect::<Vec<_>>();

    for origin in origins {
        let mut distances: HashMap<String, (usize, Vec<String>)> = HashMap::new();

        let valve = valves.get(&origin).unwrap();

        let mut status: Vec<(String, usize, HashSet<String>, Vec<String>)> = valve
            .leads
            .iter()
            .map(|name| {
                (
                    name.clone(),
                    valve.flow_rate,
                    HashSet::new(),
                    vec![name.to_string()],
                )
            })
            .collect();

        for step in 1..=30 {
            let mut next = vec![];

            for (current, flow, opened, path) in status {
                let valve = valves.get(&current).unwrap();

                if valve.flow_rate != 0 && !opened.contains(&current) {
                    let new_flow = flow + step * valve.flow_rate;
                    let (distance, max_path) =
                        distances.entry(current.clone()).or_insert((0, vec![]));
                    let mut new_path = path.clone();

                    new_path.push(current.clone());

                    if *distance >= new_flow {
                        continue;
                    } else {
                        *distance = new_flow;
                        *max_path = new_path.clone();
                    }

                    let new_lock = opened
                        .iter()
                        .cloned()
                        .chain((vec![current.clone()]).into_iter())
                        .collect();
                    next.push((current.clone(), new_flow, new_lock, new_path));
                }

                for next_valve in valve.leads.iter() {
                    let (distance, max_path) =
                        distances.entry(next_valve.clone()).or_insert((0, vec![]));
                    if *distance < flow {
                        *distance = flow;
                        let mut new_path = path.clone();
                        new_path.push(next_valve.clone());
                        *max_path = new_path.clone();
                        next.push((next_valve.clone(), flow, opened.clone(), new_path));
                    }
                }
            }

            status = next;
        }

        if let Some((dist, path)) = distances.get(&end) {
            if max_flow < *dist {
                max_flow = *dist;
                max_path = path.clone();
            }
        }
    }

    (max_flow, max_path.into_iter().rev().collect::<Vec<_>>())
}

// fn most_flow(start: String, time: usize, valves: &Network, distances: &Distances) -> usize {
//     let mut max_flow = 0;
//     let mut state: Vec<(String, usize, usize, HashSet<String>)> = vec![];
//     state.push((start.clone(), 0, 0, vec![start].into_iter().collect()));

//     while !state.is_empty() {
//         let (current, step, score, opened) = state.pop().unwrap();

//         let distances = distances.get(&current).unwrap();

//         for (target_name, &target_distance) in distances {
//             if target_distance == 0 || opened.contains(target_name) {
//                 continue;
//             }

//             let next_step = step + target_distance + 1;
//             let target_valve = valves.get(target_name).unwrap();

//             if target_valve.flow_rate == 0 {
//                 continue;
//             }

//             let next_score = score + target_valve.flow_rate * time.saturating_sub(next_step);
//             let mut next_opened = opened.clone();
//             next_opened.insert(target_name.clone());

//             state.push((target_name.clone(), next_step, next_score, next_opened));
//         }
//         max_flow = max_flow.max(score);
//     }
//     max_flow
// }

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let valves = get_input();
    //let distances = get_distance(&valves);

    println!("Distance computed");

    // println!(
    //     "The most flow that can be released is {}",
    //     most_flow(String::from("AA"), 30, &valves, &distances)
    // );

    let (temp, path) = reverse_flow(String::from("AA"), 30, &valves);

    let mut current_name = "";

    let total = path.iter().enumerate().fold(0, |acc, (step, name)| {
        if name != current_name {
            current_name = name.as_str();
            acc
        } else {
            let valve = valves.get(name).unwrap();
            acc + 30usize.saturating_sub(step) * valve.flow_rate
        }
    });

    println!("{:?}", path);

    println!("{} {}", temp, total);

    // println!("Test: {}", reverse_flow(String::from("AA"), 30, &valves));

    Ok(())
}

// 1664 too low

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
