use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;
use Tile::Sand;
use crate::Tile::Rock;

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

fn print_cave(cave: &HashMap<(i32, i32), Tile>) {
    for y in 0..25 {
        for x in 450..550 {
            match cave.get(&(x, y)) {
                None => print!("."),
                Some(t) => match t {
                    Sand => print!("o"),
                    Rock => print!("#"),
                }
            }
        }
        println!()
    }
}

fn solve_1(mut cave: HashMap<(i32, i32), Tile>) -> usize {
    let lowest_rock = *cave.iter()
        .map(|((_, y), _)| y)
        .max().unwrap();

    let mut dropped = 0;
    loop {
        print_cave(&cave);
        let mut x = 500;
        let mut y = 0;

        loop {
            if y >= lowest_rock {
                return dropped;
            } else if let None = cave.get(&(x, y + 1)) {
                y += 1;
            } else if let None = cave.get(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if let None = cave.get(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                dropped += 1;
                cave.insert((x, y), Sand);
                break;
            }
        }
    }
}

fn solve_2(mut cave: HashMap<(i32, i32), Tile>) -> usize {
    let floor = *cave.iter()
        .map(|((_, y), _)| y)
        .max().unwrap() + 2;

    for x in 0..1000 {
        cave.insert((x, floor), Rock);
    }

    let mut dropped = 0;
    while cave.get(&(500, 0)).is_none() {
        print_cave(&cave);
        let mut x = 500;
        let mut y = 0;

        loop {
            if let None = cave.get(&(x, y + 1)) {
                y += 1;
            } else if let None = cave.get(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if let None = cave.get(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                dropped += 1;
                cave.insert((x, y), Sand);
                break;
            }
        }
    }

    dropped
}

#[derive(Copy, Clone)]
enum Tile {
    Sand,
    Rock,
}

fn parse_input(lines: Vec<String>) -> HashMap<(i32, i32), Tile> {
    let paths = parse_paths(lines);
    let mut cave: HashMap<(i32, i32), Tile> = HashMap::new();
    for path in paths {
        path.windows(2).for_each(|w| {
            let (x1, y1) = w[0] as (i32, i32);
            let (x2, y2) = w[1] as (i32, i32);
            let delta = ((x2 - x1).signum(), (y2 - y1).signum());

            let mut x = x1;
            let mut y = y1;
            while (x, y) != w[1] {
                cave.insert((x, y), Rock);
                x += delta.0;
                y += delta.1;
            }
            cave.insert(w[1], Rock);
        })
    }

    cave
}

fn parse_paths(lines: Vec<String>) -> Vec<Vec<(i32, i32)>> {
    lines.iter().map(parse_path).collect()
}

fn parse_path(line: &String) -> Vec<(i32, i32)> {
    line.split(" -> ")
        .map(|p| p.split(',').map(|n| i32::from_str(n).unwrap()).collect())
        .map(|p: Vec<i32>| (p[0], p[1]))
        .collect()
}