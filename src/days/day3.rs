use std::error::Error;
use std::fs;
use std::path::Path;

fn get_input() -> Vec<(String, String)> {
    fs::read_to_string(Path::new("./input/day3.input"))
        .expect("Something went wrong with the input")
        .trim()
        .lines()
        .map(|line| {
            let count = line.chars().count();
            (
                line.chars().take(count / 2).collect(),
                line.chars().skip(count / 2).collect(),
            )
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let values = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let rucksacks = get_input();
    let mut score = 0;
    for (part_1, part_2) in rucksacks {
        for item in part_1.chars() {
            if part_2.contains(item) {
                score += values.find(item).unwrap_or_default();
                break;
            }
        }
    }

    println!("The priority sum of all misplaced items is {}", score);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
