use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve() {
    let input = std::fs::read_to_string("input/day23.prod.txt").unwrap();
    let mut elves: Vec<(isize, isize)> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.push((x as isize, y as isize));
            }
        }
    }

    let dx_dy: HashMap<&str, (isize, isize)> = HashMap::from([
        ("N", (0, -1)),
        ("NW", (-1, -1)),
        ("W", (-1, 0)),
        ("SW", (-1, 1)),
        ("S", (0, 1)),
        ("SE", (1, 1)),
        ("E", (1, 0)),
        ("NE", (1, -1)),
    ]);

    let checked_dirs: HashMap<&str, [&str; 3]> = HashMap::from([
        ("N", ["NW", "N", "NE"]),
        ("W", ["SW", "W", "NW"]),
        ("S", ["SE", "S", "SW"]),
        ("E", ["NE", "E", "SE"]),
    ]);

    let mut options: Vec<VecDeque<&str>> = vec![VecDeque::from(["N", "S", "W", "E"]); elves.len()];

    for _ in 0..10 {
        let mut new_elves: Vec<(usize, (isize, isize))> = vec![];
        let mut new_positions: HashSet<(isize, isize)> = HashSet::new();
        let mut blocked_moves: HashSet<(isize, isize)> = HashSet::new();
        for (i, (x, y)) in elves.iter().enumerate() {
            let mut neighbor_found = false;
            for (dx, dy) in dx_dy.values() {
                if elves.contains(&(x + dx, y + dy)) {
                    neighbor_found = true;
                    break;
                }
            }
            if neighbor_found {
                for dir in options[i].iter() {
                    let mut move_allowed = true;
                    let checks = checked_dirs.get(dir).unwrap();
                    for c in checks {
                        let (dx, dy) = dx_dy.get(c).unwrap();
                        if elves.contains(&(x + dx, y + dy)) {
                            move_allowed = false;
                            break;
                        }
                    }
                    if move_allowed {
                        let (dx, dy) = dx_dy.get(dir).unwrap();
                        new_elves.push((i, (x + dx, y + dy)));
                        if !new_positions.contains(&(x + dx, y + dy)) {
                            new_positions.insert((x + dx, y + dy));
                        } else {
                            blocked_moves.insert((x + dx, y + dy));
                        }
                        break;
                    }
                }
            }
            options[i].rotate_left(1);
        }
        new_elves.iter().for_each(|(i, (x, y))| {
            if !blocked_moves.contains(&(*x, *y)) {
                elves[*i] = (*x, *y);
            }
        });
    }
    println!("Day 23 - Part One: {}", analyze(&elves));
}

fn analyze(elves: &Vec<(isize, isize)>) -> usize {
    let mut empty = 0;
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for (x, y) in elves {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
                empty += 1;
            }
        }
        println!();
    }
    println!();
    empty
}
