use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::path::Path;

fn get_input() -> String {
    fs::read_to_string(Path::new("./input/day6.input"))
        .expect("Something went wrong with the input")
        .trim_end()
        .to_string()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let datastream = get_input();
    let mut marker = datastream.chars().take(3).collect::<VecDeque<char>>();

    for (index, value) in datastream.chars().skip(3).enumerate() {
        marker.push_back(value);
        let test: HashSet<char> = marker.iter().cloned().collect();
        if test.len() == 4 {
            println!("first start-of-packet marker appears on {}", index + 4);
            break;
        }
        marker.pop_front();
    }
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
