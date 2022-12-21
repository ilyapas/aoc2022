use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("input/day21.prod.txt").unwrap();
    let mut neighbors: HashMap<String, Vec<String>> = HashMap::new();
    let mut numbers: HashMap<String, isize> = HashMap::new();
    let mut operators: HashMap<String, String> = HashMap::new();
    input.lines().for_each(|line| {
        let line = line.replace(":", "");
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [item, a, op, b] => {
                neighbors.insert(item.to_string(), vec![a.to_string(), b.to_string()]);
                operators.insert(item.to_string(), op.to_string());
            }
            [item, number] => {
                let number = number.parse::<isize>().unwrap();
                numbers.insert(item.to_string(), number);
            }
            _ => (),
        }
    });
    println!("{:?}", neighbors);
    println!("{:?}", numbers);
    println!("{:?}", operators);

    while !numbers.contains_key("root") {
        dfs("root".to_string(), &neighbors, &mut numbers, &operators);
    }

    println!("Day 21 - Part One: {}", numbers.get("root").unwrap());
}

fn dfs(
    current: String,
    neighbors: &HashMap<String, Vec<String>>,
    numbers: &mut HashMap<String, isize>,
    operators: &HashMap<String, String>,
) {
    if neighbors.contains_key(&current) {
        let neighbor = neighbors.get(&current).unwrap();
        let mut calculate = true;
        neighbor.iter().for_each(|n| {
            if !numbers.contains_key(n) {
                calculate = false;
                dfs(n.to_string(), neighbors, numbers, operators);
            }
        });
        if calculate {
            let a = *numbers.get(&neighbor[0]).unwrap();
            let b = *numbers.get(&neighbor[1]).unwrap();
            let op = operators.get(&current).unwrap();
            let result = match op.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => panic!("Unknown operator"),
            };
            numbers.insert(current.clone(), result);
            println!("{} = {} {} {} = {}", current, a, op, b, result);
        }
    }
}
