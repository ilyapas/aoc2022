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

    let mut result = [0; 2];

    for (i, max_shapes_landed) in [2022, 1000000000000].iter().enumerate() {
        let mut covered: HashSet<(isize, isize)> = HashSet::new();
        let mut surface = [0; 7];
        let mut cache: Vec<(usize, usize, [isize; 7])> = Vec::new();
        let mut highest_rows: Vec<isize> = Vec::new();
        let mut shape_idx = 0;
        let mut jet_idx = 0;
        let mut highest_row = -1;
        let mut shapes_landed = 0;
        let mut cycle_length = 0;
        let mut height_offset = 0;

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
                            shapes_landed += 1;
                            covered.extend(shape.iter());
                            highest_row = covered.iter().map(|s| s.1).max().unwrap_or(highest_row);

                            for i in 0..surface.len() {
                                surface[i] = covered
                                    .iter()
                                    .filter(|s| s.0 == i as isize)
                                    .map(|s| highest_row - s.1)
                                    .min()
                                    .unwrap_or(highest_row + 1);
                            }

                            if cycle_length == 0 {
                                if cache.contains(&(shape_idx, jet_idx, surface)) {
                                    let cycle_start = cache
                                        .iter()
                                        .position(|c| {
                                            c.0 == shape_idx && c.1 == jet_idx && c.2 == surface
                                        })
                                        .unwrap();
                                    cycle_length = shapes_landed - cycle_start - 1;
                                    let height_per_cycle = highest_row - highest_rows[cycle_start];
                                    let shapes_remaining = max_shapes_landed - shapes_landed;
                                    let cycles_remaining = shapes_remaining / cycle_length;
                                    shapes_landed += cycle_length * cycles_remaining;
                                    height_offset = height_per_cycle * cycles_remaining as isize;
                                } else {
                                    cache.push((shape_idx, jet_idx, surface));
                                    highest_rows.push(highest_row);
                                }
                            }

                            shape_idx = (shape_idx + 1) % shapes.len();
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
                    let jet_direction = jets[jet_idx];
                    jet_idx = (jet_idx + 1) % jets.len();
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

            if shapes_landed == *max_shapes_landed {
                break;
            }
        }
        result[i] = highest_row + height_offset + 1;
    }
    println!("Day 17 - Part One: {}", result[0]);
    println!("Day 17 - Part Two: {}", result[1]);
}
