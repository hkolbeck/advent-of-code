use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;
use std::time::Instant;
use regex::Regex;
use Direction::{Left, Right, Up};
use Instruction::{Forward, LeftTurn, RightTurn};
use crate::BoardSquare::{Open, Void, Wall};
use crate::Direction::Down;

fn main() {
    let raw_input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<String> = raw_input.split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    let (instr, board, start_pos) = parse_input_1(lines.clone());
    let start = Instant::now();
    let answer_1 = solve_1(instr.clone(), board.clone(), start_pos.clone());
    let end = Instant::now();
    println!("Part 1: {} in {}us", answer_1, end.duration_since(start).as_micros());


    let (instructions, board, start_pos, reverse_map) = parse_input_2(lines);
    let start = Instant::now();
    let answer_2 = solve_2(instructions, board, start_pos, reverse_map);
    let end = Instant::now();
    println!("Part 2: {} in {}us", answer_2, end.duration_since(start).as_micros());
}

fn solve_1(
    instructions: Vec<Instruction>,
    board: HashMap<(usize, usize), HashMap<Direction, (usize, usize)>>,
    start: (usize, usize),
) -> usize {
    let mut you = You {
        coord: start,
        heading: Right,
    };

    for instr in &instructions {
        you.execute(&board, instr);
    }

    let facing = match &you.heading {
        Right => 0,
        Down => 1,
        Left => 2,
        Up => 3,
    };

    1000 * (you.coord.0 + 1) + 4 * (you.coord.1 + 1) + facing
}

#[derive(Debug, Clone)]
enum Instruction {
    Forward(usize),
    LeftTurn,
    RightTurn,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Clone)]
struct You {
    coord: (usize, usize),
    heading: Direction,
}

impl You {
    fn execute(&mut self, board: &HashMap<(usize, usize), HashMap<Direction, (usize, usize)>>, instr: &Instruction) {
        let new_heading = match instr {
            LeftTurn => match self.heading {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up
            },
            RightTurn => match self.heading {
                Up => Right,
                Left => Up,
                Down => Left,
                Right => Down,
            }
            Forward(steps) => {
                let mut coord = &self.coord;
                for _ in 0..*steps {
                    let links = board.get(coord).unwrap();
                    if let Some(next) = links.get(&self.heading) {
                        coord = next;
                    } else {
                        break;
                    }
                }
                self.coord = coord.clone();
                self.heading.clone()
            }
        };

        self.heading = new_heading;
    }
}

struct YouToo {
    coord: (u8, usize, usize),
    heading: Direction,
}

impl YouToo {
    fn execute(&mut self, board: &HashMap<(u8, usize, usize), HashMap<Direction, ((u8, usize, usize), Direction)>>, instr: &Instruction) {
        let new_heading = match instr {
            LeftTurn => match self.heading {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up
            },
            RightTurn => match self.heading {
                Up => Right,
                Left => Up,
                Down => Left,
                Right => Down,
            }
            Forward(steps) => {
                let mut coord = &self.coord;
                let mut dir = &self.heading;
                for _ in 0..*steps {
                    let links = board.get(coord).unwrap();
                    if let Some(next) = links.get(dir) {
                        coord = &next.0;
                        dir = &next.1;
                    } else {
                        break;
                    }
                }
                self.coord = coord.clone();
                dir.clone()
            }
        };

        self.heading = new_heading;
    }
}

fn solve_2(
    instructions: Vec<Instruction>,
    board: HashMap<(u8, usize, usize), HashMap<Direction, ((u8, usize, usize), Direction)>>,
    start_pos: (u8, usize, usize),
    orig_mapping: HashMap<(u8, usize, usize), (usize, usize)>
) -> usize {
    let mut you = YouToo {
        coord: start_pos,
        heading: Right
    };

    for instr in &instructions {
        you.execute(&board, instr);
    }

    let facing = match &you.heading {
        Right => 0,
        Down => 1,
        Left => 2,
        Up => 3,
    };

    let (orig_row, orig_col) = orig_mapping.get(&you.coord).unwrap();
    1000 * (orig_row + 1) + 4 * (orig_col + 1) + facing
}

