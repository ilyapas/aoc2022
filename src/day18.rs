use std::collections::{HashMap, HashSet, VecDeque};

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

    let mut queue: VecDeque<(isize, isize, isize)> = VecDeque::new();
    let mut visited: HashSet<(isize, isize, isize)> = HashSet::new();
    nodes.iter().for_each(|n| queue.push_back(*n));

    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        visited.insert(current);

        for step in &[
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let next = (current.0 + step.0, current.1 + step.1, current.2 + step.2);
            if nodes.contains(&next) {
                edges.entry(current).or_insert(HashSet::new()).insert(next);
                if !visited.contains(&next) {
                    queue.push_back(next);
                }
            }
        }
    }

    let mut total_surface = 0;
    nodes.iter().for_each(|n| {
        let edge_vec = edges.entry(*n).or_insert(HashSet::new());
        total_surface += 6 - edge_vec.len();
    });

    println!("Day 18 - Part One: {}", total_surface);
}
