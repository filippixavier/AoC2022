use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;

fn get_input() -> Vec<Vec<usize>> {
    fs::read_to_string(Path::new("./input/day8.input"))
        .expect("Something went wrong with the input")
        .trim_end()
        .lines()
        .map(|line| {
            line.chars()
                .map(|elem| elem.to_string().parse::<usize>())
                .collect::<Result<_, _>>()
                .unwrap()
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let trees = get_input();
    let mut visible_index = HashSet::new();
    let border = trees.len() * 2 + trees[0].len() * 2 - 4;
    let max_col = trees.len() - 1;
    let max_line = trees[0].len() - 1;

    for line in 1..max_line {
        let mut highest_l = trees[line][0];
        let mut highest_r = trees[line].last().unwrap();

        let line_view = &trees[line];

        for col in 1..max_col {
            let right_col = max_col - col;
            if highest_l < line_view[col] {
                visible_index.insert(format!("{}.{}", line, col));
                highest_l = line_view[col];
            }
            if highest_r < &line_view[right_col] {
                visible_index.insert(format!("{}.{}", line, right_col));
                highest_r = &line_view[right_col];
            }
        }
    }

    for col in 1..max_col {
        let mut highest_u = trees[0][col];
        let mut highest_d = trees[max_line][col];

        for line in 1..max_line {
            let down_line = max_line - line;
            if highest_u < trees[line][col] {
                visible_index.insert(format!("{}.{}", line, col));
                highest_u = trees[line][col];
            }
            if highest_d < trees[down_line][col] {
                visible_index.insert(format!("{}.{}", down_line, col));
                highest_d = trees[down_line][col];
            }
        }
    }

    println!("{} trees are visible", visible_index.len() + border);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
