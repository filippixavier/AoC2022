use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]

enum Tile<T> {
    Line(T),
    Plus(T),
    L(T),
    Column(T),
    Square(T),
}

use Direction::*;
use Tile::*;

fn get_input() -> Vec<Direction> {
    fs::read_to_string(Path::new("./input/day17.input"))
        .expect("Something went wrong with the input")
        .trim()
        .chars()
        .map(|dir| if dir == '>' { Right } else { Left })
        .collect()
}

fn compute_highest(movement: Vec<Direction>, mut rounds: u128) -> isize {
    let mut movement = movement.iter().cycle();
    let tiles = vec![
        Line(vec![(1, 0), (2, 0), (3, 0)]),
        Plus(vec![(1, 0), (-1, 0), (0, 1), (0, -1)]),
        L(vec![(-1, 0), (-2, 0), (0, 1), (0, 2)]),
        Column(vec![(0, 1), (0, 2), (0, 3)]),
        Square(vec![(1, 0), (0, 1), (1, 1)]),
    ];
    let mut tiles = tiles.into_iter().cycle();

    let mut highest_per_column = [0; 7];

    let mut highest_point = 0;

    let mut board: HashMap<(isize, isize), bool> = HashMap::new();

    while rounds != 0 {
        let tile = tiles.next().unwrap();
        let highest_rel_point = *highest_per_column.iter().max().unwrap();
        let mut coordinate;
        let other_relative_coordiates;
        match tile {
            Line(shape) => {
                coordinate = (2, highest_rel_point + 3);
                other_relative_coordiates = shape.clone();
            }
            Plus(shape) => {
                coordinate = (3, highest_rel_point + 4);
                other_relative_coordiates = shape.clone();
            }
            L(shape) => {
                coordinate = (4, highest_rel_point + 3);
                other_relative_coordiates = shape.clone();
            }
            Column(shape) => {
                coordinate = (2, highest_rel_point + 3);
                other_relative_coordiates = shape.clone();
            }
            Square(shape) => {
                coordinate = (2, highest_rel_point + 3);
                other_relative_coordiates = shape.clone();
            }
        }

        loop {
            let change_pos = movement.next().unwrap();

            if let Left = change_pos {
                let moved_shape: Vec<(isize, isize)> = vec![(coordinate.0 - 1, coordinate.1)]
                    .into_iter()
                    .chain(
                        other_relative_coordiates
                            .iter()
                            .map(|other| (coordinate.0 + other.0 - 1, coordinate.1 + other.1)),
                    )
                    .collect();
                if moved_shape
                    .iter()
                    .all(|elem| elem.0 >= 0 && !*board.entry(*elem).or_insert(false))
                {
                    coordinate = (coordinate.0 - 1, coordinate.1);
                }
            } else {
                let moved_shape: Vec<(isize, isize)> = vec![(coordinate.0 + 1, coordinate.1)]
                    .into_iter()
                    .chain(
                        other_relative_coordiates
                            .iter()
                            .map(|other| (coordinate.0 + other.0 + 1, coordinate.1 + other.1)),
                    )
                    .collect();
                if moved_shape
                    .iter()
                    .all(|elem| elem.0 < 7 && !*board.entry(*elem).or_insert(false))
                {
                    coordinate = (coordinate.0 + 1, coordinate.1);
                }
            }

            let can_fall = {
                let down = (coordinate.0, coordinate.1 - 1);
                let shape: Vec<(isize, isize)> = vec![down]
                    .into_iter()
                    .chain(
                        other_relative_coordiates
                            .iter()
                            .map(|other| (down.0 + other.0, down.1 + other.1)),
                    )
                    .collect();
                shape.iter().all(|elem| {
                    return elem.1 >= 0 && !*board.entry(*elem).or_insert(false);
                })
            };

            if can_fall {
                coordinate = (coordinate.0, coordinate.1 - 1);
            } else {
                let shape: Vec<(isize, isize)> = vec![coordinate]
                    .into_iter()
                    .chain(
                        other_relative_coordiates
                            .iter()
                            .map(|other| (coordinate.0 + other.0, coordinate.1 + other.1)),
                    )
                    .collect();
                for tile in shape {
                    highest_per_column[tile.0 as usize] =
                        highest_per_column[tile.0 as usize].max(tile.1 + 1);
                    board.insert(tile, true);
                }
                break;
            }
        }

        let min_rel_height = *highest_per_column.iter().min().unwrap_or(&0);

        if min_rel_height != 0 {
            highest_per_column = highest_per_column
                .into_iter()
                .map(|elem| elem - min_rel_height)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            board = board
                .into_iter()
                .filter_map(|(key, value)| {
                    if key.1 < min_rel_height {
                        None
                    } else {
                        Some(((key.0, key.1 - min_rel_height), value))
                    }
                })
                .collect();
            highest_point += min_rel_height;
        }
        rounds -= 1;
    }
    highest_point + highest_per_column.iter().max().unwrap()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let movement = get_input();

    println!(
        "After 2022 block, tower will be {} high",
        compute_highest(movement, 2022)
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let movement = get_input();

    println!(
        "After 1 000 000 000 000 block, tower will be {} high",
        compute_highest(movement, 1_000_000_000_000)
    );

    Ok(())
}
