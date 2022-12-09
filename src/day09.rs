use std::cmp::max;
use std::collections::HashSet;

fn distance(first: (isize, isize), second: (isize, isize)) -> isize {
    max((first.0 - second.0).abs(), (first.1 - second.1).abs())
}

fn direction(first: (isize, isize), second: (isize, isize)) -> (isize, isize) {
    let mut result: (isize, isize) = (0, 0);
    if first.0 != second.0 {
        result.0 = (first.0 - second.0) / (first.0 - second.0).abs();
    }
    if first.1 != second.1 {
        result.1 = (first.1 - second.1) / (first.1 - second.1).abs();
    }
    result
}

fn result(input: &str, snake_size: usize) -> usize {
    let mut snake: Vec<(isize, isize)> = vec![(0, 0); snake_size];
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    for line in input.lines() {
        let command = line.split_whitespace().collect::<Vec<&str>>();
        let (head_direction, steps_str) = match command.as_slice() {
            ["R", steps] => ((1, 0), steps),
            ["L", steps] => ((-1, 0), steps),
            ["U", steps] => ((0, 1), steps),
            ["D", steps] => ((0, -1), steps),
            _ => ((0, 0), &"0"),
        };
        let steps = steps_str.parse::<usize>().unwrap();
        for _ in 0..steps {
            let mut dir = head_direction;
            snake[0].0 += dir.0;
            snake[0].1 += dir.1;
            for i in 0..(snake.len() - 1) {
                if distance(snake[i], snake[i + 1]) > 1 {
                    dir = direction(snake[i], snake[i + 1]);
                    snake[i + 1].0 += dir.0;
                    snake[i + 1].1 += dir.1;
                }
            }
            visited.insert(*snake.last().unwrap());
        }
    }
    visited.len()
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day09.prod.txt").unwrap();
    println!("Day 09 - Part One: {}", result(&input, 2));
    println!("Day 09 - Part Two: {}", result(&input, 10));
}
