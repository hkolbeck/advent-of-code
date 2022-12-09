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

fn solve_1(lines: &Vec<String>) -> usize {
    let mut field = [[false; 1001]; 1001];
    let mut head_row = 500;
    let mut head_col = 500;
    let mut tail_row = 500;
    let mut tail_col = 500;

    for line in lines {
        let (steps, hdr, hdc) = parse_line(line);
        for _ in 0..steps {
            field[tail_row as usize][tail_col as usize] = true;
            head_row += hdr;
            head_col += hdc;

            let (tdr, tdc) = mv(head_row - tail_row, head_col - tail_col);
            tail_row += tdr;
            tail_col += tdc;
        }
    }
    field[tail_row as usize][tail_col as usize] = true;

    field.iter().flatten().filter(|v| **v).count()
}

fn solve_2(lines: &Vec<String>) -> usize {
    let mut field = [[false; 1001]; 1001];
    let mut knot_positions = [(500, 500); 10];

    for line in lines {
        let (steps, hdr, hdc) = parse_line(line);
        for _ in 0..steps {
            field[knot_positions[9].0 as usize][knot_positions[9].1 as usize] = true;
            knot_positions[0].0 += hdr;
            knot_positions[0].1 += hdc;

            for knot in 1..=9 {
                let (dr, dc) = mv(
                    knot_positions[knot - 1].0 - knot_positions[knot].0,
                    knot_positions[knot - 1].1 - knot_positions[knot].1,
                );

                knot_positions[knot].0 += dr;
                knot_positions[knot].1 += dc;
            }
        }
    }
    field[knot_positions[9].0 as usize][knot_positions[9].1 as usize] = true;

    field.iter().flatten().filter(|v| **v).count()
}

fn mv(row_diff: i32, col_diff: i32) -> (i32, i32) {
    if row_diff.abs() <= 1 && col_diff.abs() <= 1 {
        (0, 0)
    } else {
        (row_diff.signum(), col_diff.signum())
    }
}

fn parse_line(line: &String) -> (usize, i32, i32) {
    let (direction, steps) = line.split_once(' ').unwrap();

    let steps = usize::from_str(steps).unwrap();
    match direction {
        "L" => (steps, 0, -1),
        "R" => (steps, 0, 1),
        "U" => (steps, 1, 0),
        "D" => (steps, -1, 0),
        _ => panic!("Unknown direction: {}", direction)
    }
}

fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input.txt").unwrap();
    input.split('\n').filter(|s| !s.is_empty()).map(String::from).collect()
}