use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::Cycle;
use std::slice::Iter;
use std::time::Instant;
use crate::Direction::{Left, Right};
use crate::Piece::{Ell, HBar, Plus, Square, VBar};

fn main() {
    let raw_input = fs::read_to_string("input.txt").unwrap();
    let input = parse_input(raw_input);
    let pieces = [HBar, Plus, Ell, VBar, Square];
    let start = Instant::now();
    let answer_1 = solve_1(input.clone().iter().cycle(), pieces.clone().iter().cycle());
    let end = Instant::now();
    println!("Part 1: {} in {}us", answer_1, end.duration_since(start).as_micros());

    let start = Instant::now();
    let cycle_len = input.len();
    let answer_2 = solve_2(input.iter().cycle(), pieces.iter().cycle(), cycle_len);
    let end = Instant::now();
    println!("Part 2: {} in {}us", answer_2, end.duration_since(start).as_micros());
}

fn solve_1(mut jets: Cycle<Iter<Direction>>, mut pieces: Cycle<Iter<Piece>>) -> usize {
    let mut board = [[false; 7]; 2022 * 4].to_vec();
    let mut max_y = 0;
    for _ in 0..2022 {
        let piece = pieces.next().unwrap();

        let mut x = 2;
        let mut y = max_y + 2 + piece.max_height();
        loop {
            let direction = jets.next().unwrap();
            // print_board(&board, max_y, Some((x, y, piece.clone())));
            // stdin().read(&mut [0]).unwrap();
            // println!("{:?}, {}", direction, max_y);

            let (new_x, new_y, rested) = turn(&board, x, y, &piece, &direction);
            if rested {
                for delta_x in 0..piece.width() {
                    for delta_y in 0..piece.max_height() {
                        board[new_y - delta_y][new_x + delta_x] |= piece.shape()[delta_y][delta_x];
                    }
                }

                max_y = max_y.max(new_y + 1);
                // print_board(&board, max_y, None);
                // println!();
                break;
            }

            x = new_x;
            y = new_y;
        }
    }

    // print_board(&board, max_y, None);
    max_y
}

const ROUNDS: usize = 1_000_000_000_000;

fn solve_2(mut jets: Cycle<Iter<Direction>>, mut pieces: Cycle<Iter<Piece>>, dir_cycle_len: usize) -> usize {
    let mut board = [[false; 7]; 1_000_000];
    let mut seen: HashMap<BoardState, (usize, usize)> = HashMap::new();
    let mut dir_cycle_idx = 0;

    let mut max_y = 0;
    for stone in 0.. {
        let piece = pieces.next().unwrap();

        let mut x = 2;
        let mut y = max_y + 2 + piece.max_height();
        loop {
            let direction = jets.next().unwrap();
            let (new_x, new_y, rested) = turn(&board, x, y, &piece, &direction);
            if rested {
                for delta_x in 0..piece.width() {
                    for delta_y in 0..piece.max_height() {
                        board[new_y - delta_y][new_x + delta_x] |= piece.shape()[delta_y][delta_x];
                    }
                }

                max_y = max_y.max(new_y + 1);
                break;
            }

            x = new_x;
            y = new_y;
            dir_cycle_idx = (dir_cycle_idx + 1) % dir_cycle_len;
        }

        let reachable_depth = get_board_reachable_depth(&board, max_y);
        let state_key = BoardState::new(reachable_depth, piece.clone(), dir_cycle_idx);
        if let Some((seen_cycle, max_y_then)) = seen.get(&state_key) {
            // if stone <= 44212 {
            //     continue;
            // }
            println!("Cycle detected at {}, max_y: {}. Previously seen at {}, max_y {}",
                     stone, max_y, seen_cycle, max_y_then);
            print_board(&board, max_y, None);
            let turns_after_cycle = ROUNDS - seen_cycle;
            let growth_in_cycle = max_y - max_y_then;
            let cycle_len = stone - seen_cycle;
            let times_cycled = turns_after_cycle / cycle_len;
            let times_post_cycle = turns_after_cycle % cycle_len - 1;

            let mut additional = 0;
            let mut last_y = max_y;
            for _ in 0..times_post_cycle {
                let piece = pieces.next().unwrap();

                let mut x = 2;
                let mut y = max_y + 2 + piece.max_height();
                loop {
                    let direction = jets.next().unwrap();
                    let (new_x, new_y, rested) = turn(&board, x, y, &piece, &direction);
                    if rested {
                        for delta_x in 0..piece.width() {
                            for delta_y in 0..piece.max_height() {
                                board[new_y - delta_y][new_x + delta_x] |= piece.shape()[delta_y][delta_x];
                            }
                        }

                        max_y = max_y.max(new_y + 1);
                        additional += max_y - last_y;
                        last_y = max_y;
                        break;
                    }

                    x = new_x;
                    y = new_y;
                    dir_cycle_idx = (dir_cycle_idx + 1) % dir_cycle_len;
                }
            }

            println!("Height: {}", max_y_then + times_cycled * growth_in_cycle + additional);
            break;
        } else {
            seen.insert(state_key, (stone, max_y));
        }
    }

    max_y
}

#[derive(Eq, PartialEq, Hash)]
struct BoardState {
    reachable_board: Vec<[bool; 7]>,
    piece: Piece,
    dir_cycle_idx: usize
}

