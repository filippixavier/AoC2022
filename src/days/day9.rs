use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

type Coordinates = (isize, isize);
type Trail = HashSet<Coordinates>;

fn move_head(start: Coordinates, direction: Direction) -> Coordinates {
    match direction {
        Up => (start.0 + 1, start.1),
        Down => (start.0 - 1, start.1),
        Left => (start.0, start.1 - 1),
        Right => (start.0, start.1 + 1),
    }
}

fn move_tail(start: Coordinates, head: &Coordinates) -> Coordinates {
    let up = head.0 - start.0;
    let right = head.1 - start.1;

    if up.abs() > 1 {
        (
            start.0 + if up < 0 { -1 } else { 1 },
            start.1
                + match right.cmp(&0) {
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                },
        )
    } else if right.abs() > 1 {
        (
            start.0
                + match up.cmp(&0) {
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                },
            start.1 + if right < 0 { -1 } else { 1 },
        )
    } else {
        start
    }
}

fn get_input() -> Vec<(Direction, usize)> {
    fs::read_to_string(Path::new("./input/day9.input"))
        .expect("Something went wrong with the input")
        .trim()
        .lines()
        .map(|line| {
            let instruction: Vec<_> = line.split_whitespace().collect();
            let dir = if instruction[0] == "R" {
                Right
            } else if instruction[0] == "L" {
                Left
            } else if instruction[0] == "U" {
                Up
            } else {
                Down
            };
            let steps = instruction[1].parse::<usize>().unwrap_or(0);
            (dir, steps)
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let instructions = get_input();
    let mut head: Coordinates = (0, 0);
    let mut tail: Coordinates = (0, 0);
    let mut trail: Trail = HashSet::new();

    for (dir, steps) in instructions {
        for _ in 0..steps {
            head = move_head(head, dir);
            tail = move_tail(tail, &head);
            trail.insert(tail);
        }
    }

    println!("Number of positions visited by the rope: {}", trail.len());

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
