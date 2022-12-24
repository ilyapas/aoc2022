use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
struct State {
    bliz_pos: Vec<(usize, usize)>,
    free: Vec<Vec<bool>>,
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day24.prod.txt").unwrap();
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let width = map[0].len();
    let height = map.len();

    let dirs: HashMap<char, (isize, isize)> = HashMap::from([
        ('^', (0, -1)),
        ('<', (-1, 0)),
        ('>', (1, 0)),
        ('v', (0, 1)),
        ('_', (0, 0)),
    ]);

    let mut start: (usize, usize) = (0, 0);
    let mut finish: (usize, usize) = (0, 0);
    let mut bliz_dir: Vec<(isize, isize)> = vec![];
    let mut bliz_pos: Vec<(usize, usize)> = vec![];
    let mut free_init: Vec<Vec<bool>> = vec![vec![true; width]; height];
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if y == 0 && *c == '.' {
                start = (x, y);
                free_init[y][x] = false;
            }
            if y == map.len() - 1 && *c == '.' {
                finish = (x, y);
                free_init[y][x] = false;
            }
            if ['^', '<', '>', 'v'].contains(c) {
                bliz_pos.push((x, y));
                bliz_dir.push(dirs.get(c).unwrap().clone());
            }
            if *c == '#' {
                free_init[y][x] = false;
            }
        }
    }

    let mut states: Vec<State> = vec![];
    states.push(State {
        bliz_pos: bliz_pos.clone(),
        free: free_init.clone(),
    });

    let mut time = 0;
    let mut queue: HashSet<(usize, usize)> = HashSet::new();
    let mut targets = VecDeque::from([finish, start, finish]);
    let mut results: Vec<usize> = vec![];

    queue.insert(start);
    while !targets.is_empty() {
        time += 1;
        if time == states.len() {
            states.push(update(
                &bliz_dir,
                &states.last().unwrap().bliz_pos,
                &free_init,
            ));
        }
        let state = &states[time];

        let mut reachable: HashSet<(usize, usize)> = HashSet::new();
        for pos in queue.iter() {
            for (dx, dy) in dirs.values() {
                let nx = pos.0 as isize + dx;
                let ny = pos.1 as isize + dy;
                if nx >= 0
                    && nx < width as isize
                    && ny >= 0
                    && ny < height as isize
                    && (state.free[ny as usize][nx as usize]
                        || (nx as usize, ny as usize) == start
                        || (nx as usize, ny as usize) == finish)
                {
                    reachable.insert((nx as usize, ny as usize));
                }
            }
        }

        queue = reachable.clone();

        if reachable.contains(&targets[0]) {
            results.push(time);
            queue.clear();
            queue.insert(targets[0]);
            targets.pop_front();
        }
    }
    println!("Day 24 - Part One: {}", results[0]);
    println!("Day 24 - Part Two: {}", results[2]);
}

fn update(
    bliz_dir: &Vec<(isize, isize)>,
    bliz_pos: &Vec<(usize, usize)>,
    free_init: &Vec<Vec<bool>>,
) -> State {
    let mut state = State {
        bliz_pos: bliz_pos.clone(),
        free: free_init.clone(),
    };
    for (i, (x, y)) in bliz_pos.iter().enumerate() {
        let (dx, dy) = bliz_dir[i];
        let mut nx = *x as isize + dx;
        let mut ny = *y as isize + dy;
        if !free_init[ny as usize][nx as usize] {
            while free_init[(ny - dy) as usize][(nx - dx) as usize] {
                nx -= dx;
                ny -= dy;
            }
        }
        state.bliz_pos[i] = (nx as usize, ny as usize);
        state.free[ny as usize][nx as usize] = false;
    }
    state
}
