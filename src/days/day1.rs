use std::error::Error;
use std::fs;
use std::path::Path;

fn get_input() -> Vec<Vec<usize>> {
    let input = fs::read_to_string(Path::new("./input/day1.input"))
        .expect("Something went wrong with the input");

    let mut elves: Vec<Vec<usize>> = vec![];
    let mut calories: Vec<usize> = vec![];
    for line in input.trim().lines() {
        if line.is_empty() {
            elves.push(calories);
            calories = vec![];
            continue;
        }
        calories.push(line.trim().parse::<usize>().unwrap_or(0))
    }

    elves
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let elves = get_input();
    let elves_calories: Vec<usize> = elves
        .iter()
        .map(|inventory| inventory.iter().sum())
        .collect();
    println!(
        "At most, an elf have {} calories",
        elves_calories
            .iter()
            .fold(usize::MIN, |acc, calory| acc.max(*calory))
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let elves = get_input();
    let mut elves_calories: Vec<usize> = elves
        .iter()
        .map(|inventory| inventory.iter().sum())
        .collect();
    elves_calories.sort();
    println!(
        "Amount of calories carried by the top 3: {}",
        elves_calories.iter().rev().take(3).sum::<usize>()
    );
    Ok(())
}
