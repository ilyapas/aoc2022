use std::collections::HashSet;
use std::collections::VecDeque;

pub fn solve() {
    let input = std::fs::read_to_string("input/day06.prod.txt").unwrap();
    let mut queue: VecDeque<char> = VecDeque::new();
    for (i, c) in input.chars().enumerate() {
        if queue.len() < 4 {
            queue.push_back(c);
        } else {
            queue.pop_front();
            queue.push_back(c);
            let set = queue.iter().collect::<HashSet<&char>>();
            if set.len() == 4 {
                println!("Day 06 - Part One: {}", i + 1);
                break;
            }
        }
    }
}
