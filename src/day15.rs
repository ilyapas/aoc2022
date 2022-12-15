use regex::Regex;
use std::collections::HashSet;

fn dist((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day15.prod.txt").unwrap();

    let mut sensors: Vec<(isize, isize)> = vec![];
    let mut beacons: Vec<(isize, isize)> = vec![];
    let mut covered: HashSet<(isize, isize)> = HashSet::new();

    let re = Regex::new(
        r"^Sensor at x=(\-*\d+), y=(\-*\d+): closest beacon is at x=(\-*\d+), y=(\-*\d+)$",
    )
    .unwrap();

    input
        .lines()
        .map(|s| re.captures(s).unwrap())
        .for_each(|c| {
            sensors.push((
                c.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                c.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            ));
            beacons.push((
                c.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                c.get(4).unwrap().as_str().parse::<isize>().unwrap(),
            ));
        });

    let row = 2000000;
    for (i, sensor) in sensors.iter().enumerate() {
        let dist_beacon = dist(*sensor, beacons[i]);
        let dist_row = dist(*sensor, (sensor.0, row));
        if dist_row <= dist_beacon {
            for x in sensor.0 - dist_beacon..=sensor.0 + dist_beacon {
                if dist((x, row), *sensor) <= dist_beacon
                    && !sensors.contains(&(x, row))
                    && !beacons.contains(&(x, row))
                {
                    covered.insert((x, row));
                }
            }
        }
    }

    println!("Day 15 - Part One: {}", covered.len());
}
