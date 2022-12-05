use std::str::FromStr;

struct StackContainer {
    stacks: Vec<Vec<char>>,
}

impl FromStr for StackContainer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines().rev();
        let mut stacks = Vec::new();
        for _ in iter.next().unwrap().split_whitespace() {
            stacks.push(Vec::new());
        }
        for line in iter {
            for (i, c) in line.chars().enumerate() {
                if c == ' ' {
                    continue;
                }
                if i == 1 {
                    stacks[0].push(c);
                }
                if i > 1 && (i - 1) % 4 == 0 {
                    stacks[(i - 1) / 4].push(c);
                }
            }
        }
        Ok(StackContainer { stacks })
    }
}

struct MoveInstruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for MoveInstruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<&str>>();
        let amount = words[1].parse::<usize>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        Ok(MoveInstruction { amount, from, to })
    }
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day05.prod.txt").unwrap();
    let mut text_blocks = input.split_terminator("\n\n");

    let initial_stacks = text_blocks
        .next()
        .unwrap()
        .parse::<StackContainer>()
        .unwrap()
        .stacks;

    let instructions: Vec<MoveInstruction> = text_blocks
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse::<MoveInstruction>().unwrap())
        .collect();

    let mut stacks_one = initial_stacks.clone();
    let mut stacks_two = initial_stacks.clone();

    for instruction in instructions {
        let mut amount = instruction.amount;

        let length = stacks_two[instruction.from].len();
        let mut c = stacks_two[instruction.from]
            .drain(length - amount..)
            .collect();
        stacks_two[instruction.to].append(&mut c);

        while amount > 0 {
            let c = stacks_one[instruction.from].pop().unwrap();
            stacks_one[instruction.to].push(c);
            amount -= 1;
        }
    }

    let results = [stacks_one, stacks_two].map(|stacks| {
        stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    });

    println!("Day 05 - Part One: {}", results[0]);
    println!("Day 05 - Part Two: {}", results[1]);
}
