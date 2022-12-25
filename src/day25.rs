const DIGITS: [char; 5] = ['=', '-', '0', '1', '2'];

pub fn solve() {
    let input = std::fs::read_to_string("input/day25.prod.txt").unwrap();
    let sum = input.lines().map(|line| from_snafu(line)).sum::<isize>();
    println!("Day 25: {}", to_snafu(sum));
}

fn from_snafu(input: &str) -> isize {
    let mut result = 0;
    let base: isize = 5;
    for (i, c) in input.chars().rev().enumerate() {
        let exp = i as u32;
        let val = DIGITS.iter().position(|&x| x == c).unwrap() as isize;
        result += (val - 2) * base.pow(exp);
    }
    result
}

fn to_snafu(input: isize) -> String {
    let mut result = String::new();
    let base: isize = 5;
    let mut n = input;
    while n != 0 {
        n += 2;
        result.push(DIGITS[(n % base) as usize]);
        n /= base;
    }
    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUES: [(&str, isize); 15] = [
        ("1", 1),
        ("2", 2),
        ("1=", 3),
        ("1-", 4),
        ("10", 5),
        ("11", 6),
        ("12", 7),
        ("2=", 8),
        ("2-", 9),
        ("20", 10),
        ("1=0", 15),
        ("1-0", 20),
        ("1=11-2", 2022),
        ("1-0---0", 12345),
        ("1121-1110-1=0", 314159265),
    ];

    #[test]
    fn test_from_snafu() {
        for (input, expected) in VALUES.iter() {
            assert_eq!(from_snafu(input), *expected);
        }
    }

    #[test]
    fn test_to_snafu() {
        for (expected, input) in VALUES.iter() {
            println!("{} {}", expected, input);
            assert_eq!(to_snafu(*input), *expected);
        }
    }
}
