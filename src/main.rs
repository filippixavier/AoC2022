use std::error::Error;
use std::io::{self};

type Star = fn() -> Result<(), Box<dyn Error + 'static>>;

fn default_star() -> Result<(), Box<dyn Error + 'static>> {
    println!("Invalid day or not published yet");
    Ok(())
}

fn main() {
    let mut buffer = String::new();
    let mut day: usize = 0;

    let first_star: Star;
    let second_star: Star;

    println!("Welcome to Advent of Code, 2022 edition!");
    println!("Please enter the day number:");

    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            day = buffer.trim_end().parse().unwrap_or(0);
            if day > 0 {
                println!("Attemptin to run day {}...", day);
            }
        }
        Err(error) => println!("Error: {}", error),
    }

    match day {
        _ => {
            println!("No day found matching input, exiting...");
            first_star = default_star;
            second_star = default_star;
            return;
        }
    }

    first_star();
    second_star();
}
