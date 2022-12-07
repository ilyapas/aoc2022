use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("input/day07.prod.txt").unwrap();

    let mut dirs = HashMap::<String, usize>::new();
    let mut current_dir = Vec::<String>::new();

    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["$", "cd", "/"] => {
                current_dir.push(String::from("/"));
            }
            ["$", "cd", ".."] => {
                current_dir.pop();
            }
            ["$", "cd", dir] => {
                current_dir.push(format!("{}/", dir));
            }
            ["$", "ls"] => (),
            ["dir", _] => (),
            [size, _] => {
                let file_size = size.parse::<usize>().unwrap();
                for (i, _) in current_dir.iter().enumerate() {
                    let subpath = current_dir[..(i + 1)].join("");
                    match dirs.get(&subpath) {
                        Some(size) => dirs.insert(subpath, size + file_size),
                        None => dirs.insert(subpath, file_size),
                    };
                }
            }
            _ => (),
        }
    }

    let result_one: usize = dirs.values().filter(|size| *size < &100000).sum();

    let total_used_space = dirs.get("/").unwrap();
    let required_additional_space = 30000000 - (70000000 - total_used_space);

    let result_two: usize = *dirs
        .values()
        .filter(|size| *size >= &required_additional_space)
        .min()
        .unwrap();

    println!("Day 07 - Part One: {}", result_one);
    println!("Day 07 - Part Two: {:?}", result_two);
}
