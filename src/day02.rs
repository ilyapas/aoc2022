use std::collections::HashMap;

pub fn solve() {
    let scoring_map_one = HashMap::from([
        ('A', HashMap::from([('X', 4), ('Y', 8), ('Z', 3)])),
        ('B', HashMap::from([('X', 1), ('Y', 5), ('Z', 9)])),
        ('C', HashMap::from([('X', 7), ('Y', 2), ('Z', 6)])),
    ]);

    let scoring_map_two = HashMap::from([
        ('A', HashMap::from([('X', 3), ('Y', 4), ('Z', 8)])),
        ('B', HashMap::from([('X', 1), ('Y', 5), ('Z', 9)])),
        ('C', HashMap::from([('X', 2), ('Y', 6), ('Z', 7)])),
    ]);

    let input = std::fs::read_to_string("input/day02.prod.txt").unwrap();
    let result = input
        .lines()
        .map(|line| {
            let theirs = line.chars().nth(0).unwrap();
            let ours = line.chars().nth(2).unwrap();
            [
                scoring_map_one.get(&theirs).unwrap().get(&ours).unwrap(),
                scoring_map_two.get(&theirs).unwrap().get(&ours).unwrap(),
            ]
        })
        .fold([0, 0], |mut acc, score| {
            acc[0] += score[0];
            acc[1] += score[1];
            acc
        });

    println!("Day 02 - Part One: {}", result[0]);
    println!("Day 02 - Part Two: {}", result[1]);
}
