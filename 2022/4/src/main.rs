use std::fs;
use std::iter::Filter;
use std::str::{FromStr, Split};
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
    let lines = get_input();
    let mut count: u64 = 0;
    for line in lines {
        let (elf_1, elf_2) = parse_line(line);
        if overlap_completely(&elf_1, &elf_2) || overlap_completely(&elf_2, &elf_1) {
            count += 1;
        }
    }

    count
}

fn solve_part_2() -> u64 {
    let lines = get_input();
    let mut count: u64 = 0;
    for line in lines {
        let (elf_1, elf_2) = parse_line(line);
        if overlap_at_all(&elf_1, &elf_2) || overlap_at_all(&elf_2, &elf_1) {
            count += 1;
        }
    }

    count
}

fn get_input() -> Filter<Split<char>, fn(&&str) -> bool> {
    let input = fs::read_to_string("input.txt").unwrap();
    input.split('\n').filter(|s| !s.is_empty())
}

fn overlap_at_all(a: &(u32, u32), b: &(u32, u32)) -> bool {
    a.0 <= b.0 && b.0 <= a.1
}

fn overlap_completely(a: &(u32, u32), b: &(u32, u32)) -> bool {
    a.0 <= b.0 && b.1 <= a.1
}

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let (elf_1_raw, elf_2_raw) = line.split_once(',').unwrap();
    let elf_1 = parse_range(elf_1_raw);
    let elf_2 = parse_range(elf_2_raw);

    (elf_1, elf_2)
}

fn parse_range(raw: &str) -> (u32, u32) {
    let (start_str, end_str) = raw.split_once('-').unwrap();
    let start = u32::from_str(start_str).unwrap();
    let end = u32::from_str(end_str).unwrap();

    (start, end)
}