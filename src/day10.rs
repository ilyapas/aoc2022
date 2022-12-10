pub fn solve() {
    let input = std::fs::read_to_string("input/day10.prod.txt").unwrap();
    let mut x: Vec<isize> = vec![1];
    let mut signal: isize = 0;
    let mut pixels: Vec<char> = vec![];
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
    x.iter().enumerate().for_each(|(i, sprite)| {
        if [*sprite - 1, *sprite, *sprite + 1].contains(&(i as isize % 40)) {
            pixels.push('#');
        } else {
            pixels.push('.');
        }
    });
    println!("Day 10 - Part One: {}", signal);
    println!("Day 10 - Part Two:");
    for i in 0..(pixels.len() / 40) {
        println!(
            "{}",
            pixels[i * 40..(i + 1) * 40].iter().collect::<String>()
        );
    }
}
