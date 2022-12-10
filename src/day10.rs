pub fn solve() {
    let input = std::fs::read_to_string("input/day10.prod.txt").unwrap();
    let mut x: Vec<isize> = vec![1];
    let mut signal: isize = 0;
    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["noop"] => x.push(*x.last().unwrap()),
            ["addx", number] => {
                let number = number.parse::<isize>().unwrap();
                x.push(*x.last().unwrap());
                x.push(*x.last().unwrap() + number);
            }
            _ => (),
        }
    }

    [20, 60, 100, 140, 180, 220].iter().for_each(|&i: &usize| {
        signal += i as isize * x[i - 1];
    });

    println!("Day 10 - Part One: {}", signal);
    // println!("{:#?}", x);
}
