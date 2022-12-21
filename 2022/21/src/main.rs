use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;
use regex::Regex;
use crate::Monkey::{Add, Div, Eq, Human, Mult, Number, Sub};

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

fn solve_1(monkeys: HashMap<String, Monkey>) -> i64 {
    walk_tree(&monkeys, &String::from("root")).unwrap()
}

fn solve_2(monkeys: HashMap<String, Monkey>) -> i64 {
    let (val, human_tree) = find_human_subtree(&monkeys);
    collapse(&monkeys, &human_tree, val)
}

fn collapse(monkeys: &HashMap<String, Monkey>, at: &str, must_eq: i64) -> i64 {
    match monkeys.get(at).unwrap() {
        Number(n) => *n,
        Add(a, b) => match walk_tree(monkeys, a) {
            None => match walk_tree(monkeys, b) {
                None => panic!("Nada"),
                Some(val_b) => collapse(monkeys, a, must_eq - val_b),
            }
            Some(val_a) => match walk_tree(monkeys, b) {
                None => collapse(monkeys, b, must_eq - val_a),
                Some(val_b) => val_b + val_a,
            }
        },
        Sub(a, b) => match walk_tree(monkeys, a) {
            None => match walk_tree(monkeys, b) {
                None => panic!("Nada"),
                Some(val_b) => collapse(monkeys, a, must_eq + val_b),
            }
            Some(val_a) => match walk_tree(monkeys, b) {
                None => collapse(monkeys, b, val_a - must_eq),
                Some(val_b) => val_b + val_a,
            }
        },
        Mult(a, b) => match walk_tree(monkeys, a) {
            None => match walk_tree(monkeys, b) {
                None => panic!("Nada"),
                Some(val_b) => collapse(monkeys, a, must_eq / val_b),
            }
            Some(val_a) => match walk_tree(monkeys, b) {
                None => collapse(monkeys, b, must_eq / val_a),
                Some(val_b) => val_b + val_a,
            }
        },
        Div(a, b) => match walk_tree(monkeys, a) {
            None => match walk_tree(monkeys, b) {
                None => panic!("Nada"),
                Some(val_b) => collapse(monkeys, a, must_eq * val_b),
            }
            Some(val_a) => match walk_tree(monkeys, b) {
                None => collapse(monkeys, b, val_a / must_eq),
                Some(val_b) => val_b + val_a,
            }
        },
        Human => {
            println!("Must eq: {}", must_eq);
            must_eq
        }
        Eq(_, _) => panic!("Eq in tree"),
    }
}

fn find_human_subtree(monkeys: &HashMap<String, Monkey>) -> (i64, String) {
    let (left, right) = match monkeys.get("root").unwrap() {
        Eq(left, right) => (left, right),
        _ => panic!("Root wasn't Eq"),
    };

    match walk_tree(monkeys, left) {
        None => match walk_tree(monkeys, right) {
            None => panic!("Nada"),
            Some(val) => (val, left.clone())
        }
        Some(val) => match walk_tree(monkeys, right) {
            None => (val, right.clone()),
            Some(_) => panic!("Both some")
        }
    }

}

fn walk_tree(monkeys: &HashMap<String, Monkey>, start: &String) -> Option<i64> {
    match monkeys.get(start).unwrap() {
        Number(n) => Some(*n),
        Add(a, b) => walk_tree(monkeys, a)
            .zip(walk_tree(monkeys, b))
            .map(|(a, b)| a + b),
        Sub(a, b) => walk_tree(monkeys, a)
            .zip(walk_tree(monkeys, b))
            .map(|(a, b)| a - b),
        Mult(a, b) => walk_tree(monkeys, a)
            .zip(walk_tree(monkeys, b))
            .map(|(a, b)| a * b),
        Div(a, b) => walk_tree(monkeys, a)
            .zip(walk_tree(monkeys, b))
            .map(|(a, b)| a / b),
        Human => None,
        Eq(_, _) => panic!("Eq!"),
    }
}

#[derive(Debug, Hash, Clone)]
enum Monkey {
    Number(i64),
    Add(String, String),
    Sub(String, String),
    Mult(String, String),
    Div(String, String),
    Eq(String, String),
    Human,
}

fn parse_input(lines: Vec<String>) -> HashMap<String, Monkey> {
    let num_re = Regex::new("(\\w{4}): (\\d+)").unwrap();
    let op_re = Regex::new("(\\w{4}): (\\w{4}) ([-+*/]) (\\w{4})").unwrap();
    
    lines.iter().map(|line| {
        if line.starts_with("humn") {
            (String::from("humn"), Human)
        } else if let Some(captures) = num_re.captures(line.as_str()) {
            let name = String::from(captures.get(1).unwrap().as_str());
            let num = i64::from_str(captures.get(2).unwrap().as_str()).unwrap();

            (name, Number(num))
        } else if let Some(captures) = op_re.captures(line.as_str()) {
            let name = String::from(captures.get(1).unwrap().as_str());
            let other_1 = String::from(captures.get(2).unwrap().as_str());
            let op = captures.get(3).unwrap().as_str();
            let other_2 = String::from(captures.get(4).unwrap().as_str());

            if name.as_str() == "root" {
                (name, Eq(other_1, other_2))    
            } else {
                match op {
                    "-" => (name, Sub(other_1, other_2)),
                    "+" => (name, Add(other_1, other_2)),
                    "*" => (name, Mult(other_1, other_2)),
                    "/" => (name, Div(other_1, other_2)),
                    wat => panic!("Wat '{}'", wat)

                }
            }
        } else {
            panic!("Wat: '{}'", line)
        }
    }).collect()
}
