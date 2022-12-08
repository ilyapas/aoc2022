use std::cmp::max;

fn check_above(digits: &Vec<isize>, index: usize, width: usize) -> (isize, isize) {
    let mut tallest: isize = -1;
    let mut view: isize = 0;
    let mut max_view_found = false;
    let row = index / width;
    for i in 1..=row {
        let above = index - i * width;
        tallest = max(tallest, digits[above]);
        if !max_view_found {
            view += 1;
            if digits[above] >= digits[index] {
                max_view_found = true;
            }
        }
    }
    (tallest, view)
}

fn check_below(digits: &Vec<isize>, index: usize, width: usize) -> (isize, isize) {
    let mut tallest: isize = -1;
    let mut view: isize = 0;
    let mut max_view_found = false;
    let row = index / width;
    let height = digits.len() / width;
    for i in 1..(height - row) {
        let below = index + i * width;
        tallest = max(tallest, digits[below]);
        if !max_view_found {
            view += 1;
            if digits[below] >= digits[index] {
                max_view_found = true;
            }
        }
    }
    (tallest, view)
}

fn check_left(digits: &Vec<isize>, index: usize, width: usize) -> (isize, isize) {
    let mut tallest: isize = -1;
    let mut view: isize = 0;
    let mut max_view_found = false;
    let col = index % width;
    for i in 1..=col {
        let left = index - i;
        tallest = max(tallest, digits[left]);
        if !max_view_found {
            view += 1;
            if digits[left] >= digits[index] {
                max_view_found = true;
            }
        }
    }
    (tallest, view)
}

fn check_right(digits: &Vec<isize>, index: usize, width: usize) -> (isize, isize) {
    let mut tallest: isize = -1;
    let mut view: isize = 0;
    let mut max_view_found = false;
    let col = index % width;
    for i in 1..(width - col) {
        let right = index + i;
        tallest = max(tallest, digits[right]);
        if !max_view_found {
            view += 1;
            if digits[right] >= digits[index] {
                max_view_found = true;
            }
        }
    }
    (tallest, view)
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
    let mut max_scenic_score: isize = 0;
    for (i, d) in digits.iter().enumerate() {
        let (tallest_above, view_above) = check_above(&digits, i, width);
        let (tallest_below, view_below) = check_below(&digits, i, width);
        let (tallest_left, view_left) = check_left(&digits, i, width);
        let (tallest_right, view_right) = check_right(&digits, i, width);
        if d > &tallest_above || d > &tallest_below || d > &tallest_left || d > &tallest_right {
            visible += 1;
        }
        max_scenic_score = max(
            max_scenic_score,
            view_above * view_below * view_left * view_right,
        );
    }
    println!("Day 08 - Part One: {}", visible);
    println!("Day 08 - Part Two: {}", max_scenic_score);
}
