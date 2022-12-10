use std::fs;
use std::str::FromStr;
use std::time::Instant;
use crate::Op::{Addx, Noop};

fn main() {
    let input = get_input();

    let start = Instant::now();
    let score = solve_1(&input);
    let end = Instant::now();
    println!("Part 1: {} in {}us", score, end.duration_since(start).as_micros());

    let start = Instant::now();
    let score = solve_2(&input);
    let end = Instant::now();
    println!("Part 2:\n{}\nin {}us", score, end.duration_since(start).as_micros());
}

enum Op {
    Noop,
    Addx(i32),
}

fn solve_1(lines: &Vec<String>) -> i32 {
    let register_hist = get_register_hist(lines);

    [19, 59, 99, 139, 179, 219]
        .map(|cycle| {
            println!("clock: {} Register: {}", cycle, register_hist[cycle]);
            register_hist[cycle] * (cycle as i32 + 1)
        })
        .into_iter()
        .reduce(|a,b| a + b)
        .unwrap()
}

fn solve_2(lines: &Vec<String>) -> String {
    let register_hist = get_register_hist(lines);
    let mut pixels: Vec<char> = Vec::new();

    for clock in 0..register_hist.len() {
        if (register_hist[clock] - (clock as i32 % 40)).abs() <= 1 {
            pixels.push('#');
        } else {
            pixels.push('.');
        }
    }

    let display: Vec<String> = pixels.chunks(40)
        .map(|line| String::from_iter(line.iter()))
        .collect();

    display.join("\n")
}

fn get_register_hist(lines: &Vec<String>) -> Vec<i32> {
    let mut register_hist: Vec<i32> = Vec::new();
    let mut register = 1;
    register_hist.push(register);

    for line in lines {
        let op = parse_line(line);
        match op {
            Noop => register_hist.push(register),
            Addx(val) => {
                register_hist.push(register);
                register += val;
                register_hist.push(register);
            }
        }
    }

    register_hist
}

fn parse_line(line: &String) -> Op {
    if line.starts_with("addx") {
        let val = i32::from_str(line.split_whitespace().nth(1).unwrap()).unwrap();
        Addx(val)
    } else if line.starts_with("noop") {
        Noop
    } else {
        panic!("Unexpected input: {}", line)
    }
}

fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input.txt").unwrap();
    input.split('\n').filter(|s| !s.is_empty()).map(String::from).collect()
}