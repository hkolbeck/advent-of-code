use std::fs;
use std::str::FromStr;
use std::time::Instant;
use Op::{Add, Mult, Squared};


fn main() {
    let input = parse_monkeys(get_input());

    let start = Instant::now();
    let score = solve_1(input.clone());
    let end = Instant::now();
    println!("Part 1: {} in {}us", score, end.duration_since(start).as_micros());

    let start = Instant::now();
    let score = solve_2(input);
    let end = Instant::now();
    println!("Part 2: {} in {}us", score, end.duration_since(start).as_micros());
}

fn solve_1(mut monkies: Vec<Monkey>) -> u128 {
    let mut inspections = [0; 8];
    for _ in 0..20 {
        for idx in 0..8 {
            let monkey = monkies[idx].clone();
            for item in &monkey.items {
                let new_level = monkey.op.apply(*item) / 3;
                inspections[monkey.num] += 1;
                if new_level % monkey.test_mod == 0 {
                    monkies[monkey.true_monkey].items.push(new_level);
                } else {
                    monkies[monkey.false_monkey].items.push(new_level);
                }
            }

            let monkey = monkies.get_mut(idx).unwrap();
            monkey.items.clear();
        }
    }

    inspections.sort();
    inspections[6] * inspections[7]
}

fn solve_2(mut monkies: Vec<Monkey>) -> u128 {
    let modulus = monkies.iter()
        .map(|m| m.test_mod)
        .reduce(|a, b| a * b).unwrap();
    let mut inspections = [0; 8];
    for _ in 0..10000 {
        for idx in 0..8 {
            let monkey = monkies[idx].clone();
            for item in &monkey.items {
                let new_level = monkey.op.apply(*item) % modulus;
                inspections[monkey.num] += 1;
                if new_level % monkey.test_mod == 0 {
                    monkies[monkey.true_monkey].items.push(new_level);
                } else {
                    monkies[monkey.false_monkey].items.push(new_level);
                }
            }

            let monkey = monkies.get_mut(idx).unwrap();
            monkey.items.clear();
        }
    }

    inspections.sort();
    inspections[6] * inspections[7]
}

#[derive(Clone)]
enum Op {
    Mult(u128),
    Add(u128),
    Squared,
}

impl Op {
    fn apply(&self, old: u128) -> u128 {
        match self {
            Mult(val) => old * val,
            Add(val) => old + val,
            Squared => old * old,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    num: usize,
    items: Vec<u128>,
    op: Op,
    test_mod: u128,
    true_monkey: usize,
    false_monkey: usize,
}

fn parse_monkeys(input: Vec<String>) -> Vec<Monkey> {
    input.chunks(6).map(parse_monkey).collect()
}

fn parse_monkey(lines: &[String]) -> Monkey {
    let items: Vec<&str> = lines[1].split_whitespace().collect();
    let items: Vec<&str> = Vec::from(&items[2..]);

    Monkey {
        num: usize::from_str(&lines[0].split_whitespace().nth(1).unwrap()[0..1]).unwrap(),
        items: items.iter().map(|num| num.replace(',', ""))
            .map(|num| u128::from_str(num.as_str()).unwrap())
            .collect(),
        op: {
            let op_raw: Vec<&str> = lines[2].split_whitespace().rev().take(2).collect();
            let val = u128::from_str(op_raw[0]);
            match val {
                Ok(val) => match op_raw[1] {
                    "+" => Add(val),
                    "*" => Mult(val),
                    o => panic!("Unexpected op: {}", o)
                }
                Err(_) => Squared
            }
        },
        test_mod: u128::from_str(lines[3].split_whitespace().nth(3).unwrap()).unwrap(),
        true_monkey: usize::from_str(lines[4].split_whitespace().last().unwrap()).unwrap(),
        false_monkey: usize::from_str(lines[5].split_whitespace().last().unwrap()).unwrap(),
    }
}

fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input.txt").unwrap();
    input.split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}