extern crate core;

use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let score = solve_part_1();
    let end = Instant::now();
    println!("Part 1: {} in {}us", score, end.duration_since(start).as_micros());

    let start = Instant::now();
    let score = solve_part_2();
    let end = Instant::now();
    println!("Part 2: {} in {}us", score, end.duration_since(start).as_micros());
}

fn solve_part_1() -> u64 {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.split('\n');

    let mut score: u64 = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let comp_len = line.len() / 2;
        let comp_1: HashSet<char> = line[0..comp_len].chars().collect();
        let comp_2: HashSet<char> = line[comp_len..line.len()].chars().collect();
        let intersection: Vec<&char> = comp_1.intersection(&comp_2).collect();
        if intersection.len() != 1 {
            panic!("Got multiple items on line: '{}'", line);
        }

        score += score_for_letter(**intersection.last().unwrap())
    }

    score
}

fn solve_part_2() -> u64 {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();
    let chunks = lines.chunks(3);
    let mut score: u64 = 0;
    for chunk in chunks {
        let elf_1: HashSet<char> = chunk[0].chars().collect();
        let elf_2: HashSet<char> = chunk[1].chars().collect();
        let elf_3: HashSet<char> = chunk[2].chars().collect();
        let overlap: HashSet<char> = elf_1.intersection(&elf_2).map(|c| *c).collect();
        let overlap: Vec<&char> = overlap.intersection(&elf_3).collect();
        score += score_for_letter(**overlap.last().unwrap())
    }

    score
}

fn score_for_letter(letter: char) -> u64 {
    if 'a' <= letter && letter <= 'z' {
        letter as u64 - ('a' as u64) + 1
    } else if 'A' <= letter && letter <= 'Z' {
        26 + letter as u64 - 'A' as u64 + 1
    } else {
        panic!("Unexpected char: '{}'", letter)
    }
}