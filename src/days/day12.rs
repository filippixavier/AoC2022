use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::path::Path;

type Coordinates = (usize, usize);
type Map = Vec<Vec<isize>>;

fn get_input() -> (Coordinates, Coordinates, Map) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = fs::read_to_string(Path::new("./input/day12.input"))
        .expect("Something went wrong with the input")
        .trim()
        .lines()
        .enumerate()
        .map(|(line_no, line)| {
            line.chars()
                .enumerate()
                .map(|(col_no, elem)| {
                    if elem == 'S' {
                        start = (line_no, col_no);
                        0
                    } else if elem == 'E' {
                        end = (line_no, col_no);
                        25
                    } else {
                        (elem as isize) - ('a' as isize)
                    }
                })
                .collect()
        })
        .collect();
    (start, end, map)
}

fn bfs(start: Coordinates, end: Coordinates, map: &Map) -> usize {
    let mut visited: HashSet<Coordinates> = HashSet::new();
    let mut to_visit: VecDeque<(Coordinates, usize)> = VecDeque::new();

    let (max_line, max_col) = (map.len() - 1, map[0].len() - 1);

    visited.insert(start);
    to_visit.push_back((start, 0));

    while !to_visit.is_empty() {
        let (coordinates, length) = to_visit.pop_front().unwrap();
        if coordinates == end {
            return length;
        }
        let height = map[coordinates.0][coordinates.1];

        if coordinates.0 > 0 {
            let up = (coordinates.0 - 1, coordinates.1);
            let up_height = map[up.0][up.1];
            let diff = height - up_height;
            if diff >= -1 && visited.insert(up) {
                to_visit.push_back((up, length + 1));
            }
        }
        if coordinates.0 < max_line {
            let down = (coordinates.0 + 1, coordinates.1);
            let down_height = map[down.0][down.1];
            let diff = height - down_height;
            if diff >= -1 && visited.insert(down) {
                to_visit.push_back((down, length + 1));
            }
        }
        if coordinates.1 > 0 {
            let left = (coordinates.0, coordinates.1 - 1);
            let left_height = map[left.0][left.1];
            let diff = height - left_height;
            if diff >= -1 && visited.insert(left) {
                to_visit.push_back((left, length + 1));
            }
        }
        if coordinates.1 < max_col {
            let right = (coordinates.0, coordinates.1 + 1);
            let right_height = map[right.0][right.1];
            let diff = height - right_height;
            if diff >= -1 && visited.insert(right) {
                to_visit.push_back((right, length + 1));
            }
        }
    }
    0
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (start, end, map) = get_input();
    println!("The shortest path is {} units long", bfs(start, end, &map));
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (_, end, map) = get_input();
    let mut min = usize::MAX;

    let starts = map
        .iter()
        .enumerate()
        .flat_map(|(line_no, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(col_no, col)| {
                    if *col == 0 {
                        Some((line_no, col_no))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for start in starts {
        let result = bfs(start, end, &map);
        if result != 0 {
            min = min.min(result);
        }
    }

    println!("The shortest path of all from a to z is {} units long", min);
    Ok(())
}
