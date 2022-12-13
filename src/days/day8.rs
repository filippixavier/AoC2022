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

    for (line_no, line_view) in trees.iter().enumerate().take(max_line).skip(1) {
        let mut highest_l = trees[line_no][0];
        let mut highest_r = trees[line_no].last().unwrap();

        for col in 1..max_col {
            let right_col = max_col - col;
            if highest_l < line_view[col] {
                visible_index.insert(format!("{}.{}", line_no, col));
                highest_l = line_view[col];
            }
            if highest_r < &line_view[right_col] {
                visible_index.insert(format!("{}.{}", line_no, right_col));
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
    let trees = get_input();
    let max_line = trees.len();
    let max_col = trees[0].len();
    let scenics_scores = trees
        .iter()
        .enumerate()
        .map(|(line_no, line)| {
            line.iter()
                .enumerate()
                .map(|(col_no, initial_height)| {
                    let mut left = 0;
                    let mut right = 0;
                    let mut up = 0;
                    let mut down = 0;
                    for li in (0..line_no).rev() {
                        up += 1;
                        if trees[li][col_no] >= *initial_height {
                            break;
                        }
                    }
                    for line in trees.iter().take(max_line).skip(line_no + 1) {
                        down += 1;
                        if line[col_no] >= *initial_height {
                            break;
                        }
                    }
                    for col in (0..col_no).rev() {
                        left += 1;
                        if trees[line_no][col] >= *initial_height {
                            break;
                        }
                    }
                    for col in (col_no + 1)..max_col {
                        right += 1;
                        if trees[line_no][col] >= *initial_height {
                            break;
                        }
                    }
                    up * down * left * right
                })
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();
    let most_scenic = scenics_scores.iter().flatten().max().unwrap_or(&0);

    println!("Most scenic view have a score of {}", most_scenic);
    Ok(())
}
