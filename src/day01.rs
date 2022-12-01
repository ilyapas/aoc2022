pub mod puzzle1 {
    pub fn solve() {
        let input = include_str!("../input/day01_puzzle1.txt");
        let mut sum = 0;
        let mut max_sum: i32 = 0;
        for line in input.lines() {
            if line == "" {
                sum = 0;
                continue;
            }
            let value = line.parse::<i32>().unwrap();
            sum += value;
            if sum > max_sum {
                max_sum = sum;
                println!("{} {}", sum, max_sum);
            }
        }
        println!("Day 01 - Puzzle 1: {}", max_sum);
    }
}
