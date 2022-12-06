use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

fn get_input() -> String {
    fs::read_to_string(Path::new("./input/day6.input"))
        .expect("Something went wrong with the input")
        .trim_end()
        .to_string()
}

fn get_marker(stream: Vec<char>, len: usize) -> usize {
    for (index, input) in stream.windows(len).enumerate() {
        let test: HashSet<char> = input.iter().cloned().collect();
        if test.len() == len {
            return index + len;
        }
    }
    0
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let datastream: Vec<_> = get_input().chars().collect();

    println!(
        "first start-of-packet marker appears on {}",
        get_marker(datastream, 4)
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let datastream: Vec<_> = get_input().chars().collect();

    println!(
        "first start-of-message marker appears on {}",
        get_marker(datastream, 14)
    );

    Ok(())
}
