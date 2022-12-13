use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn neighbors((row, col): (isize, isize), width: isize, height: isize) -> Vec<(isize, isize)> {
    let mut result: Vec<(isize, isize)> = Vec::new();
    if row > 0 {
        result.push((row - 1, col));
    }
    if row < height - 1 {
        result.push((row + 1, col));
    }
    if col > 0 {
        result.push((row, col - 1));
    }
    if col < width - 1 {
        result.push((row, col + 1));
    }
    result
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day12.prod.txt").unwrap();
    let mut heights: HashMap<(isize, isize), isize> = HashMap::new();
    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;
    let mut start_part_one: (isize, isize) = (0, 0);
    let mut possible_starts: Vec<(isize, isize)> = Vec::new();
    let mut end: (isize, isize) = (0, 0);
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                start_part_one = (row as isize, col as isize);
                possible_starts.push(start_part_one);
                heights.insert((row as isize, col as isize), 'a' as isize);
            } else if c == 'E' {
                end = (row as isize, col as isize);
                heights.insert((row as isize, col as isize), 'z' as isize);
            } else {
                heights.insert((row as isize, col as isize), c as isize);
                if c == 'a' {
                    possible_starts.push((row as isize, col as isize));
                }
            }
        }
    }

    let mut shortest_path = std::isize::MAX;

    let mut bfs_queue: VecDeque<(isize, isize)> = VecDeque::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut prev: HashMap<(isize, isize), (isize, isize)> = HashMap::new();

    bfs_queue.push_back(end);

    while bfs_queue.len() > 0 {
        let current = bfs_queue.pop_front().unwrap();
        let neighbors = neighbors(current, width, height);
        for neighbor in neighbors {
            if heights[&current] - heights[&neighbor] <= 1 {
                if !visited.contains(&neighbor) {
                    bfs_queue.push_back(neighbor);
                    visited.insert(neighbor);
                    prev.insert(neighbor, current);
                }
            }
        }
        visited.insert(current);
    }

    for start in possible_starts {
        let mut path: Vec<(isize, isize)> = vec![end];
        let mut current = start;
        let mut path_valid = true;
        while current != end {
            let prev_option = prev.get(&current);
            if prev_option.is_none() {
                path_valid = false;
                break;
            }
            path.push(*prev_option.unwrap());
            current = *prev_option.unwrap();
        }

        if start == start_part_one {
            println!("Day 12 - Part One: {}", path.len() - 1);
        }
        if path_valid {
            shortest_path = shortest_path.min(path.len() as isize - 1);
        }
    }

    println!("Day 12 - Part Two: {}", shortest_path);
}
