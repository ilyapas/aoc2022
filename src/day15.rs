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
            covered.insert(*sensors.last().unwrap());
            covered.insert(*beacons.last().unwrap());
        });

    let y_range = 0..=4000000;
    let mut covered_ranges: Vec<Vec<(isize, isize)>> = vec![];
    for row in y_range {
        let mut ranges: Vec<(isize, isize)> = vec![];
        for (i, sensor) in sensors.iter().enumerate() {
            let dist_beacon = dist(*sensor, beacons[i]);
            let dist_row = dist(*sensor, (sensor.0, row));
            if dist_row <= dist_beacon {
                ranges.push((
                    sensor.0 - dist_beacon + dist_row,
                    sensor.0 + dist_beacon - dist_row,
                ));
            }
        }
        ranges.sort_by(|a, b| a.0.cmp(&b.0));

        let mut merged_ranges: Vec<(isize, isize)> = vec![ranges[0]];
        for i in 1..ranges.len() {
            if ranges[i].0 <= merged_ranges.last().unwrap().1 {
                merged_ranges.last_mut().unwrap().1 =
                    merged_ranges.last().unwrap().1.max(ranges[i].1);
            } else {
                merged_ranges.push(ranges[i]);
            }
        }
        covered_ranges.push(merged_ranges);
    }

    let row = 2000000;
    let covered_in_row = covered.iter().filter(|(_, y)| *y == row).count() as isize;
    let result_one = covered_ranges[row as usize]
        .iter()
        .map(|(a, b)| b - a + 1)
        .sum::<isize>()
        - covered_in_row;

    let mut distress_beacon: (isize, isize) = (0, 0);
    for (i, ranges) in covered_ranges.iter().enumerate() {
        if ranges.len() > 1 {
            distress_beacon.0 = (ranges[1].0 + ranges[0].1) / 2;
            distress_beacon.1 = i as isize;
            break;
        }
    }
    let result_two = distress_beacon.0 * 4000000 + distress_beacon.1;

    println!("Day 15 - Part One: {}", result_one);
    println!("Day 15 - Part Two: {}", result_two);
}
