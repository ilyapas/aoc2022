use std::collections::HashSet;

pub fn solve() {
    let input = std::fs::read_to_string("input/day09.prod.txt").unwrap();

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut visited = HashSet::new();
    visited.insert(tail);

    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["R", steps] => {
                let steps = steps.parse::<isize>().unwrap();
                for _ in 0..steps {
                    if head.0 > tail.0 {
                        tail = head;
                        visited.insert(tail);
                    }
                    head.0 += 1;
                }
            }
            ["L", steps] => {
                let steps = steps.parse::<isize>().unwrap();
                for _ in 0..steps {
                    if head.0 < tail.0 {
                        tail = head;
                        visited.insert(tail);
                    }
                    head.0 -= 1;
                }
            }
            ["U", steps] => {
                let steps = steps.parse::<isize>().unwrap();
                for _ in 0..steps {
                    if head.1 > tail.1 {
                        tail = head;
                        visited.insert(tail);
                    }
                    head.1 += 1;
                }
            }
            ["D", steps] => {
                let steps = steps.parse::<isize>().unwrap();
                for _ in 0..steps {
                    if head.1 < tail.1 {
                        tail = head;
                        visited.insert(tail);
                    }
                    head.1 -= 1;
                }
            }
            _ => (),
        }
    }

    println!("Day 09 - Part One: {}", visited.len());
}
