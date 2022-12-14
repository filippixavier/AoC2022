use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

type Coordinates = (usize, usize);

#[derive(Debug)]
enum Tile {
    Sand,
    Rock,
}

use Tile::*;

fn get_input() -> Vec<Vec<Coordinates>> {
    fs::read_to_string(Path::new("./input/day14.input"))
        .expect("Something went wrong with the input")
        .trim()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|elem| {
                    let coordinates = elem
                        .split(',')
                        .map(|value| value.parse::<usize>())
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap();
                    (coordinates[0], coordinates[1])
                })
                .collect()
        })
        .collect()
}

fn get_map(rocks: &Vec<Vec<Coordinates>>) -> (HashMap<Coordinates, Tile>, Coordinates) {
    let mut sand_map: HashMap<Coordinates, Tile> = HashMap::new();

    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;

    for lines in rocks {
        let mut point = lines[0];
        min_x = min_x.min(point.0);
        max_x = max_x.max(point.0);
        for next_point in lines.iter().skip(1) {
            min_x = min_x.min(next_point.0);
            max_x = max_x.max(next_point.0);
            for right in point.0.min(next_point.0)..=point.0.max(next_point.0) {
                sand_map.insert((right, point.1), Rock);
            }
            for down in point.1.min(next_point.1)..=point.1.max(next_point.1) {
                sand_map.insert((point.0, down), Rock);
            }
            point = *next_point;
        }
    }

    (sand_map, (min_x, max_x))
}

fn visualize(map: &HashMap<Coordinates, Tile>) {
    let mut min = (usize::MAX, usize::MAX);
    let mut max = (usize::MIN, usize::MIN);

    for point in map.keys() {
        min.0 = min.0.min(point.0);
        min.1 = min.1.min(point.1);
        max.0 = max.0.max(point.0);
        max.1 = max.1.max(point.1);
    }

    for y in min.1..=max.1 {
        let mut line = vec![];
        for x in min.0..=max.0 {
            line.push(if let Some(x) = map.get(&(x, y)) {
                match x {
                    Sand => "+",
                    Rock => "#",
                }
            } else {
                "."
            });
        }
        println!("{}", line.concat());
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let path = get_input();
    let (mut sand_map, borders) = get_map(&path);

    let start_sand = (500, 0);
    let mut current_sand = start_sand;
    let mut is_forever_falling = false;
    let mut is_settled = true;

    while !is_forever_falling {
        if is_settled {
            current_sand = start_sand;
            sand_map.insert(current_sand, Sand);
            is_settled = false;
        }

        let inspect = [
            (current_sand.0, current_sand.1 + 1),
            (current_sand.0 - 1, current_sand.1 + 1),
            (current_sand.0 + 1, current_sand.1 + 1),
        ];

        let mut moved = false;

        for possible in inspect {
            if sand_map.get(&possible).is_none() {
                sand_map.remove(&current_sand);
                sand_map.insert(possible, Sand);
                current_sand = possible;
                moved = true;
                break;
            }
        }

        if !moved {
            is_settled = true;
        }

        if current_sand.0 < borders.0 || current_sand.0 > borders.1 {
            is_forever_falling = true;
        }
    }

    println!("Full map visualization:");
    visualize(&sand_map);

    println!(
        "Amount of rested sand: {}",
        sand_map.values().fold(0, |acc, x| {
            if let Sand = x {
                acc + 1
            } else {
                acc
            }
        }) - 1
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let path = get_input();
    let (mut sand_map, _) = get_map(&path);

    let floor = sand_map.keys().map(|(_, y)| y).max().unwrap() + 2;

    let start_sand = (500, 0);
    let mut current_sand = start_sand;
    let mut is_entry_blocked = false;
    let mut is_settled = true;

    while !is_entry_blocked {
        if is_settled {
            current_sand = start_sand;
            sand_map.insert(current_sand, Sand);
            is_settled = false;
        }

        let inspect = [
            (current_sand.0, current_sand.1 + 1),
            (current_sand.0 - 1, current_sand.1 + 1),
            (current_sand.0 + 1, current_sand.1 + 1),
        ];

        let mut moved = false;

        for possible in inspect {
            if possible.1 != floor && sand_map.get(&possible).is_none() {
                sand_map.remove(&current_sand);
                sand_map.insert(possible, Sand);
                current_sand = possible;
                moved = true;
                break;
            }
        }

        if !moved {
            is_settled = true;
        }

        if current_sand == start_sand && is_settled {
            is_entry_blocked = true;
        }
    }

    println!("Full map visualization with floor:");
    visualize(&sand_map);

    println!(
        "Amount of rested sand (including floor): {}",
        sand_map.values().fold(0, |acc, x| {
            if let Sand = x {
                acc + 1
            } else {
                acc
            }
        })
    );

    Ok(())
}
