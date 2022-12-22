const DIR: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn solve() {
    let input = std::fs::read_to_string("input/day22.prod.txt").unwrap();
    let parts = input.split("\n\n").collect::<Vec<_>>();

    let map = parts[0]
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut commands: Vec<String> = vec![];
    let mut temp_cmd: Vec<char> = vec![];
    for c in parts[1].chars() {
        if c >= '0' && c <= '9' {
            temp_cmd.push(c);
        } else if c == 'R' || c == 'L' {
            commands.push(temp_cmd.iter().collect());
            commands.push(c.to_string());
            temp_cmd.clear();
        }
    }
    commands.push(temp_cmd.iter().collect());

    let mut r = 0;
    let mut c = map[0].iter().position(|&c| c == '.').unwrap() as isize;
    let mut d = 0;
    for cmd in commands.iter() {
        if cmd == "R" {
            d = (d + 1) % 4;
        } else if cmd == "L" {
            d = (d + 3) % 4;
        } else {
            let steps = cmd.parse::<usize>().unwrap();
            let (dr, dc) = DIR[d];
            for _ in 0..steps {
                match map
                    .get((r + dr) as usize)
                    .and_then(|row| row.get((c + dc) as usize))
                    .unwrap_or(&' ')
                {
                    '#' => break,
                    '.' => (r, c) = (r + dr, c + dc),
                    ' ' => {
                        let mut temp_r = r;
                        let mut temp_c = c;
                        while map
                            .get((temp_r - dr) as usize)
                            .and_then(|row| row.get((temp_c - dc) as usize))
                            .unwrap_or(&' ')
                            != &' '
                        {
                            (temp_r, temp_c) = (temp_r - dr, temp_c - dc);
                        }
                        if map[temp_r as usize][temp_c as usize] == '#' {
                            break;
                        }
                        (r, c) = (temp_r, temp_c);
                    }
                    _ => {}
                }
            }
        }
    }

    let result_one = 1000 * (r + 1) + 4 * (c + 1) + d as isize;
    println!("Day 22 - Part One: {}", result_one);
}
