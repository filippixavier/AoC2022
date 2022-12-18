use std::collections::HashSet;
use std::convert::TryInto;
use std::error::Error;
use std::fs;
use std::path::Path;

type Coordinate = [usize; 3];

fn get_input() -> Vec<Coordinate> {
    fs::read_to_string(Path::new("./input/day18.input"))
        .expect("Something went wrong with the input")
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse::<usize>)
                .collect::<Result<Vec<usize>, _>>()
                .unwrap()
                .try_into()
                .unwrap()
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let droplets = get_input();
    let mut max = (0, 0, 0);

    println!("{}", droplets.len());

    for drop in droplets.iter() {
        max.0 = max.0.max(drop[0]);
        max.1 = max.1.max(drop[1]);
        max.2 = max.2.max(drop[2]);
    }

    let cube: HashSet<Coordinate> = droplets.into_iter().collect();

    println!("{}", cube.len());

    let mut faces = 0;

    for x in 0..=max.0 {
        for y in 0..=max.1 {
            for z in 0..=max.2 {
                if cube.contains(&[x, y, z]) {
                    let neighbors = vec![
                        (x.checked_sub(1), Some(y), Some(z)),
                        (Some(x + 1), Some(y), Some(z)),
                        (Some(x), y.checked_sub(1), Some(z)),
                        (Some(x), Some(y + 1), Some(z)),
                        (Some(x), Some(y), z.checked_sub(1)),
                        (Some(x), Some(y), Some(z + 1)),
                    ];
                    for neighbor in neighbors {
                        if let (Some(x), Some(y), Some(z)) = neighbor {
                            if !cube.contains(&[x, y, z]) {
                                faces += 1;
                            }
                        } else {
                            faces += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", faces);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
