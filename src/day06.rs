use std::collections::HashSet;
use std::collections::VecDeque;

fn find_marker(tag: &str, input: &str, marker_size: usize) {
    let mut queue: VecDeque<char> = VecDeque::new();
    for (i, c) in input.chars().enumerate() {
        if queue.len() < marker_size {
            queue.push_back(c);
        } else {
            queue.pop_front();
            queue.push_back(c);
            let set = queue.iter().collect::<HashSet<&char>>();
            if set.len() == marker_size {
                println!("Day 06 - Part {}: {}", tag, i + 1);
                break;
            }
        }
    }
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day06.prod.txt").unwrap();
    find_marker("One", &input, 4);
    find_marker("Two", &input, 14);
}
