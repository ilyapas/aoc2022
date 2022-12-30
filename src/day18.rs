use std::collections::{HashMap, HashSet};

const DIR: [(isize, isize, isize); 6] = [
    (0, 1, 0),
    (0, -1, 0),
    (1, 0, 0),
    (-1, 0, 0),
    (0, 0, 1),
    (0, 0, -1),
];

pub fn solve() {
    let input = std::fs::read_to_string("input/day18.prod.txt").unwrap();
    let nodes = input
        .lines()
        .map(|l| {
            l.split_terminator(",")
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1], v[2]))
        .collect::<Vec<_>>();

    let mut edges: HashMap<(isize, isize, isize), HashSet<(isize, isize, isize)>> = HashMap::new();
    nodes.iter().for_each(|current| {
        for step in DIR.iter() {
            let next = (current.0 + step.0, current.1 + step.1, current.2 + step.2);
            if nodes.contains(&next) {
                edges.entry(*current).or_insert(HashSet::new()).insert(next);
            }
        }
    });

    let mut total_surface = 0;
    nodes.iter().for_each(|n| {
        let edge_vec = edges.entry(*n).or_insert(HashSet::new());
        total_surface += 6 - edge_vec.len();
    });

    let x_min = nodes.iter().map(|n| n.0).min().unwrap();
    let x_max = nodes.iter().map(|n| n.0).max().unwrap();
    let y_min = nodes.iter().map(|n| n.1).min().unwrap();
    let y_max = nodes.iter().map(|n| n.1).max().unwrap();
    let z_min = nodes.iter().map(|n| n.2).min().unwrap();
    let z_max = nodes.iter().map(|n| n.2).max().unwrap();

    let fill_start = (x_min - 1, y_min - 1, z_min - 1);
    let mut reachable_faces = HashSet::new();
    let mut queue = vec![fill_start];
    let mut visited = HashSet::new();
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        visited.insert(current);
        for step in DIR.iter() {
            let next = (current.0 + step.0, current.1 + step.1, current.2 + step.2);
            if next.0 < x_min - 1
                || next.0 > x_max + 1
                || next.1 < y_min - 1
                || next.1 > y_max + 1
                || next.2 < z_min - 1
                || next.2 > z_max + 1
            {
                continue;
            } else if nodes.contains(&next) {
                reachable_faces.insert((current, next));
            } else if !visited.contains(&next) {
                queue.push(next);
            }
        }
    }

    println!("Day 18 - Part One: {}", total_surface);
    println!("Day 18 - Part Two: {}", reachable_faces.len());
}
