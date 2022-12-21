use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("input/day21.prod.txt").unwrap();
    let mut neighbors: HashMap<String, Vec<String>> = HashMap::new();
    let mut numbers: HashMap<String, isize> = HashMap::new();
    let mut operators: HashMap<String, String> = HashMap::new();
    let mut humn_path: Vec<String> = vec![];
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

    while !numbers.contains_key("root") {
        dfs(
            "root".to_string(),
            &neighbors,
            &mut numbers,
            &operators,
            &mut humn_path,
        );
    }

    let root_neighbors = neighbors.get("root").unwrap();
    let root_left = root_neighbors[0].clone();
    let root_right = root_neighbors[1].clone();
    let mut oppisite = root_left.clone();
    if humn_path.contains(&root_left) {
        oppisite = root_right.clone();
    }

    // 150 = cczh / lfqf = x / 4 -> cczh = 150 * 4 = 600
    // 600 = sllz + lgvd = 4 + x -> lgvd = 600 - 4 = 596
    // 596 = ljgn * ptdq = 2 * x -> ptdq = 596 / 2 = 298
    // 298 = humn - dvpt = x - 3 -> humn = 298 + 3 = 301

    let mut res = numbers[&oppisite];
    for i in 1..humn_path.len() - 1 {
        let current = humn_path[i].clone();
        let neighbors = neighbors.get(&current).unwrap();
        let mut other = (0, "".to_string());
        for (i, n) in neighbors.iter().enumerate() {
            if !humn_path.contains(n) {
                other = (i, n.clone());
            }
        }
        let operator = operators.get(&current).unwrap();
        if operator == "-" {
            if other.0 == 0 {
                res = numbers[&other.1] - res;
            } else {
                res = res + numbers[&other.1];
            }
        } else if operator == "/" {
            if other.0 == 0 {
                res = numbers[&other.1] / res;
            } else {
                res = res * numbers[&other.1];
            }
        } else if operator == "+" {
            res = res - numbers[&other.1];
        } else if operator == "*" {
            res = res / numbers[&other.1];
        }
    }

    println!("Day 21 - Part One: {}", numbers.get("root").unwrap());
    println!("Day 21 - Part Two: {}", res);
}

fn dfs(
    current: String,
    neighbors: &HashMap<String, Vec<String>>,
    numbers: &mut HashMap<String, isize>,
    operators: &HashMap<String, String>,
    humn_path: &mut Vec<String>,
) {
    if humn_path.len() == 0 || humn_path.last().unwrap() != "humn" {
        humn_path.push(current.clone());
    }
    if neighbors.contains_key(&current) {
        let neighbor = neighbors.get(&current).unwrap();
        let mut calculate = true;
        neighbor.iter().for_each(|n| {
            if !numbers.contains_key(n) {
                calculate = false;
                dfs(n.to_string(), neighbors, numbers, operators, humn_path);
            }
            if n == "humn" && humn_path.last().unwrap() != "humn" {
                humn_path.push(n.to_string());
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
        }
    }
    if humn_path.len() > 0 && humn_path.last().unwrap() != "humn" {
        humn_path.pop();
    }
}
