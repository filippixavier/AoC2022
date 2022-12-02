use std::error::Error;
use std::io::{self};
use std::time::Instant;

type Star = fn() -> Result<(), Box<dyn Error + 'static>>;

mod days;

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
        1 => {
            first_star = days::day1::first_star;
            second_star = days::day1::second_star;
        }
        2 => {
            first_star = days::day2::first_star;
            second_star = days::day2::second_star;
        }
        _ => {
            println!("No day found matching input, exiting...");
            return;
        }
    }

    let now = Instant::now();
    match first_star() {
        Err(x) => {
            println!("Error: {:?}", x);
        }
        _ => {
            println!("First star: Success!");
        }
    }

    match second_star() {
        Err(x) => {
            println!("Error {:?}", x);
        }
        _ => {
            println!("Second star: Success!");
        }
    }

    let end = now.elapsed();
    println!(
        "Duration: {}sec {}millisec",
        end.as_secs(),
        end.subsec_millis()
    );
}
