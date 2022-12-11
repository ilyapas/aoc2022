use std::{collections::VecDeque, str::FromStr};

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: fn(usize, usize) -> usize,
    operand: usize,
    test_divisor: usize,
    test_true_target: usize,
    test_false_target: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = VecDeque::new();
        let mut operation: fn(usize, usize) -> usize = |y, _| y;
        let mut operand = 0;
        let mut test_divisor = 0;
        let mut test_true_target = 0;
        let mut test_false_target = 0;
        for line in s.lines() {
            match line
                .trim()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .as_slice()
            {
                ["Monkey", _] => (),
                ["Starting", "items:", items_str @ ..] => {
                    items = items_str
                        .iter()
                        .map(|x| x.replace(",", "").parse::<usize>().unwrap())
                        .collect::<VecDeque<usize>>();
                }
                ["Operation:", "new", "=", "old", operator_str, operand_str] => {
                    operand = match operand_str.parse::<usize>() {
                        Ok(x) => x,
                        Err(_) => 0,
                    };
                    operation = match operator_str {
                        &"+" => |y, x| y + x,
                        &"*" => |y, x| y * x,
                        _ => |y, _| y,
                    }
                }
                ["Test:", "divisible", "by", divisor] => {
                    test_divisor = divisor.parse::<usize>().unwrap();
                }
                ["If", "true:", "throw", "to", "monkey", target_str] => {
                    test_true_target = target_str.parse::<usize>().unwrap();
                }
                ["If", "false:", "throw", "to", "monkey", target_str] => {
                    test_false_target = target_str.parse::<usize>().unwrap();
                }
                _ => (),
            }
        }
        Ok(Monkey {
            items,
            operation,
            operand,
            test_divisor,
            test_true_target,
            test_false_target,
        })
    }
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day11.prod.txt").unwrap();
    let text_blocks = input.split("\n\n").collect::<Vec<&str>>();
    let monkeys_start = text_blocks
        .iter()
        .map(|x| x.parse::<Monkey>().unwrap())
        .collect::<Vec<Monkey>>();
    let modulo = monkeys_start
        .iter()
        .map(|x| x.test_divisor)
        .product::<usize>();
    let mut result: Vec<usize> = vec![];

    for (part, rounds) in [20, 10000].iter().enumerate() {
        let mut monkeys = monkeys_start.clone();
        let mut inspections: Vec<usize> = vec![0; monkeys.len()];
        for _ in 0..*rounds {
            for i in 0..monkeys.len() {
                while monkeys[i].items.len() > 0 {
                    let mut item = monkeys[i].items.pop_front().unwrap();
                    inspections[i] += 1;
                    let operand = match monkeys[i].operand {
                        0 => item,
                        _ => monkeys[i].operand,
                    };
                    item = (monkeys[i].operation)(item, operand);
                    if part == 0 {
                        item = item / 3;
                    } else {
                        item = item % modulo;
                    }
                    if item % monkeys[i].test_divisor == 0 {
                        let index = monkeys[i].test_true_target;
                        monkeys[index].items.push_back(item);
                    } else {
                        let index = monkeys[i].test_false_target;
                        monkeys[index].items.push_back(item);
                    }
                }
            }
        }
        inspections.sort_by(|a, b| b.cmp(a));
        result.push(inspections.iter().take(2).product::<usize>());
    }
    println!("Day 11 - Part One: {}", result[0]);
    println!("Day 11 - Part Two: {}", result[1]);
}
