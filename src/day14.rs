use std::collections::HashSet;

pub fn solve() {
    let input = std::fs::read_to_string("input/day14.prod.txt").unwrap();
    let mut blocked_init: HashSet<(isize, isize)> = HashSet::new();
    let mut bottom_obstacle: isize = 0;
    input
        .lines()
        .map(|line| line.split_terminator(" -> ").collect::<Vec<&str>>())
        .map(|vec| {
            vec.iter()
                .map(|s| s.split_terminator(","))
                .map(|mut pair| {
                    (
                        pair.next().unwrap().parse::<isize>().unwrap(),
                        pair.next().unwrap().parse::<isize>().unwrap(),
                    )
                })
                .collect::<Vec<(isize, isize)>>()
        })
        .for_each(|line| {
            for i in 0..line.len() - 1 {
                let start = line[i];
                let end = line[i + 1];
                bottom_obstacle = bottom_obstacle.max(start.1.max(end.1));
                if start.0 == end.0 {
                    for y in start.1.min(end.1)..=start.1.max(end.1) {
                        blocked_init.insert((start.0, y));
                    }
                } else {
                    for x in start.0.min(end.0)..=start.0.max(end.0) {
                        blocked_init.insert((x, start.1));
                    }
                }
            }
        });

    let mut units_at_rest_one = 0;
    let mut simulation_end = false;
    let mut blocked = blocked_init.clone();
    loop {
        let mut sand: (isize, isize) = (500, 0);
        loop {
            if !blocked.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !blocked.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.0 -= 1;
                sand.1 += 1;
            } else if !blocked.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                units_at_rest_one += 1;
                blocked.insert(sand);
                break;
            }
            if sand.1 > bottom_obstacle {
                simulation_end = true;
                break;
            }
        }
        if simulation_end {
            break;
        }
    }

    let mut units_at_rest_two = 0;
    let bottom = bottom_obstacle + 2;
    simulation_end = false;
    blocked = blocked_init.clone();
    loop {
        let mut sand: (isize, isize) = (500, 0);
        loop {
            if sand.1 == bottom - 1 {
                units_at_rest_two += 1;
                blocked.insert(sand);
                break;
            } else if !blocked.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !blocked.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.0 -= 1;
                sand.1 += 1;
            } else if !blocked.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                units_at_rest_two += 1;
                blocked.insert(sand);
                if sand == (500, 0) {
                    simulation_end = true;
                }
                break;
            }
        }
        if simulation_end {
            break;
        }
    }

    println!("Day 14 - Part One: {}", units_at_rest_one);
    println!("Day 14 - Part Two: {}", units_at_rest_two);
}
