use std::{fs, usize};
use std::collections::VecDeque;
use std::str::FromStr;
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

fn solve_part_1() -> String {
    let (mut columns, moves) = get_columns();
    for order in moves {
        let (reps, from, to) = parse_order(&order);
        for _ in 0..reps {
            let boxx = columns.get_mut(from).unwrap().pop_back().unwrap();
            columns.get_mut(to).unwrap().push_back(boxx)
        }
    }

    columns.iter().map(|col| *col.back().unwrap_or(&' ')).collect()
}

fn solve_part_2() -> String {
    let (mut columns, moves) = get_columns();
    for order in moves {
        let (reps, from, to) = parse_order(&order);

        let mut moves = vec![];
        for _ in 0..reps {
            moves.push(columns.get_mut(from).unwrap().pop_back().unwrap());
        }

        moves.reverse();
        for boxx in moves {
            columns.get_mut(to).unwrap().push_back(boxx)
        }
    }

    columns.iter().map(|col| *col.back().unwrap_or(&' ')).collect()
}

fn get_columns() -> (Vec<VecDeque<char>>, Vec<String>) {
    let lines = get_input();
    let (mut stacks, col_labels_and_moves): (Vec<String>, Vec<String>) = lines.into_iter()
        .partition(|l| l.find("[").is_some());
    let (labels, moves) = col_labels_and_moves.split_at(1);
    let column_labels: Vec<String>= labels[0].split(' ').into_iter()
        .map(String::from)
        .filter(|s| !s.is_empty())
        .collect();
    let last_col = column_labels.last().unwrap();
    let num_columns = usize::from_str(last_col).unwrap();
    let mut columns: Vec<VecDeque<char>> = (0..=num_columns).map(|_| VecDeque::new()).collect();

    stacks.reverse();
    for row in stacks {
        for i in 1..=num_columns {
            let letter: char = row.char_indices().nth(4 * (i - 1) + 1).unwrap().1;
            if letter != ' ' {
                columns.get_mut(i).unwrap().push_back(letter)
            }
        }
    }

    (columns, Vec::from(moves))
}

fn parse_order(str: &String) -> (usize, usize, usize) {
    let parts: Vec<&str> = str.split(' ').collect();

    (
        usize::from_str(parts[1]).unwrap(),
        usize::from_str(parts[3]).unwrap(),
        usize::from_str(parts[5]).unwrap()
    )
}

fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input.txt").unwrap();
    input.split('\n').filter(|s| !s.is_empty()).map(String::from).collect()
}