impl BoardState {
    fn new(reachable_board: Vec<[bool; 7]>, piece: Piece, dir_cycle_idx: usize) -> BoardState {
        BoardState {
            reachable_board,
            piece,
            dir_cycle_idx,
        }
    }
}

fn get_board_reachable_depth(board: &[[bool; 7]], max_y: usize) -> Vec<[bool; 7]> {
    let mut min_y = max_y;
    let mut explored = HashSet::new();
    for x in 0..7 {
        min_y = min_y.min(explore(board, x, max_y, &mut explored));
    }

    let mut result = Vec::new();
    for y in min_y..=max_y {
        result.push(board[y].clone())
    }

    result
}

fn explore(board: &[[bool; 7]], x: usize, y: usize, explored: &mut HashSet<(usize, usize)>) -> usize {
    if explored.contains(&(x, y)) {
        return usize::MAX;
    }

    explored.insert((x, y));

    let mut lowest_y = y;
    if y > 0 && !board[y - 1][x] {
        lowest_y = lowest_y.min(explore(board, x, y - 1, explored))
    }

    if x > 0 && !board[y][x - 1] {
        lowest_y = lowest_y.min(explore(board, x - 1, y, explored))
    }

    if x < 6 && !board[y][x + 1] {
        lowest_y = lowest_y.min(explore(board, x + 1, y, explored))
    }

    lowest_y
}
fn turn(
    board: &[[bool; 7]], x: usize, y: usize, piece: &Piece, dir: &Direction,
) -> (usize, usize, bool) {
    let new_x = match dir {
        Left if x > 0 => {
            let mut collision = false;
            for delta_y in 0..piece.max_height() {
                for delta_x in 0..piece.width() {
                    if piece.shape()[delta_y][delta_x] && board[y - delta_y][x + delta_x - 1] {
                        collision = true;
                        break;
                    }
                }
            }

            if collision {
                x
            } else {
                x - 1
            }
        }
        Right if x + piece.width() < 7 => {
            let mut collision = false;
            for delta_y in 0..piece.max_height() {
                for delta_x in 0..piece.width() {
                    if piece.shape()[delta_y][delta_x] && board[y - delta_y][x + delta_x + 1] {
                        collision = true;
                        break;
                    }
                }
            }

            if collision {
                x
            } else {
                x + 1
            }
        }
        _ => x
    };

    if y < piece.max_height() {
        return (new_x, y, true);
    }

    let height_map = piece.height_map();
    for idx in 0..piece.width() {
        if board[y - height_map[idx]][new_x + idx] {
            return (new_x, y, true);
        }
    }

    (new_x, y - 1, false)
}

fn print_board(board: &[[bool; 7]], max_y: usize, piece: Option<(usize, usize, Piece)>) {
    match piece {
        None => {
            for y in ((0.max(max_y as i64 - 20) as usize)..=max_y).rev() {
                board[y].iter().for_each(|s| if *s { print!("#") } else { print!(".") });
                println!();
            }
        }
        Some((piece_x, piece_y, piece)) => {
            for y in ((0.max(max_y as i64 - 20) as usize)..=max_y.max(piece_y)).rev() {
                if ((piece_y + 1 - piece.max_height())..=piece_y).contains(&y) {
                    let piece_row = piece_y - y;
                    for x in 0..7 {
                        if board[y][x] {
                            print!("#")
                        } else if (piece_x..(piece_x + piece.width())).contains(&x) {
                            if piece.shape()[piece_row][x - piece_x] {
                                print!("@")
                            } else {
                                print!(".")
                            }
                        } else {
                            print!(".");
                        }
                    }
                } else {
                    board[y].iter().for_each(|s| if *s { print!("#") } else { print!(".") });
                }
                println!();
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Piece {
    Plus,
    HBar,
    VBar,
    Square,
    Ell,
}

impl Piece {
    fn width(&self) -> usize {
        match self {
            Plus => 3,
            HBar => 4,
            Square => 2,
            Ell => 3,
            VBar => 1,
        }
    }

    fn max_height(&self) -> usize {
        match self {
            Plus => 3,
            HBar => 1,
            VBar => 4,
            Square => 2,
            Ell => 3,
        }
    }

    fn height_map(&self) -> Vec<usize> {
        match self {
            Plus => vec![2, 3, 2],
            HBar => vec![1, 1, 1, 1],
            Square => vec![2, 2],
            Ell => vec![3, 3, 3],
            VBar => vec![4]
        }
    }

    fn shape(&self) -> Vec<Vec<bool>> {
        match self {
            Plus => vec![
                vec![false, true, false],
                vec![true, true, true],
                vec![false, true, false],
            ],
            HBar => vec![vec![true, true, true, true]],
            Square => vec![
                vec![true, true],
                vec![true, true],
            ],
            Ell => vec![
                vec![false, false, true],
                vec![false, false, true],
                vec![true, true, true],
            ],
            VBar => vec![
                vec![true],
                vec![true],
                vec![true],
                vec![true],
            ]
        }
    }
}

#[derive(Clone, Debug)]
enum Direction {
    Left,
    Right,
}

fn parse_input(input: String) -> Vec<Direction> {
    input.chars().map(|c| if c == '>' { Right } else { Left }).collect()
}
