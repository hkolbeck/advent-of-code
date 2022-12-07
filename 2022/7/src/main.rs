use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let score = solve_1();
    let end = Instant::now();
    println!("Part 1: {} in {}us", score, end.duration_since(start).as_micros());

    let start = Instant::now();
    let score = solve_2();
    let end = Instant::now();
    println!("Part 2: {} in {}us", score, end.duration_since(start).as_micros());
}

const MAX: u64 = 100000;
fn solve_1() -> u64 {
    let dir_sizes = get_dir_sizes();

    dir_sizes.iter()
        .filter(|(_, size)| size <= &&MAX)
        .map(|(_, size)| *size)
        .reduce(|sum, size| sum + size)
        .unwrap()
}

const NEEDED: u64 = 30000000;
const TOTAL: u64 = 70000000;
fn solve_2() -> u64 {
    let dir_sizes = get_dir_sizes();

    let used = dir_sizes[&String::from("/")];
    let unused = TOTAL - used;
    let need_to_delete = NEEDED - unused;

    dir_sizes.iter()
        .map(|(_, size)| *size)
        .filter(|size| size >= &need_to_delete)
        .min()
        .unwrap()
}

fn get_dir_sizes() -> HashMap<String, u64> {
    let lines = get_input();
    let mut dir_sizes: HashMap<String, u64> = HashMap::new();
    let mut cwd: Vec<String> = Vec::new();

    let mut idx = 0;
    while idx < lines.len() {
        let cmd_line = &lines[idx];
        if !cmd_line.starts_with('$') {
            panic!("Got unexpected line: '{}' on line {}", cmd_line, idx);
        }

        let cmd = &cmd_line[2..];
        idx += 1;

        if cmd.starts_with("cd") {
            let target = &cmd[3..];
            match target {
                "/" => cwd.push(String::from("")),
                ".." => { cwd.pop(); },
                dir => cwd.push(String::from(dir)),
            }
        } else if cmd.starts_with("ls") {
            while idx < lines.len() && !lines[idx].starts_with('$') {
                if !lines[idx].starts_with("dir") {
                    let (size_str, _) = lines[idx].split_once(' ').unwrap();
                    let size = u64::from_str(size_str).unwrap();
                    for i in 1..=cwd.len() {
                        dir_sizes.entry(dir_to_str(&cwd[..i]))
                            .and_modify(|s| *s += size)
                            .or_insert(size);
                    }
                }

                idx += 1;
            }
        } else {
            panic!("Got unexpected command '{}' on line {}", cmd, idx)
        }
    }
    dir_sizes
}

fn dir_to_str(dir: &[String]) -> String {
    format!("{}/", dir.join("/"))
}

fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input.txt").unwrap();
    input.split('\n').filter(|s| !s.is_empty()).map(String::from).collect()
}