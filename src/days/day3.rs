use std::error::Error;
use std::fs;
use std::path::Path;

const VALUES: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_input() -> Vec<String> {
    fs::read_to_string(Path::new("./input/day3.input"))
        .expect("Something went wrong with the input")
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let rucksacks = get_input();
    let mut score = 0;
    for (part_1, part_2) in rucksacks.iter().map(|line| {
        let count = line.chars().count();
        (
            line.chars().take(count / 2).collect::<String>(),
            line.chars().skip(count / 2).collect::<String>(),
        )
    }) {
        for item in part_1.chars() {
            if part_2.contains(item) {
                score += VALUES.find(item).unwrap_or_default();
                break;
            }
        }
    }

    println!("The priority sum of all misplaced items is {}", score);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let rucksacks = get_input();
    let mut score = 0;

    for group in rucksacks.chunks(3) {
        for item in group[0].chars() {
            if group[1].contains(item) && group[2].contains(item) {
                score += VALUES.find(item).unwrap_or_default();
                break;
            }
        }
    }
    println!("The priority sum of all group badges is {}", score);
    Ok(())
}
