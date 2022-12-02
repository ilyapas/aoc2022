pub fn solve() {
    let input = std::fs::read_to_string("input/day01.prod.txt").unwrap();
    let mut sum = 0;
    let mut sums: Vec<i32> = Vec::new();
    for line in input.lines() {
        if line == "" {
            sums.push(sum);
            sum = 0;
        } else {
            let value = line.parse::<i32>().unwrap();
            sum += value;
        }
    }

    sums.sort();
    sums.reverse();

    println!("Day 01 - Part One: {}", sums[0]);
    println!("Day 01 - Part Two: {}", sums[..3].iter().sum::<i32>());
}
