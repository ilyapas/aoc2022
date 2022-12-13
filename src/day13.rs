use itertools::{EitherOrBoth::*, Itertools};
use serde_json::{Result, Value};

fn parse_line(packet: &str) -> Result<Value> {
    let v: Value = serde_json::from_str(packet)?;
    Ok(v)
}

fn compare(left: &Value, right: &Value) -> Option<bool> {
    if left.is_array() && right.is_array() {
        let left_items = left.as_array().unwrap();
        let right_items = right.as_array().unwrap();
        for pair in left_items.iter().zip_longest(right_items.iter()) {
            match pair {
                Both(left_item, right_item) => match compare(left_item, right_item) {
                    Some(result) => {
                        return Some(result);
                    }
                    None => (),
                },
                Left(_) => {
                    return Some(false);
                }
                Right(_) => {
                    return Some(true);
                }
            }
        }
    } else if left.is_array() && right.is_number() {
        return compare(left, &Value::Array(vec![right.clone()]));
    } else if left.is_number() && right.is_array() {
        return compare(&Value::Array(vec![left.clone()]), right);
    } else if left.is_number() && right.is_number() {
        let left_number = left.as_u64().unwrap();
        let right_number = right.as_u64().unwrap();
        if left_number < right_number {
            return Some(true);
        } else if left_number > right_number {
            return Some(false);
        } else {
            return None;
        }
    }
    None
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day13.prod.txt").unwrap();

    let text_blocks = input.split("\n\n").collect::<Vec<&str>>();
    let mut result_one = 0;
    for (i, block) in text_blocks.iter().enumerate() {
        let left = parse_line(block.lines().nth(0).unwrap()).unwrap();
        let right = parse_line(block.lines().nth(1).unwrap()).unwrap();
        let result = compare(&left, &right);
        if Some(true) == result {
            result_one += i + 1;
        }
    }

    let mut input_vec = input.lines().filter(|x| x.len() > 0).collect::<Vec<&str>>();
    input_vec.push(&"[[2]]");
    input_vec.push(&"[[6]]");
    input_vec.sort_by(|a, b| {
        let left = parse_line(a).unwrap();
        let right = parse_line(b).unwrap();
        match compare(&left, &right) {
            Some(true) => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Greater,
        }
    });
    let result_two = (input_vec.iter().position(|&x| x == "[[6]]").unwrap() + 1)
        * (input_vec.iter().position(|&x| x == "[[2]]").unwrap() + 1);

    println!("Day 13 - Part One: {}", result_one);
    println!("Day 13 - Part Two: {}", result_two);
}
