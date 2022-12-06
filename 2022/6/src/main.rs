use std::collections::HashSet;
use std::fs;
use std::time::Instant;
use nom::InputIter;

fn main() {
    let start = Instant::now();
    let score = solve_n(4);
    let end = Instant::now();
    println!("Part 1: {} in {}us", score, end.duration_since(start).as_micros());

    let start = Instant::now();
    let score = solve_n(14);
    let end = Instant::now();
    println!("Part 2: {} in {}us", score, end.duration_since(start).as_micros());
}

fn solve_n(n: usize) -> i64 {
    let input = fs::read_to_string("input.txt").unwrap();
    for i in 0..input.len() {
        let set: HashSet<char> = (&input[i..i+n]).iter_elements().collect();
        if set.len() == n {
            return (i + n) as i64;
        }
    }

    -1
}