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

    let mut result = [0; 2];
    for (i, wrap) in [wrap_one, wrap_two].iter().enumerate() {
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
                for _ in 0..steps {
                    let (dr, dc) = DIR[d];
                    match map
                        .get((r + dr) as usize)
                        .and_then(|row| row.get((c + dc) as usize))
                        .unwrap_or(&' ')
                    {
                        '#' => break,
                        '.' => (r, c) = (r + dr, c + dc),
                        ' ' => {
                            let (temp_r, temp_c, temp_d) = wrap(&map, r, c, d);
                            if map[temp_r as usize][temp_c as usize] == '#' {
                                break;
                            }
                            (r, c, d) = (temp_r, temp_c, temp_d);
                        }
                        _ => {}
                    }
                }
            }
        }
        result[i] = 1000 * (r + 1) + 4 * (c + 1) + d as isize;
    }

    println!("Day 22 - Part One: {}", result[0]);
    println!("Day 22 - Part Two: {}", result[1]);
}

fn wrap_one(map: &Vec<Vec<char>>, r: isize, c: isize, dir: usize) -> (isize, isize, usize) {
    let (dr, dc) = DIR[dir];
    let (mut nr, mut nc) = (r, c);
    while *map
        .get((nr - dr) as usize)
        .and_then(|row| row.get((nc - dc) as usize))
        .unwrap_or(&' ')
        != ' '
    {
        (nr, nc) = (nr - dr, nc - dc);
    }
    (nr, nc, dir)
}

fn wrap_two(_: &Vec<Vec<char>>, r: isize, c: isize, dir: usize) -> (isize, isize, usize) {
    let (dr, dc) = (r % 50, c % 50);
    let (nr, nc, ndir) = match (r / 50, c / 50, dir) {
        (0, 1, 0) => (dr, 100, 0),
        (0, 1, 1) => (100, 100 + dc, 1),
        (0, 1, 2) => (149 - dr, 0, 0),
        (0, 1, 3) => (150 + dc, 0, 0),
        (0, 2, 0) => (149 - dr, 99, 2),
        (0, 2, 1) => (50 + dc, 99, 2),
        (0, 2, 2) => (dr, 99, 2),
        (0, 2, 3) => (199, dc, 3),
        (1, 1, 0) => (49, 100 + dr, 3),
        (1, 1, 1) => (100, 50 + dc, 1),
        (1, 1, 2) => (100, dr, 1),
        (1, 1, 3) => (49, 50 + dc, 3),
        (2, 0, 0) => (100 + dr, 50, 0),
        (2, 0, 1) => (150, dc, 1),
        (2, 0, 2) => (49 - dr, 50, 0),
        (2, 0, 3) => (50 + dc, 50, 0),
        (2, 1, 0) => (49 - dr, 149, 2),
        (2, 1, 1) => (150 + dc, 49, 2),
        (2, 1, 2) => (100 + dr, 49, 2),
        (2, 1, 3) => (99, 50 + dc, 3),
        (3, 0, 0) => (149, 50 + dr, 3),
        (3, 0, 1) => (0, 100 + dc, 1),
        (3, 0, 2) => (0, 50 + dr, 1),
        (3, 0, 3) => (149, dc, 3),
        _ => unreachable!(),
    };
    (nr, nc, ndir)
}
