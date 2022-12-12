use std::{fs, thread};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let raw_input = fs::read_to_string("input.txt").unwrap();
    let mut map: Vec<Vec<u8>> = raw_input.split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| format!("z{}z", s))
        .map(|s| s.chars().map(|c| c as u8).collect())
        .collect();
    map.push(Vec::from(['z' as u8; 79]));
    map.insert(0, Vec::from(['z' as u8; 79]));

    let you = find_you(&mut map);

    let builder = thread::Builder::new()
        .name("reductor".into())
        .stack_size(6 * 1024 * 1024 * 1024);

    let handler = builder.spawn(move || {
        let start = Instant::now();
        let score = solve_1(&map, you);
        let end = Instant::now();
        println!("Part 1: {} in {}us", score, end.duration_since(start).as_micros());

        let start = Instant::now();
        let score = solve_2(&map);
        let end = Instant::now();
        println!("Part 2: {} in {}us", score, end.duration_since(start).as_micros());
    }).unwrap();

    handler.join().unwrap();
}

fn find_you(map: &mut Vec<Vec<u8>>) -> (usize, usize) {
    for row in 0..map.len() {
        if map[row][1] == 'S' as u8 {
            map[row][1] = 'a' as u8;
            return (row, 1);
        }
    }

    panic!("No you found!");
}

fn compare_pos(pos_1: Option<u64>, pos_2: Option<u64>) -> Ordering {
    match pos_1 {
        None => match pos_2 {
            None => Ordering::Equal,
            Some(_) => Ordering::Greater,
        }
        Some(v1) => match pos_2 {
            None => Ordering::Less,
            Some(v2) => v1.cmp(&v2),
        }
    }
}

fn djikstra(map: &Vec<Vec<u8>>, you: (usize, usize)) -> Option<u64> {
    let mut dist: Vec<Vec<Option<u64>>> = (0..map.len()).map(|_| [None; 79].to_vec()).collect();
    let mut prev: Vec<Vec<Option<(usize, usize)>>> = (0..map.len()).map(|_| [None; 79].to_vec()).collect();
    let mut queue: Vec<(usize, usize)> = Vec::new();

    dist[you.0][you.1] = Some(0);
    for row in 1..map.len() - 1 {
        for col in 1..map[0].len() - 1 {
            queue.push((row, col));
        }
    }

    let mut goal: Option<(usize, usize)> = None;
    while !queue.is_empty() {
        let loc = queue.iter().min_by(|(row_1, col_1), (row_2, col_2)| {
            compare_pos(dist[*row_1][*col_1], dist[*row_2][*col_2])
        }).unwrap().clone();
        queue.remove(queue.iter().position(|t| t == &loc).unwrap());

        [(1, 0), (0, 1), (-1, 0), (0, -1)].iter().for_each(|delta| {
            let row = (loc.0 as i32 + delta.0) as usize;
            let col = (loc.1 as i32 + delta.1) as usize;
            // println!("({}, {}): {} -> {}", row, col, map[loc.0][loc.1], map[row][col]);

            let height = if map[row][col] == 'E' as u8 {
                goal = Some((row.clone(), col.clone()));
                'z' as u8
            } else {
                map[row][col]
            };

            if height <= map[loc.0][loc.1] + 1 && queue.contains(&(row, col)){
                let alt = dist[loc.0][loc.1].map(|v| v + 1);
                if compare_pos(alt, dist[row][col]) == Ordering::Less {
                    dist[row][col] = alt;
                    prev[row][col] = Some(loc.clone());
                }
            }
        });

        if goal.is_some() && !queue.contains(&goal.unwrap()) {
            let goal = goal.unwrap();
            return dist[goal.0][goal.1];
        }
    }

    None
}

fn solve_1(map: &Vec<Vec<u8>>, you: (usize, usize)) -> u64 {
    djikstra(map, you).unwrap()
}

fn solve_2(map: &Vec<Vec<u8>>) -> u64 {
    let mut dists: HashMap<(usize, usize), u64> = HashMap::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'a' as u8 {
                djikstra(map, (row, col)).map(|dist| dists.insert((row, col), dist));
            }
        }
    }

    let (_, dist) = dists.iter()
        .filter(|(_, v)| **v != 0)
        .min_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();
    *dist
}