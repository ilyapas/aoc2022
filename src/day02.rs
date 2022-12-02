use std::collections::HashMap;

pub fn solve() {
    let scoring_map = HashMap::from([
        ('A', HashMap::from([('X', 4), ('Y', 8), ('Z', 3)])),
        ('B', HashMap::from([('X', 1), ('Y', 5), ('Z', 9)])),
        ('C', HashMap::from([('X', 7), ('Y', 2), ('Z', 6)])),
    ]);

    let input = std::fs::read_to_string("input/day02.prod.txt").unwrap();
    let result: usize = input
        .lines()
        .map(|line| {
            let theirs = line.chars().nth(0).unwrap();
            let ours = line.chars().nth(2).unwrap();
            scoring_map.get(&theirs).unwrap().get(&ours).unwrap()
        })
        .sum();
    println!("Day 02 - Part One: {}", result);
}
