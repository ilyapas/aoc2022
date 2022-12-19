use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Quantity {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    time: usize,
    materials: Quantity,
    robots: Quantity,
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day19.prod.txt").unwrap();
    let blueprints = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<usize>())
                .filter(|o| o.is_ok())
                .map(|o| o.unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let init_state = State {
        time: 0,
        materials: Quantity {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        robots: Quantity {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
    };

    let mut result = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        println!("{:?}", blueprint);
        let req_ore = Quantity {
            ore: blueprint[0],
            ..Default::default()
        };
        let req_clay = Quantity {
            ore: blueprint[1],
            ..Default::default()
        };
        let req_obsidian = Quantity {
            ore: blueprint[2],
            clay: blueprint[3],
            ..Default::default()
        };
        let req_geode = Quantity {
            ore: blueprint[4],
            obsidian: blueprint[5],
            ..Default::default()
        };

        let mut queue: VecDeque<State> = VecDeque::new();
        let mut visited: HashSet<State> = HashSet::new();
        queue.push_back(init_state.clone());

        let mut counter = 0;
        while queue.len() > 0 {
            let current = queue.pop_front().unwrap();
            visited.insert(current.clone());

            counter += 1;
            if counter % 10000000 == 0 {
                println!("{:?}", current);
            }

            let mut next = current.clone();

            next.time += 1;
            if next.time > 24 {
                continue;
            }

            next.materials.ore += current.robots.ore;
            next.materials.clay += current.robots.clay;
            next.materials.obsidian += current.robots.obsidian;
            next.materials.geode += current.robots.geode;

            if !visited.contains(&next) {
                queue.push_back(next.clone());
            }

            if current.materials.ore >= req_geode.ore
                && current.materials.obsidian >= req_geode.obsidian
            {
                let mut n = next.clone();
                n.materials.ore -= req_geode.ore;
                n.materials.obsidian -= req_geode.obsidian;
                n.robots.geode += 1;
                if !visited.contains(&n) {
                    queue.push_back(n.clone());
                }
                continue;
            }

            if current.materials.ore >= req_obsidian.ore
                && current.materials.clay >= req_obsidian.clay
                && current.robots.obsidian < req_geode.obsidian
            {
                let mut n = next.clone();
                n.materials.ore -= req_obsidian.ore;
                n.materials.clay -= req_obsidian.clay;
                n.robots.obsidian += 1;
                if !visited.contains(&n) {
                    queue.push_back(n.clone());
                }
                continue;
            }

            if current.materials.ore >= req_clay.ore && current.robots.clay < req_obsidian.clay {
                let mut n = next.clone();
                n.materials.ore -= req_clay.ore;
                n.robots.clay += 1;
                if !visited.contains(&n) {
                    queue.push_back(n.clone());
                }
            }

            if current.materials.ore >= req_ore.ore
                && current.robots.ore
                    < *[req_clay.ore, req_obsidian.ore, req_geode.ore]
                        .iter()
                        .max()
                        .unwrap()
            {
                let mut n = next.clone();
                n.materials.ore -= req_ore.ore;
                n.robots.ore += 1;
                if !visited.contains(&n) {
                    queue.push_back(n.clone());
                }
            }
        }

        result += (i + 1)
            * visited
                .iter()
                .filter(|s| s.time == 24)
                .max_by_key(|s| s.materials.geode)
                .unwrap()
                .materials
                .geode;
    }

    println!("Day 19 - Part One: {}", result);
}
