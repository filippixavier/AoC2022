use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Directory {
    parents: HashSet<String>,
    childs: HashSet<String>,
    size: usize,
}

fn get_input() -> Vec<Vec<String>> {
    fs::read_to_string(Path::new("./input/day7.input"))
        .expect("Something went wrong with the input")
        .trim_end()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|elem| elem.to_string())
                .collect()
        })
        .collect()
}

fn compute_size(directories: &mut HashMap<String, Directory>, current: String) -> usize {
    let current_dir = directories.get(&current).unwrap();
    let childs = current_dir.childs.iter().cloned().collect::<Vec<_>>();
    let mut total_size = current_dir.size;

    for child in childs {
        total_size += compute_size(directories, child);
    }

    if let Some(cur) = directories.get_mut(&current) {
        cur.size = total_size;
    }

    total_size
}

fn explore() -> Vec<usize> {
    let root = String::from("/");
    let instructions = get_input();
    let mut path: Vec<String> = vec![root.clone()];
    let mut folder_name: String = String::new();

    let mut directories: HashMap<String, Directory> = HashMap::new();

    for instruction in instructions {
        let ins_type = instruction[0].clone();
        if ins_type == "$" {
            if instruction[1] == "cd" {
                let mut going_down = false;
                let previous_folder_name = folder_name;
                let target = instruction[2].clone();
                if target == "/" {
                    path = vec![root.clone()];
                } else if target == ".." {
                    path.pop();
                    if path.is_empty() {
                        path = vec![root.clone()];
                    }
                } else {
                    going_down = true;
                    let mut folder_name_vec = path
                        .last()
                        .unwrap()
                        .split('.')
                        .map(|elem| elem.to_string())
                        .collect::<Vec<String>>();
                    folder_name_vec.push(target);
                    path.push(folder_name_vec.join("."));
                }

                folder_name = path.last().unwrap().clone();
                if !directories.contains_key(&folder_name) {
                    directories.insert(
                        folder_name.clone(),
                        Directory {
                            parents: HashSet::new(),
                            childs: HashSet::new(),
                            size: 0,
                        },
                    );
                }
                if going_down {
                    if let Some(dir) = directories.get_mut(&folder_name) {
                        dir.parents.insert(previous_folder_name.clone());
                    }
                }
            }
        } else if ins_type == "dir" {
            let mut child_name = folder_name.split('.').collect::<Vec<_>>();
            child_name.push(&instruction[1]);
            let child_folder_name = child_name.join(".");
            if let Some(dir) = directories.get_mut(&folder_name) {
                dir.childs.insert(child_folder_name);
            }
        } else {
            let size: usize = ins_type.parse().unwrap_or_default();
            if let Some(dir) = directories.get_mut(&folder_name) {
                dir.size += size;
            }
        }
    }

    compute_size(&mut directories, String::from("/"));

    directories.values().map(|dir| dir.size).collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let sizes = explore();
    println!(
        "Removable content size: {}",
        sizes.iter().fold(0, |acc, size| {
            if size <= &100_000 {
                acc + size
            } else {
                acc
            }
        })
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut sizes = explore();
    sizes.sort();
    let max = 70_000_000;
    let required = 30_000_000;
    let available = max - sizes.iter().max().unwrap_or(&0);

    for size in sizes {
        if available + size >= required {
            println!("Total size of deleted directory: {}", size);
            break;
        }
    }

    Ok(())
}
