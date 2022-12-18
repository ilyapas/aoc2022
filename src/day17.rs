use std::collections::{HashSet, VecDeque};

pub fn solve() {
    let input = std::fs::read_to_string("input/day17.prod.txt").unwrap();

    let mut jets: VecDeque<(isize, isize)> = VecDeque::new();
    for c in input.chars() {
        match c {
            '<' => jets.push_back((-1, 0)),
            '>' => jets.push_back((1, 0)),
            _ => (),
        }
    }

    let shapes: Vec<Vec<(isize, isize)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let mut covered: HashSet<(isize, isize)> = HashSet::new();
    let mut shape_idx = 0;
    let mut jet_idx = 0;
    let mut highest_row = -1;
    let mut shapes_landed = 0;
    let max_shapes_landed: i64 = 2022;

    loop {
        let mut shape = shapes[shape_idx].clone();
        let start = (2, highest_row + 4);
        shape
            .iter_mut()
            .for_each(|s| *s = (s.0 + start.0, s.1 + start.1));

        let mut counter = 0;
        loop {
            let mut new_position = shape.clone();
            let mut valid_move = true;
            if counter % 2 != 0 {
                // move down
                new_position.iter_mut().for_each(|s| *s = (s.0, s.1 - 1));
                for s in new_position.iter() {
                    if covered.contains(s) || s.1 < 0 {
                        valid_move = false;
                        covered.extend(shape.iter());
                        highest_row = highest_row.max(covered.iter().map(|s| s.1).max().unwrap());
                        shapes_landed += 1;
                        shape_idx = (shape_idx + 1) % shapes.len();
                        // display_covered(&covered, highest_row);
                        break;
                    }
                }
                if valid_move {
                    shape = new_position;
                } else {
                    break;
                }
            } else {
                // move left/right
                let jet_direction = jets[jet_idx % jets.len()];
                jet_idx += 1;
                new_position
                    .iter_mut()
                    .for_each(|s| *s = (s.0 + jet_direction.0, s.1));
                for s in new_position.iter() {
                    if covered.contains(s) {
                        valid_move = false;
                    }
                    if s.0 < 0 || s.0 > 6 {
                        valid_move = false;
                    }
                }
                if valid_move {
                    shape = new_position;
                }
            }
            counter += 1;
        }

        if shapes_landed % 1000 == 0 {
            println!(
                "Shapes landed: {} / {} ({}%)",
                shapes_landed,
                max_shapes_landed,
                shapes_landed * 100 / max_shapes_landed
            );
        }

        if shapes_landed == max_shapes_landed {
            break;
        }
    }
    println!("Day 17 - Part One: {}", highest_row + 1);
}

fn display_covered(covered: &HashSet<(isize, isize)>, highest_row: isize) {
    for y in 0..=highest_row {
        for x in 0..=6 {
            if covered.contains(&(x, highest_row - y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}
