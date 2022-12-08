use std::cmp::max;

fn largest_above(digits: &Vec<isize>, index: usize, width: usize) -> isize {
    let mut result: isize = -1;
    let row = index / width;
    for i in 1..=row {
        let above = index - i * width;
        result = max(result, digits[above]);
    }
    result
}

fn largest_below(digits: &Vec<isize>, index: usize, width: usize) -> isize {
    let mut result: isize = -1;
    let row = index / width;
    let height = digits.len() / width;
    for i in 1..(height - row) {
        let below = index + i * width;
        result = max(result, digits[below]);
    }
    result
}

fn largest_left(digits: &Vec<isize>, index: usize, width: usize) -> isize {
    let mut result: isize = -1;
    let col = index % width;
    for i in 1..=col {
        let left = index - i;
        result = max(result, digits[left]);
    }
    result
}

fn largest_right(digits: &Vec<isize>, index: usize, width: usize) -> isize {
    let mut result: isize = -1;
    let col = index % width;
    for i in 1..(width - col) {
        let right = index + i;
        result = max(result, digits[right]);
    }
    result
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day08.prod.txt").unwrap();
    let width = input.lines().next().unwrap().len();
    let digits = input
        .replace("\n", "")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect::<Vec<isize>>();

    let mut visible: isize = 0;
    for (i, d) in digits.iter().enumerate() {
        let above = largest_above(&digits, i, width);
        let below = largest_below(&digits, i, width);
        let left = largest_left(&digits, i, width);
        let right = largest_right(&digits, i, width);
        if d > &above || d > &below || d > &left || d > &right {
            visible += 1;
        }
    }
    println!("Day 08 - Part One: {}", visible);
}
