extern crate core;

use std::fs;
use std::time::Instant;

enum Play {
    Rock,
    Paper,
    Scissors,
}

enum Resolution {
    Lose,
    Draw,
    Win,
}

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

        let (opponent_raw, me_raw) = line.split_once(' ').unwrap();
        let opponent = match opponent_raw {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!("Got unexpected input for opponent {}", opponent_raw),
        };
        let me = match me_raw {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => panic!("Got unexpected input for me {}", me_raw),
        };

        score += play_round_1((opponent, me))
    }

    score
}

fn play_round_1(round: (Play, Play)) -> u64 {
    match round {
        (Play::Rock, Play::Rock) => 4,
        (Play::Rock, Play::Paper) => 8,
        (Play::Rock, Play::Scissors) => 3,
        (Play::Paper, Play::Paper) => 5,
        (Play::Paper, Play::Rock) => 1,
        (Play::Paper, Play::Scissors) => 9,
        (Play::Scissors, Play::Scissors) => 6,
        (Play::Scissors, Play::Paper) => 2,
        (Play::Scissors, Play::Rock) => 7
    }
}

fn solve_part_2() -> u64 {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.split('\n');

    let mut score: u64 = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let (opponent_raw, res_raw) = line.split_once(' ').unwrap();
        let opponent = match opponent_raw {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!("Got unexpected input for opponent {}", opponent_raw),
        };
        let res = match res_raw {
            "X" => Resolution::Lose,
            "Y" => Resolution::Draw,
            "Z" => Resolution::Win,
            _ => panic!("Got unexpected input for res {}", res_raw),
        };

        score += play_round_2((opponent, res))
    }

    score
}

fn play_round_2(round: (Play, Resolution)) -> u64 {
    match round {
        (Play::Rock, Resolution::Lose) => 3,
        (Play::Rock, Resolution::Draw) => 4,
        (Play::Rock, Resolution::Win) => 8,
        (Play::Paper, Resolution::Lose) => 1,
        (Play::Paper, Resolution::Draw) => 5,
        (Play::Paper, Resolution::Win) => 9,
        (Play::Scissors, Resolution::Lose) => 2,
        (Play::Scissors, Resolution::Draw) => 6,
        (Play::Scissors, Resolution::Win) => 7
    }
}