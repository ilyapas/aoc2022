pub fn solve() {
    let input = std::fs::read_to_string("input/day20.prod.txt").unwrap();
    println!("Day 20 - Part One: {}", mix(&input, 1, 1));
    println!("Day 20 - Part Two: {}", mix(&input, 811589153, 10));
}

fn mix(input: &str, key: isize, runs: usize) -> isize {
    let encrypted = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .map(|i| i * key)
        .collect::<Vec<isize>>();
    let size = encrypted.len() as isize;
    let mut positions: Vec<isize> = (0..size).collect();
    let mut decrypted: Vec<isize> = encrypted.clone();

    for _ in 0..runs {
        for i in 0..size {
            let item_position = positions.iter().position(|x| *x == i).unwrap() as isize;
            shift(&mut decrypted, item_position, encrypted[i as usize]);
            shift(&mut positions, item_position, encrypted[i as usize]);
        }
    }

    let index_zero = decrypted.iter().position(|x| *x == 0).unwrap() as usize;
    [1000, 2000, 3000].iter().fold(0, |acc, x| {
        acc + decrypted[(index_zero + x) % size as usize]
    })
}

fn shift(vec: &mut Vec<isize>, index: isize, shift: isize) {
    let value = vec[index as usize];
    let new_index = (index + shift).rem_euclid(vec.len() as isize - 1);
    vec.remove(index as usize);
    vec.insert(new_index as usize, value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_right() {
        let mut vec = vec![1, 2, 3, 4, 5];
        shift(&mut vec, 2, 1);
        assert_eq!(vec, vec![1, 2, 4, 3, 5]);
    }

    #[test]
    fn test_shift_left() {
        let mut vec = vec![1, 2, 3, 4, 5];
        shift(&mut vec, 2, -1);
        assert_eq!(vec, vec![1, 3, 2, 4, 5]);
    }

    #[test]
    fn test_shift_right_wrap() {
        let mut vec = vec![1, 2, 3, 4, 5];
        shift(&mut vec, 3, 3);
        assert_eq!(vec, vec![1, 2, 4, 3, 5]);
    }

    #[test]
    fn test_shift_left_wrap() {
        let mut vec = vec![1, 2, 3, 4, 5];
        shift(&mut vec, 2, -2);
        assert_eq!(vec, vec![3, 1, 2, 4, 5]);
    }
}
