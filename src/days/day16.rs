use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    leads: Vec<String>,
}

type Network = HashMap<String, Valve>;

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

#[derive(Clone, Debug)]
struct Cell<'a> {
    flow: usize,
    time: usize,
    path: Vec<&'a str>,
}

#[derive(Clone, Debug)]
struct Matrix<'a> {
    cells: HashMap<(&'a str, &'a str), Cell<'a>>,
    vertices: Vec<&'a str>,
}

impl<'a> Matrix<'a> {
    fn new(network: &'a Network) -> Self {
        let mut cells = HashMap::new();

        for (name, valve) in network {
            for target in &valve.leads {
                cells.insert(
                    (target.as_str(), name.as_str()),
                    Cell {
                        flow: valve.flow_rate,
                        time: 1,
                        path: vec![target.as_str(), name.as_str()],
                    },
                );
            }
        }

        Matrix {
            cells,
            vertices: network.keys().map(|key| key.as_str()).collect(),
        }
    }

    fn max_flow_at(&self, turns: usize) -> Self {
        let mut clone = self.clone();
        for permutation in self.vertices.iter().permutations(3) {
            let (start, end, intermediate) = (*permutation[0], *permutation[1], *permutation[2]);

            let cell = clone.cells.get_mut(&(start, end)).unwrap();
            let left_part = self.cells.get(&(start, intermediate)).unwrap();
            let right_part = self.cells.get(&(intermediate, end)).unwrap();
        }
        clone
    }
}

fn get_score(
    start: String,
    end: String,
    remaining_time: usize,
    network: &Network,
    memoize: &mut HashMap<String, HashMap<String, usize>>,
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
    let matrix = Matrix::new(&network);
    println!("{:?}", matrix);
    let mut max_pressure = 0;
    println!(
        "At most, in 26 minutes, you and your elephant can free {} pressure",
        max_pressure
    );
    Ok(())
}
// pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
//     let network = get_input();

//     let mut memoize = HashMap::new();

//     let remaining_valves = network
//         .iter()
//         .filter_map(|(name, valve)| {
//             if name != "AA" && valve.flow_rate > 0 {
//                 Some(name)
//             } else {
//                 None
//             }
//         })
//         .collect::<Vec<_>>();

//     let mut queue: VecDeque<_> =
//         VecDeque::from(vec![("AA", 26, "AA", 26, remaining_valves, 0, 26)]);

//     let mut max_pressure = 0;

//     while !queue.is_empty() {
//         let (
//             p1_start,
//             p1_ready_time,
//             p2_start,
//             p2_ready_time,
//             available_valves,
//             score,
//             remaining_time,
//         ) = queue.pop_front().unwrap();

//         max_pressure = max_pressure.max(score);

//         if available_valves.is_empty() || remaining_time == 0 {
//             break;
//             continue;
//         }

//         if p1_ready_time == p2_ready_time && available_valves.len() >= 2 {
//             let iter: Vec<_> = if p1_start == "AA" {
//                 available_valves.iter().combinations(2).collect()
//             } else {
//                 available_valves.iter().permutations(2).collect()
//             };
//             for targets in iter {
//                 let p1_end = *targets[0];
//                 let mut next_p1_ready = 0;
//                 let mut next_score = score;
//                 let mut remaining_valves = available_valves.clone();
//                 if let Some((_, ready_time, score_to_add)) = get_score(
//                     String::from(p1_start),
//                     String::from(p1_end),
//                     p1_ready_time,
//                     &network,
//                     &mut memoize,
//                 ) {
//                     next_p1_ready = ready_time;
//                     next_score += score_to_add;
//                     remaining_valves = remaining_valves
//                         .into_iter()
//                         .filter(|&name| name != p1_end)
//                         .collect();
//                 }

//                 let p2_end = *targets[1];
//                 let mut next_p2_ready = 0;

//                 if let Some((_, ready_time, score_to_add)) = get_score(
//                     String::from(p2_start),
//                     String::from(p2_end),
//                     p2_ready_time,
//                     &network,
//                     &mut memoize,
//                 ) {
//                     next_p2_ready = ready_time;
//                     next_score += score_to_add;
//                     remaining_valves = remaining_valves
//                         .into_iter()
//                         .filter(|&name| name != p2_end)
//                         .collect();
//                 }

//                 queue.push_back((
//                     p1_end,
//                     next_p1_ready,
//                     p2_end,
//                     next_p2_ready,
//                     remaining_valves,
//                     next_score,
//                     next_p1_ready.max(next_p2_ready),
//                 ))
//             }
//         } else if p1_ready_time == remaining_time && available_valves.len() >= 2 {
//             for target in available_valves.iter() {
//                 let p1_end = *target;
//                 if let Some((_, ready_time, added_score)) = get_score(
//                     String::from(p1_start),
//                     String::from(p1_end),
//                     p1_ready_time,
//                     &network,
//                     &mut memoize,
//                 ) {
//                     let remaining_valves = available_valves
//                         .iter()
//                         .cloned()
//                         .filter(|&name| name != p1_end)
//                         .collect();

//                     queue.push_back((
//                         p1_end,
//                         ready_time,
//                         p2_start,
//                         p2_ready_time,
//                         remaining_valves,
//                         score + added_score,
//                         ready_time.max(p2_ready_time),
//                     ));
//                 }
//             }
//         } else if p2_ready_time == remaining_time && available_valves.len() >= 2 {
//             for target in available_valves.iter() {
//                 let p2_end = *target;
//                 if let Some((_, ready_time, added_score)) = get_score(
//                     String::from(p2_start),
//                     String::from(p2_end),
//                     p2_ready_time,
//                     &network,
//                     &mut memoize,
//                 ) {
//                     let remaining_valves = available_valves
//                         .iter()
//                         .cloned()
//                         .filter(|&name| name != p2_end)
//                         .collect();

//                     queue.push_back((
//                         p1_start,
//                         p1_ready_time,
//                         p2_end,
//                         ready_time,
//                         remaining_valves,
//                         score + added_score,
//                         ready_time.max(p1_ready_time),
//                     ));
//                 }
//             }
//         } else {
//             let target = available_valves[0];
//             let mut max = 0;

//             if let Some((_, _, score)) = get_score(
//                 String::from(p1_start),
//                 String::from(target),
//                 p1_ready_time,
//                 &network,
//                 &mut memoize,
//             ) {
//                 max = score;
//             }

//             if let Some((_, _, score)) = get_score(
//                 String::from(p1_start),
//                 String::from(target),
//                 p1_ready_time,
//                 &network,
//                 &mut memoize,
//             ) {
//                 max = max.max(score);
//             }

//             queue.push_back((
//                 p1_start,
//                 p1_ready_time,
//                 p2_start,
//                 p2_ready_time,
//                 vec![],
//                 score + max,
//                 p1_ready_time,
//             ));
//         }
//     }

//     println!(
//         "At most, in 26 minutes, you and your elephant can free {} pressure",
//         max_pressure
//     );
//     Ok(())
// }
