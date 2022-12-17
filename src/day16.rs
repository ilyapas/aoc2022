use regex::Regex;
use std::collections::{HashMap, VecDeque};

pub fn solve() {
    let input = std::fs::read_to_string("input/day16.prod.txt").unwrap();
    let mut nodes: Vec<String> = vec![];
    let mut flows: Vec<isize> = vec![];
    let mut tunnels: Vec<Vec<String>> = vec![];
    let re =
        Regex::new(r"^Valve (.+?) has flow rate=(\d+); tunnel[s]* lead[s]* to valve[s]* (.+)$")
            .unwrap();
    input
        .lines()
        .map(|s| re.captures(s).unwrap())
        .for_each(|c| {
            let name = c.get(1).unwrap().as_str().to_string();
            let flow = c.get(2).unwrap().as_str().parse::<isize>().unwrap();
            let tunnel_vec = c
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            nodes.push(name.clone());
            flows.push(flow);
            tunnels.push(tunnel_vec);
        });

    let mut valves: Vec<usize> = vec![];
    for i in 0..nodes.len() {
        if flows[i] > 0 {
            valves.push(i);
        }
    }

    let mut distances: Vec<Vec<isize>> = vec![vec![1000; nodes.len()]; nodes.len()];
    for i in 0..nodes.len() {
        distances[i][i] = 0;
        for neighbor in &tunnels[i] {
            let j = nodes.iter().position(|n| n == neighbor).unwrap();
            distances[i][j] = 1;
        }
    }

    // Floyd-Warshall
    for k in 0..nodes.len() {
        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
            }
        }
    }

    let best = search(&nodes, &flows, &valves, &distances, &30);
    let result_one = best.values().max().unwrap();

    let best = search(&nodes, &flows, &valves, &distances, &26);
    let mut result_vec: Vec<isize> = vec![];
    let mut counter: i64 = 0;
    best.clone().iter().for_each(|(k1, v1)| {
        best.iter().for_each(|(k2, v2)| {
            let mut intersection = false;
            for i in 0..k1.1.len() {
                if k2.1.contains(&k1.1[i]) {
                    intersection = true;
                    break;
                }
            }
            if !intersection {
                result_vec.push(*v1 + *v2);
            }
            counter += 1;
            if counter % 10000000 == 0 {
                println!(
                    "{} / {} ({}%)",
                    counter,
                    best.len() * best.len(),
                    counter as f64 / (best.len() * best.len()) as f64 * 100.0
                );
            }
        });
    });
    let result_two = result_vec.iter().max().unwrap();

    println!("Day 16 - Part One: {}", result_one);
    println!("Day 16 - Part Two: {}", result_two);
}

fn search(
    nodes: &Vec<String>,
    flows: &Vec<isize>,
    valves: &Vec<usize>,
    distances: &Vec<Vec<isize>>,
    minutes: &isize,
) -> HashMap<(usize, Vec<usize>, isize), isize> {
    let mut queue: VecDeque<(usize, Vec<usize>, isize, isize)> = VecDeque::new();
    let mut best: HashMap<(usize, Vec<usize>, isize), isize> = HashMap::new();
    let aa = nodes.iter().position(|n| n == "AA").unwrap();

    queue.push_back((aa, vec![], *minutes, 0));
    while queue.len() > 0 {
        let (current, open, time, pressure) = queue.pop_front().unwrap();
        if !open.contains(&current) && time >= 1 {
            let p = flows[current] * (time - 1);
            if pressure + p > *best.entry((current, open.clone(), time - 1)).or_insert(-1) {
                let mut new_open = open.clone();
                new_open.push(current);
                best.insert((current, new_open.clone(), time - 1), pressure + p);
                queue.push_back((current, new_open, time - 1, pressure + p));
            }
        }

        for valve in valves.iter() {
            if current != *valve {
                let time_to_move = distances[current][*valve];
                if time_to_move <= time {
                    if pressure
                        > *best
                            .entry((*valve, open.clone(), time - time_to_move))
                            .or_insert(-1)
                    {
                        best.insert((*valve, open.clone(), time - time_to_move), pressure);
                        queue.push_back((*valve, open.clone(), time - time_to_move, pressure));
                    }
                }
            }
        }
    }
    best
}
