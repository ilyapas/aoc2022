pub fn solve() {
    let input = std::fs::read_to_string("input/day04.prod.txt").unwrap();
    let (result_one, result_two) = input
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
            let part_one = (start_one >= start_two && end_one <= end_two)
                || (start_two >= start_one && end_two <= end_one);
            let part_two = part_one
                || (start_one >= start_two && start_one <= end_two)
                || (end_one >= start_two && end_one <= end_two);
            (part_one, part_two)
        })
        .fold((0, 0), |mut acc, result| {
            if result.0 {
                acc.0 += 1;
            }
            if result.1 {
                acc.1 += 1;
            }
            acc
        });
    println!("Day 04 - Part One: {}", result_one);
    println!("Day 04 - Part Two: {}", result_two);
}
