use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve() {
    let priorities = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let input = std::fs::read_to_string("input/day03.prod.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    let result_one: usize = lines
        .iter()
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

    let result_two: usize = lines
        .chunks(3)
        .flat_map(|group| {
            group
                .iter()
                .map(|member| HashSet::from_iter(member.chars()))
                .fold(None, |acc, set: HashSet<char>| match acc {
                    None => Some(set),
                    Some(acc) => Some(acc.intersection(&set).copied().collect()),
                })
        })
        .map(|intersection| {
            priorities
                .iter()
                .position(|&item| intersection.contains(&item))
                .unwrap()
                + 1
        })
        .sum();

    println!("Day 03 - Part One: {}", result_one);
    println!("Day 03 - Part Two: {}", result_two);
}
