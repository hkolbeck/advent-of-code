use std::fs;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let input = get_input();

    let start = Instant::now();
    let score = solve_1(&input);
    let end = Instant::now();
    println!("Part 1: {} in {}us", score, end.duration_since(start).as_micros());

    let start = Instant::now();
    let score = solve_2(&input);
    let end = Instant::now();
    println!("Part 2: {} in {}us", score, end.duration_since(start).as_micros());
}

fn solve_1(input: &Vec<String>) -> usize {
    let forest = parse_forest(input);
    let cols = forest[0].len();
    let rows = forest.len();

    let mut count: usize = 0;
    for row in 0..rows {
        for col in 0..cols {
            if let Some(max) = forest[row][0..col].iter().max() {
                if max < &forest[row][col] {
                    count += 1;
                    continue;
                }
            } else {
                count += 1;
                continue;
            }

            if let Some(max) = forest[row][(col+1)..cols].iter().max() {
                if max < &forest[row][col] {
                    count += 1;
                    continue;
                }
            } else {
                count += 1;
                continue;
            }

            if let Some(max) = (0..row).map(|r| forest[r][col]).max() {
                if max < forest[row][col] {
                    count += 1;
                    continue;
                }
            } else {
                count += 1;
                continue;
            }

            if let Some(max) = ((row + 1)..rows).map(|r| forest[r][col]).max() {
                if max < forest[row][col] {
                    count += 1;
                    continue;
                }
            } else {
                count += 1;
                continue;
            }
        }
    }

    count
}

fn solve_2(input: &Vec<String>) -> i32 {
    let forest = parse_forest(input);
    let cols = forest[0].len();
    let rows = forest.len();

    let mut best_score: i32 = 0;
    for row in 0..rows {
        for col in 0..cols {
            let mut left = forest[row][0..col].iter().rev()
                .take_while(|t| t < &&forest[row][col])
                .count() as i32;
            if left < (col as i32) - 1 {
                left += 1;
            }

            let mut right = forest[row][(col + 1)..cols].iter()
                .take_while(|t| t < &&forest[row][col])
                .count() as i32;
            if right < cols as i32 - col as i32 - 1 {
                right += 1;
            }

            let mut up = (0..row).rev()
                .map(|r| forest[r][col])
                .take_while(|t| t < &&forest[row][col])
                .count() as i32;
            if up < row as i32 - 1 {
                up += 1;
            }

            let mut down = ((row + 1)..rows)
                .map(|r| forest[r][col])
                .take_while(|t| t < &&forest[row][col])
                .count() as i32;
            if down < rows as i32 - row as i32 - 1 {
                down += 1;
            }

            best_score = best_score.max(up * down * left * right);
        }
    }

    best_score
}

fn parse_forest(lines: &Vec<String>) -> Vec<Vec<i8>> {
    let mut rows: Vec<Vec<i8>> = Vec::new();
    for line in lines {
        let row: Vec<i8> = line.chars()
            .map(|c| i8::from_str(c.to_string().as_str()).unwrap())
            .collect();
        rows.push(row);
    }

    rows
}

fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input.txt").unwrap();
    input.split('\n').filter(|s| !s.is_empty()).map(String::from).collect()
}