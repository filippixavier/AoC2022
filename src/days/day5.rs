use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::path::Path;

fn get_input() -> (Vec<VecDeque<char>>, Vec<[usize; 3]>) {
    let input = fs::read_to_string(Path::new("./input/day5.input"))
        .expect("Something went wrong with the input");
    let blocks = input.trim_end().split("\r\n\r\n").collect::<Vec<_>>();

    let max_index = blocks[0]
        .trim_end()
        .lines()
        .last()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); max_index];

    for line in blocks[0].lines() {
        for (index, id) in line.chars().skip(1).step_by(4).enumerate() {
            if id.is_alphabetic() {
                stacks[index].push_front(id);
            }
            if id.is_numeric() {
                break;
            }
        }
    }

    let instructions: Vec<[usize; 3]> = blocks[1]
        .lines()
        .map(|line| {
            let mut inst = [0usize; 3];
            let tmp = line.split(' ').collect::<Vec<_>>();
            inst[0] = tmp[1].parse().unwrap();
            inst[1] = tmp[3].parse().unwrap();
            inst[2] = tmp[5].parse().unwrap();
            inst
        })
        .collect();
    (stacks, instructions)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut stacks, instructions) = get_input();

    for step in instructions {
        for _ in 0..step[0] {
            let id = stacks[step[1] - 1].pop_back().unwrap();
            stacks[step[2] - 1].push_back(id);
        }
    }

    let code = stacks.iter().fold(String::new(), |mut acc, ids| {
        if let Some(id) = ids.back() {
            acc.push(*id);
        }
        acc
    });

    println!("Crate code is: {}", code);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut stacks, instructions) = get_input();

    for step in instructions {
        let from = &stacks[step[1] - 1];
        let mut slice = from
            .iter()
            .skip(from.len() - step[0])
            .cloned()
            .collect::<VecDeque<_>>();
        stacks[step[1] - 1] = from.iter().cloned().take(from.len() - step[0]).collect();
        stacks[step[2] - 1].append(&mut slice);
    }

    let code = stacks.iter().fold(String::new(), |mut acc, ids| {
        if let Some(id) = ids.back() {
            acc.push(*id);
        }
        acc
    });

    println!("Crate code for 9001 is: {}", code);
    Ok(())
}
