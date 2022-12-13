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
    let mut start: (isize, isize) = (0, 0);
    let mut end: (isize, isize) = (0, 0);
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (row as isize, col as isize);
                heights.insert((row as isize, col as isize), 'a' as isize);
            } else if c == 'E' {
                end = (row as isize, col as isize);
                heights.insert((row as isize, col as isize), 'z' as isize);
            } else {
                heights.insert((row as isize, col as isize), c as isize);
            }
        }
    }

    let mut bfs_queue: VecDeque<(isize, isize)> = VecDeque::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut prev: HashMap<(isize, isize), (isize, isize)> = HashMap::new();

    bfs_queue.push_back(start);

    while bfs_queue.len() > 0 {
        let current = bfs_queue.pop_front().unwrap();
        let neighbors = neighbors(current, width, height);
        for neighbor in neighbors {
            if heights[&neighbor] - heights[&current] <= 1 {
                if neighbor == end {
                    prev.insert(neighbor, current);
                    break;
                } else if !visited.contains(&neighbor) {
                    bfs_queue.push_back(neighbor);
                    visited.insert(neighbor);
                    prev.insert(neighbor, current);
                }
            }
        }
        visited.insert(current);
    }

    let mut path: Vec<(isize, isize)> = vec![end];
    let mut current = end;
    while current != start {
        let prev = prev[&current];
        path.push(prev);
        current = prev;
    }

    println!("Day 12 - Part One: {}", path.len() - 1);
}
