use std::error::Error;
use std::fs;
use std::path::Path;

enum Op {
    Noop,
    Addx(isize),
}

use Op::*;

fn get_input() -> Vec<Op> {
    fs::read_to_string(Path::new("./input/day10.input"))
        .expect("Something went wrong with the input")
        .trim()
        .lines()
        .map(|line| {
            let input = line.split_whitespace().collect::<Vec<_>>();
            if input.len() == 2 {
                Addx(input[1].parse().unwrap_or(0))
            } else {
                Noop
            }
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let input = get_input();
    let mut reg = 1;
    let mut signal_strength = 0;
    let mut next_check = 0;

    let ops: Vec<Option<isize>> = input
        .iter()
        .flat_map(|op| {
            if let Addx(x) = op {
                vec![None, Some(*x)]
            } else {
                vec![None]
            }
        })
        .collect();

    for (cycle, op) in ops.iter().enumerate() {
        let cy = cycle as isize + 1;
        if cy == 20 + next_check * 40 {
            signal_strength += cy * reg;
            next_check += 1;
        }
        if let Some(x) = op {
            reg += *x;
        }
    }

    println!(
        "Sum of signal strength during targeted cycles: {}",
        signal_strength
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
