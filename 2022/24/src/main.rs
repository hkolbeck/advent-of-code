use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
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
    let answer_1 = solve_1(input.clone());
    let end = Instant::now();
    println!("Part 1: {} in {}us", answer_1, end.duration_since(start).as_micros());

    let start = Instant::now();
    let answer_2 = solve_2(input);
    let end = Instant::now();
    println!("Part 2: {} in {}us", answer_2, end.duration_since(start).as_micros());
}

fn solve_1(board: Vec<Vec<Vec<Blizzard>>>) -> usize {
    println!("Parsed!");

    let states = get_board_states(&board);
    println!("States generated!");
    let max_row = states[0].len() - 1;
    let max_col = states[0][0].len() - 1;
    let state_cnt = states.len();

    let graph = states_to_graph(&states);
    println!("Graph generated!");

    let (shortest, _) = find_shortest(&graph, (1, 0, 0), (max_row, max_col), state_cnt).unwrap();
    shortest
}

// High: 947
// High: 946
fn solve_2(board: Vec<Vec<Vec<Blizzard>>>) -> usize {
    println!("Parsed!");

    let states = get_board_states(&board);
    println!("States generated!");

    let graph = states_to_graph(&states);
    println!("Graph generated!");

    let max_row = states[0].len() - 1;
    let max_col = states[0][0].len() - 1;
    let state_cnt = states.len();


    let (shortest_there, first_stop) =
        find_shortest(&graph, (1, 0, 0), (max_row, max_col), state_cnt).unwrap();
    println!("There: {}, {}", shortest_there, first_stop);

    let (shortest_back, second_stop) =
        find_shortest(&graph, (first_stop, max_row, max_col), (0, 0), state_cnt).unwrap();
    println!("Back: {}, {}", shortest_back, second_stop);

    let (shortest_there_again, _) =
        find_shortest(&graph, (second_stop, 0, 0), (max_row, max_col), state_cnt).unwrap();
    println!("There again: {}", shortest_there_again);

    println!("{} -> {} -> {}", shortest_there, shortest_back, shortest_there_again);
    shortest_there + shortest_back + shortest_there_again + 2
}

