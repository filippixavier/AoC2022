use std::error::Error;
use std::fs;
use std::path::Path;

fn get_input() -> Vec<((usize, usize), (usize, usize))> {
    fs::read_to_string(Path::new("./input/day4.input"))
        .expect("Something went wrong with the input")
        .trim()
        .lines()
        .map(|line| {
            let range: Vec<(usize, usize)> = line
                .split(',')
                .map(|range| {
                    let tmp: Vec<usize> = range
                        .split('-')
                        .map(|input| input.parse::<usize>())
                        .collect::<Result<_, _>>()
                        .unwrap();
                    (tmp[0], tmp[1])
                })
                .collect();
            (range[0], range[1])
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let groups = get_input();
    let mut count = 0;
    for (elf_1, elf_2) in groups {
        if elf_1.0 <= elf_2.0 && elf_1.1 >= elf_2.1 || elf_2.0 <= elf_1.0 && elf_2.1 >= elf_1.1 {
            count += 1;
        }
    }
    println!("Number of overlapping cleaning operations: {}", count);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let groups = get_input();
    let mut count = 0;
    for (elf_1, elf_2) in groups {
        if !(elf_1.1 < elf_2.0 || elf_1.0 > elf_2.1) {
            count += 1;
        }
    }

    println!("Number of colliding cleaning operations: {}", count);

    Ok(())
}
