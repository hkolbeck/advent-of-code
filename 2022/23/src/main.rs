use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;
use crate::Move::{East, North, South, West};

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
    println!("Part 2: {:?} in {}us", answer_2, end.duration_since(start).as_micros());
}

fn solve_1(mut elves: HashSet<(i128, i128)>) -> i128 {
    let (initial_check, mut rules) = get_rules();
    for _ in 0..10 {
        let mut considered_moves = HashMap::new();
        let mut frozen = HashSet::new();
        for elf in &elves {
            if matches_rule(&elves, &initial_check, elf) {
                continue;
            }

            for (rule, mv) in &rules {
                if matches_rule(&elves, rule, elf) {
                    let (delta_x, delta_y) = mv.get_delta();
                    let existing = considered_moves.insert(
                        (elf.0 + delta_x, elf.1 + delta_y),
                        elf.clone(),
                    );

                    if let Some(other_elf) = existing {
                        frozen.insert(other_elf);
                        frozen.insert(elf.clone());
                    }

                    break;
                }
            }
        }

        for (elf_dest, elf_loc) in considered_moves {
            if !frozen.contains(&elf_loc) {
                elves.remove(&elf_loc);
                elves.insert(elf_dest);
            }
        }

        rules.rotate_left(1);
    }

    let (x_min, x_max, y_min, y_max) = get_rect(&elves);
    (x_max - x_min + 1) * (y_max - y_min + 1) - elves.len() as i128
}

fn solve_2(mut elves: HashSet<(i128, i128)>) -> i32 {
    let (initial_check, mut rules) = get_rules();
    for round in 1.. {
        let mut considered_moves = HashMap::new();
        let mut frozen = HashSet::new();
        for elf in &elves {
            if matches_rule(&elves, &initial_check, elf) {
                frozen.insert(elf.clone());
                continue;
            }

            for (rule, mv) in &rules {
                if matches_rule(&elves, rule, elf) {
                    let (delta_x, delta_y) = mv.get_delta();
                    let existing = considered_moves.insert(
                        (elf.0 + delta_x, elf.1 + delta_y),
                        elf.clone(),
                    );

                    if let Some(other_elf) = existing {
                        frozen.insert(other_elf);
                        frozen.insert(elf.clone());
                    }

                    break;
                }
            }
        }

        if frozen.len() == elves.len() {
            println!("OMG! {}", round);
            return round;
        }

        for (elf_dest, elf_loc) in considered_moves {
            if !frozen.contains(&elf_loc) {
                elves.remove(&elf_loc);
                elves.insert(elf_dest);
            }
        }

        rules.rotate_left(1);
    }

    -1
}

enum Move {
    North,
    South,
    East,
    West,
}

impl Move {
    fn get_delta(&self) -> (i128, i128) {
        match self {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        }
    }
}

fn matches_rule(elves: &HashSet<(i128, i128)>, rule: &Vec<(i128, i128)>, elf: &(i128, i128)) -> bool {
    for (delta_x, delta_y) in rule {
        if elves.contains(&(elf.0 + delta_x, elf.1 + delta_y)) {
            return false;
        }
    }

    true
}

fn get_rules() -> (Vec<(i128, i128)>, Vec<(Vec<(i128, i128)>, Move)>) {
    (
        vec![
            (-1, -1), (0, -1), (1, -1),
            (-1, 0), /*     */ (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ], // Check for no-one
        vec![
            (vec![(-1, -1), (0, -1), (1, -1)], North),
            (vec![(-1, 1), (0, 1), (1, 1)], South),
            (vec![(-1, -1), (-1, 0), (-1, 1)], West),
            (vec![(1, -1), (1, 0), (1, 1)], East),
        ]
    )
}

fn print_elves(elves: &HashSet<(i128, i128)>, rules: &Vec<(Vec<(i128, i128)>, Move)>) -> String {
    let (x_min, x_max, y_min, y_max) = get_rect(&elves);
    let rule_order: Vec<&str> = rules.iter().map(|(_, mv)| match mv.clone() {
        North => "N",
        South => "S",
        East => "E",
        West => "W",
    }).collect();
    let mut str = format!("{:?}\n", rule_order.join(""));
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if elves.contains(&(x, y)) {
                str.push('#');
            } else {
                str.push('.');
            }
        }
        str.push('\n');
    }

    str
}

fn get_rect(elves: &HashSet<(i128, i128)>) -> (i128, i128, i128, i128) {
    elves.iter().map(|(x, y)| (*x, *x, *y, *y))
        .reduce(|(x_min, x_max, y_min, y_max), (x, _, y, _)|
            (x_min.min(x), x_max.max(x), y_min.min(y), y_max.max(y))
        ).unwrap()
}

fn parse_input(lines: Vec<String>) -> HashSet<(i128, i128)> {
    let mut elves = HashSet::default();
    for (line, y) in lines.iter().zip(0..) {
        line.chars().zip(0..).for_each(|(c, x)| {
            if c == '#' {
                elves.insert((x, y));
            }
        });
    }

    elves
}
