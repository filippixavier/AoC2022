use std::error::Error;
use std::fs;
use std::path::Path;

fn get_input(crypto: isize) -> Vec<isize> {
    fs::read_to_string(Path::new("./input/day20.input"))
        .expect("Something went wrong with the input")
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn mix(input: &Vec<isize>) -> Vec<isize> {
    input.clone()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let input = get_input(1);
    let mixed = mix(&input);

    println!("{:?}", mixed);
    let zero = mixed.iter().position(|x| x == &0).unwrap_or(0);
    println!(
        "Encryption sum is: {}",
        (1..=3)
            .map(|elem| mixed[(elem * 1_000 + zero) % input.len()])
            .sum::<isize>()
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
