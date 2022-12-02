use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

enum Round {
    Rock(char),
    Paper(char),
    Scissors(char),
}

use Round::*;

fn get_input() -> Vec<Round> {
    fs::read_to_string(Path::new("./input/day2.input"))
        .expect("Something went wrong with the input")
        .trim()
        .lines()
        .map(|line| {
            let round: Vec<char> = line.trim().chars().collect();
            match round[0] {
                'A' => Rock(round[2]),
                'B' => Paper(round[2]),
                'C' => Scissors(round[2]),
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let scoring = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);
    let rounds = get_input();
    let mut score: usize = 0;
    for round in rounds {
        match round {
            Rock(other) => {
                score += scoring.get(&other).unwrap_or(&0);
                score += match other {
                    'X' => 3,
                    'Y' => 6,
                    'Z' => 0,
                    _ => unreachable!(),
                }
            }
            Paper(other) => {
                score += scoring.get(&other).unwrap_or(&0);
                score += match other {
                    'X' => 0,
                    'Y' => 3,
                    'Z' => 6,
                    _ => unreachable!(),
                }
            }
            Scissors(other) => {
                score += scoring.get(&other).unwrap_or(&0);
                score += match other {
                    'X' => 6,
                    'Y' => 0,
                    'Z' => 3,
                    _ => unreachable!(),
                }
            }
        }
    }
    println!("According to the strategy guide, I should get {}", score);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let scoring = HashMap::from([('X', 0), ('Y', 3), ('Z', 6)]);
    let rounds = get_input();
    let mut score: usize = 0;
    for round in rounds {
        match round {
            Rock(other) => {
                score += scoring.get(&other).unwrap_or(&0);
                score += match other {
                    'X' => 3,
                    'Y' => 1,
                    'Z' => 2,
                    _ => unreachable!(),
                }
            }
            Paper(other) => {
                score += scoring.get(&other).unwrap_or(&0);
                score += match other {
                    'X' => 1,
                    'Y' => 2,
                    'Z' => 3,
                    _ => unreachable!(),
                }
            }
            Scissors(other) => {
                score += scoring.get(&other).unwrap_or(&0);
                score += match other {
                    'X' => 2,
                    'Y' => 3,
                    'Z' => 1,
                    _ => unreachable!(),
                }
            }
        }
    }

    println!(
        "According to the REAL strategy guide, I should get {}",
        score
    );
    Ok(())
}