fn _print_board_state(state: &Vec<Vec<bool>>) {
    for row in state {
        for space in row {
            if *space {
                print!(" ");
            } else {
                print!("#")
            }
        }
        println!()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct SearchState {
    key: (usize, usize, usize),
    cost: usize,
}

impl PartialOrd<Self> for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn find_shortest(
    graph: &HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>>,
    start: (usize, usize, usize),
    end: (usize, usize),
    states: usize,
) -> Option<(usize, usize)> {
    let distances = dijkstra(graph, start);

    let mut shortest = usize::MAX;
    let mut shortest_state = usize::MAX;
    for state in 0..states {
        if let Some(dist) = distances.get(&(state, end.0, end.1)) {
            if *dist < shortest {
                shortest = *dist;
                shortest_state = state;
            }
        }
    }

    if shortest == usize::MAX {
        None
    } else {
        Some((shortest + 1, shortest_state))
    }
}

fn dijkstra(
    graph: &HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>>,
    start: (usize, usize, usize),
) -> HashMap<(usize, usize, usize), usize> {
    let mut dist: HashMap<(usize, usize, usize), usize> = graph.keys()
        .map(|k| (k.clone(), usize::MAX))
        .collect();
    println!("Graph has {} nodes", dist.len());
    let mut heap = BinaryHeap::new();

    dist.insert(start.clone(), 1);
    heap.push(SearchState {
        key: start,
        cost: 1,
    });

    let mut peep = 1;
    while let Some(SearchState { key, cost }) = heap.pop() {
        if peep % 10_000_000 == 0 {
            println!("Pop: {} Heap size: {}", peep, heap.len());
        }
        peep += 1;
        if &cost > dist.get(&key).unwrap() {
            continue;
        }

        for neighbor in graph.get(&key).unwrap_or(&vec![]) {
            let next = SearchState { cost: cost + 1, key: neighbor.clone() };
            if &next.cost < dist.get(&neighbor).unwrap_or(&usize::MAX) {
                heap.push(next);
                dist.insert(neighbor.clone(), cost + 1);
            }
        }
    }

    dist
}

fn states_to_graph(
    states: &Vec<Vec<Vec<bool>>>
) -> HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>> {
    let max_row = states[0].len() as i32 - 1;
    let max_col = states[0][0].len() as i32 - 1;
    let mut graph = HashMap::new();
    for state in 0..states.len().min(1000) {
        let next_state = (state + 1) % states.len();
        for row in 0..states[state].len() {
            for col in 0..states[state][0].len() {
                let connections: Vec<(usize, usize, usize)> =
                    [(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)].into_iter()
                        .filter_map(|(delta_row, delta_col)| {
                            let new_row = row as i32 + delta_row;
                            let new_col = col as i32 + delta_col;

                            if new_row < 0 || new_row > max_row || new_col < 0 || new_col > max_col {
                                None
                            } else if states[next_state][new_row as usize][new_col as usize] {
                                Some((new_row as usize, new_col as usize))
                            } else {
                                None
                            }
                        }).map(|(new_row, new_col)| (next_state, new_row, new_col))
                        .collect();

                graph.insert((state, row, col), connections);
            }
        }
    }

    graph
}

fn get_board_states(init: &Vec<Vec<Vec<Blizzard>>>) -> Vec<Vec<Vec<bool>>> {
    let mut states = Vec::new();
    let state_count = init.len() * init[0].len();

    let start = init.clone();

    let mut board = init.clone();
    for _ in 0..state_count {
        let mut state = Vec::new();
        for row in 0..board.len() {
            let mut row_vec = Vec::new();
            for col in 0..board[0].len() {
                row_vec.push(board[row][col].is_empty());
            }
            state.push(row_vec);
        }
        states.push(state);

        board = advance_board(board);
    }

    if board != start {
        panic!("No cycles detected");
    }

    states
}

fn advance_board(board: Vec<Vec<Vec<Blizzard>>>) -> Vec<Vec<Vec<Blizzard>>> {
    let last_row = board.len() - 1;
    let last_col = board[0].len() - 1;

    let mut new_board = Vec::new();
    for _ in 0..=last_row {
        let mut row = Vec::new();
        for _ in 0..=last_col {
            row.push(vec![]);
        }
        new_board.push(row);
    }

    for row in 0..=last_row {
        for col in 0..=last_col {
            for blizzard in &board[row][col] {
                let mut next_row = row as i32 + blizzard.delta_row;
                if next_row < 0 {
                    next_row = last_row as i32;
                } else if next_row > last_row as i32 {
                    next_row = 0;
                }

                let mut next_col = col as i32 + blizzard.delta_col;
                if next_col < 0 {
                    next_col = last_col as i32;
                } else if next_col > last_col as i32 {
                    next_col = 0;
                }

                new_board[next_row as usize][next_col as usize].push(blizzard.clone());
            }
        }
    }

    new_board
}


#[derive(Clone, Eq, PartialEq, Hash)]
struct Blizzard {
    delta_row: i32,
    delta_col: i32,
}

impl Blizzard {
    fn new(delta_row: i32, delta_col: i32) -> Blizzard {
        Blizzard {
            delta_row,
            delta_col,
        }
    }
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<Vec<Blizzard>>> {
    let mut board = Vec::new();
    lines.iter().skip(1).for_each(|line| {
        if line.starts_with("##") {
            return;
        }

        let mut board_line = Vec::new();
        line.chars().skip(1).for_each(|c| {
            match c {
                '.' => board_line.push(vec![]),
                '^' => board_line.push(vec![Blizzard::new(-1, 0)]),
                '<' => board_line.push(vec![Blizzard::new(0, -1)]),
                '>' => board_line.push(vec![Blizzard::new(0, 1)]),
                'v' => board_line.push(vec![Blizzard::new(1, 0)]),
                '#' => {}
                _ => panic!("Wat: {}", c),
            };
        });

        board.push(board_line);
    });

    board
}
