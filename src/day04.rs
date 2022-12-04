pub fn solve() {
    let input = std::fs::read_to_string("input/day04.prod.txt").unwrap();
    let result = input
        .lines()
        .map(|line| {
            line.split_terminator(',').map(|range| {
                let range = range.split_terminator('-').collect::<Vec<&str>>();
                let start = range[0].parse::<usize>().unwrap();
                let end = range[1].parse::<usize>().unwrap();
                (start, end)
            })
        })
        .map(|ranges| {
            let (start_one, end_one) = ranges.clone().nth(0).unwrap();
            let (start_two, end_two) = ranges.clone().nth(1).unwrap();
            (start_one >= start_two && end_one <= end_two)
                || (start_two >= start_one && end_two <= end_one)
        })
        .filter(|&x| x)
        .count();
    println!("Day 04 - Part One: {}", result);
}
