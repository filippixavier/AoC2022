use std::collections::HashMap;
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

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let movement = get_input();
    let mut movement = movement.iter().cycle();
    let mut tiles = vec![
        Line(vec![(1, 0), (2, 0), (3, 0)]),
        Plus(vec![(1, 0), (-1, 0), (0, 1), (0, -1)]),
        L(vec![(-1, 0), (-2, 0), (0, 1), (0, 2)]),
        Column(vec![(0, 1), (0, 2), (0, 3)]),
        Square(vec![(1, 0), (0, 1), (1, 1)]),
    ];
    tiles = tiles.into_iter().cycle().take(2022).collect();

    let mut highest_point = 0;

    let mut board: HashMap<(isize, isize), bool> = HashMap::new();

    for tile in tiles.iter() {
        let mut coordinate;
        let other_relative_coordiates;
        match tile {
            Line(shape) => {
                coordinate = (2, highest_point + 3);
                other_relative_coordiates = shape.clone();
            }
            Plus(shape) => {
                coordinate = (3, highest_point + 4);
                other_relative_coordiates = shape.clone();
            }
            L(shape) => {
                coordinate = (4, highest_point + 3);
                other_relative_coordiates = shape.clone();
            }
            Column(shape) => {
                coordinate = (2, highest_point + 3);
                other_relative_coordiates = shape.clone();
            }
            Square(shape) => {
                coordinate = (2, highest_point + 3);
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
                    highest_point = highest_point.max(tile.1 + 1);
                    board.insert(tile, true);
                }
                break;
            }
        }
    }

    // for y in (0..=highest_point).rev() {
    //     let mut scan_line = vec![];
    //     for x in 0..7 {
    //         scan_line.push(if *board.entry((x, y)).or_insert(false) {
    //             '#'
    //         } else {
    //             '.'
    //         });
    //     }
    //     println!("{}", scan_line.into_iter().collect::<String>());
    // }

    println!("After 2022 block, tower will be {} high", highest_point);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
