use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let raw_input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<String> = raw_input.split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    let input = parse_input(lines);

    let start = Instant::now();
    let answer_1 = solve_1(input);
    let end = Instant::now();
    println!("Part 1: {} in {}us", answer_1, end.duration_since(start).as_micros());
}

fn solve_1(nums: Vec<(Vec<i128>, String)>) -> String {
    let sum: i128 = nums.iter().map(|(raw, _)| parse_snafu(raw)).sum();
    to_snafu(sum)
}

fn parse_snafu(snafu: &Vec<i128>) -> i128 {
    let mut place = 1;
    let mut sum = 0;
    for d in snafu {
        sum += d * place;
        place *= 5;
    }

    sum
}

fn to_snafu(num: i128) -> String {
    let mut places: HashMap<usize, i128> = HashMap::new();
    let mut num = num;
    let mut carry = 0;

    let mut place = 0;
    while num > 0 {
        let mut digit = num % 5 + carry;
        carry = 0;
        if digit > 2 {
            carry = 1;
            digit -= 5;
        }

        places.insert(place, digit);
        place += 1;
        num /= 5;
    }

    if carry == 1 {
        places.insert(place, 1);
    }

    let max_place = *places.keys().max().unwrap();
    (0..=max_place).rev().map(|place| match places.get(&place) {
        None => '0',
        Some(-2) => '=',
        Some(-1) => '-',
        Some(0) => '0',
        Some(1) => '1',
        Some(2) => '2',
        Some(a) => panic!("Uhhh: '{}'", a),
    }).collect()
}

fn parse_input(lines: Vec<String>) -> Vec<(Vec<i128>, String)> {
    lines.iter().map(|line| (line.chars().map(|c| match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Unknown: '{}'", c),
    }).rev().collect(), line.clone())).collect()
}
