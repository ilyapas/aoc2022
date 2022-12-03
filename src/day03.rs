use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve() {
    let priorities = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let input = std::fs::read_to_string("input/day03.prod.txt").unwrap();
    let result: usize = input
        .lines()
        .map(|line| {
            let (first, last) = line.split_at(line.len() / 2);
            let first: HashSet<char> = HashSet::from_iter(first.chars());
            let last: HashSet<char> = HashSet::from_iter(last.chars());
            let intersection = first.intersection(&last).collect::<Vec<&char>>()[0];
            priorities
                .iter()
                .position(|&item| item == *intersection)
                .unwrap()
                + 1
        })
        .sum();
    println!("Day 03 - Part One: {}", result);
}
