use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    time: isize,
    materials: [usize; 4],
    robots: [usize; 4],
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
        materials: [0, 0, 0, 0],
        robots: [1, 0, 0, 0],
    };

    let mut result = [0, 1];
    let time_limit = [24, 32];
    for part in 0..result.len() {
        for (i, blueprint) in blueprints.iter().enumerate() {
            if part == 1 && i > 2 {
                break;
            }

            let req: [[usize; 3]; 4] = [
                [blueprint[0], 0, 0],
                [blueprint[1], 0, 0],
                [blueprint[2], blueprint[3], 0],
                [blueprint[4], 0, blueprint[5]],
            ];

            let max_req: [usize; 3] = [
                req.iter().map(|r| r[0]).max().unwrap(),
                req.iter().map(|r| r[1]).max().unwrap(),
                req.iter().map(|r| r[2]).max().unwrap(),
            ];

            let mut cache: HashMap<State, usize> = HashMap::new();
            let max_geode = dfs(
                State {
                    time: time_limit[part],
                    ..init_state.clone()
                },
                &mut cache,
                req,
                max_req,
            );

            if part == 0 {
                result[part] += (i + 1) * max_geode;
            } else {
                result[part] *= max_geode;
            }
        }
    }

    println!("Day 19 - Part One: {}", result[0]);
    println!("Day 19 - Part Two: {}", result[1]);
}

fn dfs(
    state: State,
    cache: &mut HashMap<State, usize>,
    req: [[usize; 3]; 4],
    max_req: [usize; 3],
) -> usize {
    if state.time == 0 {
        return state.materials[3];
    }
    if cache.contains_key(&state) {
        return *cache.get(&state).unwrap();
    }

    let mut max_geode = state.materials[3] + state.robots[3] * state.time as usize;

    for (i, recipe) in req.iter().enumerate() {
        // if we produce more than we can spend in a round, skip
        if i != 3 && state.robots[i] >= max_req[i] {
            continue;
        }

        // Determine how much time we need to wait until we have enough materials for a new robot
        let mut wait_time: isize = 0;
        let mut can_build = true;
        for (j, mat_req) in recipe.iter().enumerate() {
            if *mat_req > 0 {
                if state.robots[j] == 0 {
                    can_build = false;
                    break;
                }
                wait_time = std::cmp::max(
                    wait_time,
                    ((*mat_req as f64 - state.materials[j] as f64) / state.robots[j] as f64).ceil()
                        as isize,
                );
            }
        }

        if !can_build {
            continue;
        }

        let mut next = state.clone();
        next.time = state.time - wait_time as isize - 1;
        if next.time <= 0 {
            continue;
        }

        // Accumulate the materials we have produced during wait time
        for j in 0..4 {
            next.materials[j] += next.robots[j] * (wait_time as usize + 1);
        }

        // Subtract the materials we need for the new robot
        for (j, mat_req) in recipe.iter().enumerate() {
            next.materials[j] -= mat_req;
        }

        // Throw away the excess materials
        for j in 0..3 {
            next.materials[j] = std::cmp::min(next.materials[j], max_req[j] * next.time as usize);
        }

        next.robots[i] += 1;
        max_geode = std::cmp::max(max_geode, dfs(next, cache, req, max_req));
    }

    cache.insert(state, max_geode);
    max_geode
}
