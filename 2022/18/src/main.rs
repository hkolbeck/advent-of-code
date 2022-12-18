use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;
use std::time::Instant;

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

static SIDES: [(i64, i64, i64); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1)
];


fn solve_1(input: HashSet<(i64, i64, i64)>) -> usize {
    let mut exposed_sides = 0;
    for (x, y, z) in &input {
        for (delta_x, delta_y, delta_z) in SIDES {
            if !input.contains(&(x + delta_x, y + delta_y, z + delta_z)) {
                exposed_sides += 1
            }
        }
    }

    exposed_sides
}

fn solve_2(input: HashSet<(i64, i64, i64)>) -> usize {
    let mut cache = HashMap::new();
    let max_x = input.iter().map(|(x, _, _)| *x).max().unwrap();
    let max_y = input.iter().map(|(_, y, _)| *y).max().unwrap();
    let max_z = input.iter().map(|(_, _, z)| *z).max().unwrap();

    let mut exposed_sides = 0;
    for (x, y, z) in &input {
        for (delta_x, delta_y, delta_z) in SIDES {
            if is_exposed(&input, max_x, max_y, max_z, x + delta_x, y + delta_y, z + delta_z, &mut cache) {
                exposed_sides += 1
            }
        }
    }

    exposed_sides
}

fn is_exposed(
    blob: &HashSet<(i64, i64, i64)>,
    max_x: i64,
    max_y: i64,
    max_z: i64,
    x: i64,
    y: i64,
    z: i64,
    cache: &mut HashMap<(i64, i64, i64), bool>
) -> bool {
    return search_for_exterior(blob, max_x, max_y, max_z, x, y, z, cache, &mut HashSet::new())
}

fn search_for_exterior(
    blob: &HashSet<(i64, i64, i64)>,
    max_x: i64,
    max_y: i64,
    max_z: i64,
    x: i64,
    y: i64,
    z: i64,
    cache: &mut HashMap<(i64, i64, i64), bool>,
    visited: &mut HashSet<(i64, i64, i64)>,
) -> bool {
    let key = (x, y, z);
    if blob.contains(&key) {
        return false
    } else if let Some(cached) = cache.get(&key) {
        return *cached;
    } else if visited.contains(&key) {
        return false;
    } else if !(0..=max_x).contains(&x) || !(0..=max_y).contains(&y) || !(0..=max_z).contains(&z) {
        return true;
    }

    visited.insert(key.clone());
    for (delta_x, delta_y, delta_z) in SIDES {
        if search_for_exterior(blob, max_x, max_y, max_z, x + delta_x, y + delta_y, z + delta_z, cache, visited) {
            cache.insert(key.clone(), true);
            return true
        }
    }

    cache.insert(key, false);
    false
}

fn parse_input(lines: Vec<String>) -> HashSet<(i64, i64, i64)> {
    lines.iter()
        .map(|l| l.split(','))
        .map(|p| p.map(|n| i64::from_str(n).unwrap()))
        .map(|mut p| (p.next().unwrap(), p.next().unwrap(), p.next().unwrap()))
        .collect()
}