fn _print_board(board: &HashMap<(usize, usize), HashMap<Direction, (usize, usize)>>, you: &You) {
    let (max_row, max_col) = board.clone().into_iter()
        .map(|(k, _)| k)
        .reduce(|(max_row, max_col), (r, c)|
            (max_row.max(r), max_col.max(c))
        ).unwrap().clone();

    for row in 0..=max_row {
        for col in 0..=max_col {
            if you.coord.0 == row && you.coord.1 == col {
                print!("@")
            } else if let Some(_) = board.get(&(row, col)) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn print_face(board: &HashMap<(u8, usize, usize), HashMap<Direction, ((u8, usize, usize), Direction)>>, you: &YouToo) {
    let (your_face, your_row, your_col) = &you.coord;
    println!("Face: {}", your_face);
    for row in 0..50 {
        for col in 0..50 {
            if *your_row == row && *your_col == col {
                match &you.heading {
                    Up => print!("^"),
                    Left => print!("<"),
                    Down => print!("v"),
                    Right => print!(">"),
                }
            } else if let Some(_) = board.get(&(*your_face, row, col)) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}


#[derive(Eq, PartialEq)]
enum BoardSquare {
    Open,
    Wall,
    Void,
}

fn parse_input_1(
    lines: Vec<String>
) -> (Vec<Instruction>, HashMap<(usize, usize), HashMap<Direction, (usize, usize)>>, (usize, usize)) {
    let mut board_rows = Vec::new();
    for line in &lines[0..(lines.len() - 1)] {
        let row: Vec<BoardSquare> = line.chars().map(|c| {
            match c {
                ' ' => Void,
                '.' => Open,
                '#' => Wall,
                _ => panic!("Board wat: '{}'", c),
            }
        }).collect();
        board_rows.push(row);
    }

    let mut reachable = HashSet::new();
    let mut walls = HashSet::new();
    for row in 0..board_rows.len() {
        for col in 0..board_rows[row].len() {
            if board_rows[row][col] == Open {
                reachable.insert((row, col));
            } else if board_rows[row][col] == Wall {
                walls.insert((row, col));
            }
        }
    }

    let max_row = board_rows.len() - 1;
    let max_col = board_rows.iter().map(|r| r.len()).max().unwrap();

    let mut board = HashMap::new();
    for (row, col) in &reachable {
        let mut links = HashMap::new();
        [Up, Down, Right, Left].into_iter()
            .for_each(|dir| {
                let mut r = *row;
                let mut c = *col;
                loop {
                    let next_coord = match dir {
                        Up if r == 0 => (max_row, c),
                        Up => (r - 1, c),
                        Left if c == 0 => (r, max_col),
                        Left => (r, c - 1),
                        Down if r >= max_row => (0, c),
                        Down => (r + 1, c),
                        Right if c >= max_col => (r, 0),
                        Right => (r, c + 1),
                    };

                    if reachable.contains(&next_coord) {
                        links.insert(dir, next_coord);
                        break;
                    } else if walls.contains(&next_coord) {
                        break;
                    } else {
                        r = next_coord.0;
                        c = next_coord.1;
                    }
                };
            });


        board.insert((*row, *col), links);
    }


    let mut idx = 0;
    let start = loop {
        if board_rows[0][idx] == Open {
            break (0, idx);
        }
        idx += 1;
    };

    let instruction_line = lines.last().unwrap();
    let number_re = Regex::new("\\d+").unwrap();
    let direction_re = Regex::new("[LR]").unwrap();
    let numbers: Vec<usize> = direction_re
        .split(instruction_line)
        .map(|n| usize::from_str(n).unwrap())
        .collect();
    let turns: Vec<&str> = number_re.split(instruction_line)
        .filter(|s| !s.is_empty())
        .collect();

    let mut instructions = Vec::new();
    for idx in 0..turns.len() {
        let dir = match turns[idx] {
            "L" => LeftTurn,
            "R" => RightTurn,
            a => panic!("Uhhh: -'{}'- ", a),
        };
        instructions.push(Forward(numbers[idx]));
        instructions.push(dir);
    }
    instructions.push(Forward(*numbers.last().unwrap()));

    (instructions, board, start)
}

fn parse_input_2(
    lines: Vec<String>
) -> (
    Vec<Instruction>,
    HashMap<(u8, usize, usize), HashMap<Direction, ((u8, usize, usize), Direction)>>,
    (u8, usize, usize),
    HashMap<(u8, usize, usize), (usize, usize)>
) {

    let mut board_rows = Vec::new();
    for line in &lines[0..(lines.len() - 1)] {
        let row: Vec<BoardSquare> = line.chars().map(|c| {
            match c {
                ' ' => Void,
                '.' => Open,
                '#' => Wall,
                _ => panic!("Board wat: '{}'", c),
            }
        }).collect();
        board_rows.push(row);
    }

    let mut reachable = HashSet::new();
    for row in 0..board_rows.len() {
        for col in 0..board_rows[row].len() {
            if board_rows[row][col] == Open {
                reachable.insert((row, col));
            }
        }
    }

    let wraps: HashMap<(Direction, u8), (Direction, u8)> = [
        ((Up, 0), (Right, 5)),
        ((Down, 0), (Down, 2)),
        ((Left, 0), (Right, 3)),
        ((Right, 0), (Right, 1)),
        ((Up, 1), (Up, 5)),
        ((Down, 1), (Left, 2)),
        ((Left, 1), (Left, 0)),
        ((Right, 1), (Left, 4)),
        ((Up, 2), (Up, 0)),
        ((Down, 2), (Down, 4)),
        ((Left, 2), (Down, 3)),
        ((Right, 2), (Up, 1)),
        ((Up, 3), (Right, 2)),
        ((Down, 3), (Down, 5)),
        ((Left, 3), (Right, 0)),
        ((Right, 3), (Right, 4)),
        ((Up, 4), (Up, 2)),
        ((Down, 4), (Left, 5)),
        ((Left, 4), (Left, 3)),
        ((Right, 4), (Left, 1)),
        ((Up, 5), (Up, 3)),
        ((Down, 5), (Down, 1)),
        ((Left, 5), (Down, 0)),
        ((Right, 5), (Up, 4)),
    ].into_iter().collect();

    let mut orig_idx = HashMap::new();
    let mut invert_orig = HashMap::new();
    let mut board = HashMap::new();
    for (row, col) in &reachable {
        let mut links: HashMap<Direction, ((u8, usize, usize), Direction)> = HashMap::new();
        let face = get_face(row, col);
        let leftmost = (col / 50) * 50;
        let rightmost = leftmost + 49;
        let top = (row / 50) * 50;
        let bottom = top + 49;
        let local_row = row % 50;
        let local_col = col % 50;

        orig_idx.insert((*row, *col), (face, local_row, local_col));
        invert_orig.insert((face, local_row, local_col), (*row, *col));

        [Up, Down, Right, Left].into_iter()
            .for_each(|dir| {
                let next_coord = match dir {
                    Up if *row == top => {
                        let (new_dir, new_face) = wraps.get(&(Up, face)).unwrap().clone();
                        match new_dir {
                            Up => ((new_face, 49, local_col), Up),
                            Right => ((new_face, local_col, 0), Right),
                            Left | Down => panic!("Doesn't"),
                        }
                    }
                    Up => ((face, local_row - 1, local_col), Up),
                    Left if *col == leftmost => {
                        let (new_dir, new_face) = wraps.get(&(Left, face)).unwrap().clone();
                        match new_dir {
                            Left => ((new_face, local_row, 49), Left),
                            Right => ((new_face, 49 - local_row, 0), Right),
                            Down => ((new_face, 0, local_row), Down),
                            Up => panic!("Doesn't"),
                        }
                    }
                    Left => ((face, local_row, local_col - 1), Left),
                    Down if *row == bottom => {
                        let (new_dir, new_face) = wraps.get(&(Down, face)).unwrap().clone();
                        match new_dir {
                            Left => ((new_face, local_col, 49), Left),
                            Down => ((new_face, 0, local_col), Down),
                            Up | Right => panic!("Doesn't"),
                        }
                    }
                    Down => ((face, local_row + 1, local_col), Down),
                    Right if *col == rightmost => {
                        let (new_dir, new_face) = wraps.get(&(Right, face)).unwrap().clone();
                        match new_dir {
                            Left => ((new_face, 49 - local_row, 49), Left),
                            Right => ((new_face, local_row, 0), Right),
                            Up => ((new_face, 49, local_row), Up),
                            Down => panic!("Doesn't"),
                        }
                    }
                    Right => ((face, local_row, local_col + 1), Right)
                };

                links.insert(dir, next_coord);
            });
        board.insert((face, local_row, local_col), links);
    }

    for orig_coord in &reachable {
        let mapped = orig_idx.get(orig_coord).unwrap();
        let links = board.get_mut(mapped).unwrap();
        let dirs_to_remove: Vec<Direction>= links.iter()
            .filter(|(_, (coord, _))| !invert_orig.contains_key(coord))
            .map(|(dir, _)| dir.clone())
            .collect();
        for dir in &dirs_to_remove {
            links.remove(dir);
        }
    }

    let mut idx = 0;
    let start = loop {
        if board_rows[0][idx] == Open {
            break orig_idx.get(&(0, idx)).unwrap().clone();
        }
        idx += 1;
    };

    let instruction_line = lines.last().unwrap();
    let number_re = Regex::new("\\d+").unwrap();
    let direction_re = Regex::new("[LR]").unwrap();
    let numbers: Vec<usize> = direction_re
        .split(instruction_line)
        .map(|n| usize::from_str(n).unwrap())
        .collect();
    let turns: Vec<&str> = number_re.split(instruction_line)
        .filter(|s| !s.is_empty())
        .collect();

    let mut instructions = Vec::new();
    for idx in 0..turns.len() {
        let dir = match turns[idx] {
            "L" => LeftTurn,
            "R" => RightTurn,
            a => panic!("Uhhh: -'{}'- ", a),
        };
        instructions.push(Forward(numbers[idx]));
        instructions.push(dir);
    }
    instructions.push(Forward(*numbers.last().unwrap()));

    (instructions, board, start, invert_orig)
}

fn get_face(row: &usize, col: &usize) -> u8 {
    match (row / 50, col / 50) {
        (0, 1) => 0,
        (0, 2) => 1,
        (1, 1) => 2,
        (2, 0) => 3,
        (2, 1) => 4,
        (3, 0) => 5,
        _ => panic!("Unknown Face!"),
    }
}
