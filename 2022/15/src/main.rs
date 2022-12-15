use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;
use std::time::Instant;
use regex::Regex;

fn main() {
    let raw_input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<String> = raw_input.split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    let input = parse_input(lines);

    let start = Instant::now();
    let answer_1 = solve_1(input.clone());
    let end = Instant::now();
    println!("Part 1: {} in {}us", answer_1, end.duration_since(start).as_micros());

    let start = Instant::now();
    let answer_2 = solve_2(input);
    let end = Instant::now();
    println!("Part 2: {} in {}us", answer_2, end.duration_since(start).as_micros());
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Beacon {
    x: i64,
    y: i64,
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Sensor {
    x: i64,
    y: i64,
}

fn solve_1(input: HashMap<Sensor, Beacon>) -> usize {
    let beacon_distances: HashMap<Sensor, usize> = beacon_dists(&input);

    let beacon_coords: HashSet<(i64, i64)> = input.values()
        .map(|b| (b.x, b.y))
        .collect();

    let (min_x, max_x) = input.iter().map(|(sensor, beacon)| {
        let radius = *beacon_distances.get(sensor).unwrap() as i64;
        (sensor.x.min(beacon.x) - radius, sensor.x.max(beacon.x) + radius)
    }).reduce(|(min_x, max_x), (l_min, l_max)|
        (min_x.min(l_min), max_x.max(l_max))
    ).unwrap();

    let mut count = 0;
    // let y = 10;
    let y = 2000000;
    for x in min_x..=max_x {
        if beacon_coords.contains(&(x, y)) {
            continue;
        }
        let maybe_beacon = Beacon { x, y };
        for (sensor, dist) in &beacon_distances {
            if manhatten_dist(sensor, &maybe_beacon) <= *dist {
                count += 1;
                break;
            }
        }
    }

    count
}

fn solve_2(input: HashMap<Sensor, Beacon>) -> i64 {
    let beacon_distances: HashMap<Sensor, usize> = beacon_dists(&input);
    for (sensor, dist) in &beacon_distances {
        let horizon = get_horizon(sensor, *dist, 4_000_000);
        for possibility in horizon {
            let mut covered = false;
            for (s, d) in &beacon_distances {
                if manhatten_dist(s, &possibility) <= *d {
                    covered = true;
                    break;
                }
            }
            if !covered {
                return possibility.x * 4_000_000 + possibility.y;
            }
        }
    }

    panic!("No solution found!")
}

fn get_horizon(sensor: &Sensor, dist: usize, ceil: i64) -> Vec<Beacon> {
    let hdist = dist as i64 + 1;
    let mut vec = Vec::with_capacity(4 * dist + 4);
    [(0, 1, 1, -1), (1, 0, -1, -1), (0, -1, -1, 1), (-1, 0, 1, 1)].iter()
        .for_each(|(start_x, start_y, delta_x, delta_y)| {
            let mut x = hdist * start_x + sensor.x;
            let mut y = hdist * start_y + sensor.y;

            for _ in 0..=dist {
                vec.push(Beacon{ x, y });
                x += delta_x;
                y += delta_y;
            }
        });

    vec.iter()
        .filter(|b| 0 <= b.x && b.x <= ceil && 0 <= b.y && b.y <= ceil)
        .map(|b| b.clone())
        .collect()
}

fn beacon_dists(input: &HashMap<Sensor, Beacon>) -> HashMap<Sensor, usize> {
    input.iter()
        .map(|(sensor, beacon)| {
            (sensor.clone(), manhatten_dist(sensor, beacon))
        }).collect()
}

fn manhatten_dist(sensor: &Sensor, beacon: &Beacon) -> usize {
    ((sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs()) as usize
}

// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
fn parse_input(lines: Vec<String>) -> HashMap<Sensor, Beacon> {
    let regex = Regex::new("(-?\\d+)").unwrap();
    let mut sensors = HashMap::new();

    for line in lines {
        let m: Vec<i64> = regex.find_iter(line.as_str())
            .map(|s| i64::from_str(s.as_str()).unwrap())
            .collect();

        sensors.insert(Sensor { x: m[0], y: m[1] }, Beacon { x: m[2], y: m[3] });
    }

    sensors
}
