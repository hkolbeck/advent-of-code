use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;
use std::time::Instant;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let raw_input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<String> = raw_input.split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    let (graph, pressures) = parse_input(lines);

    let start = Instant::now();
    let answer_1 = solve_1(graph.clone(), pressures.clone());
    let end = Instant::now();
    println!("Part 1: {} in {}us", answer_1, end.duration_since(start).as_micros());

    let start = Instant::now();
    let answer_2 = solve_2(graph, pressures);
    let end = Instant::now();
    println!("Part 2: {} in {}us", answer_2, end.duration_since(start).as_micros());
}

fn solve_1(graph: HashMap<String, Vec<String>>, pressures: HashMap<String, usize>) -> usize {
    let start = String::from("AA");
    let distances = distances(&graph);
    let has_pressure = pressures.iter()
        .filter(|(_, p)| p > &&0)
        .map(|(v, _)| v)
        .collect();

    search(0, 30, &start, &has_pressure, &pressures, &distances, &mut HashSet::new())
}

fn solve_2(graph: HashMap<String, Vec<String>>, pressures: HashMap<String, usize>) -> usize {
    let start = String::from("AA");
    let distances = distances(&graph);
    let has_pressure: HashSet<String> = pressures.iter()
        .filter(|(_, p)| p > &&0)
        .map(|(v, _)| v.clone())
        .collect();
    let mut c = 0;
    let mut max = 0;
    has_pressure.iter().combinations(8)
        .for_each(|you| {
            let elephant: Vec<&String> = has_pressure.iter()
                .filter(|v| !you.contains(v))
                .collect();

            let you_released = search(
                0,
                26,
                &start,
                &you,
                &pressures,
                &distances,
                &mut HashSet::new()
            );
            let ele_released = search(
                0,
                26,
                &start,
                &elephant,
                &pressures,
                &distances,
                &mut HashSet::new()
            );

            c += 1;
            if c % 25 == 0 {
                println!("{}, max: {}", c, max);
            }

            max = max.max(ele_released + you_released)
        });

    max
}

fn search<'a>(
    minute: usize,
    max_minute: usize,
    at: &'a String,
    have_pressure: &'a Vec<&String>,
    pressures: &HashMap<String, usize>,
    distances: &HashMap<(&String, &String), usize>,
    open: &mut HashSet<&'a String>,
) -> usize {
    open.insert(at);

    let pressure = pressures.get(at).unwrap();
    let mut max = 0;
    for next in have_pressure {
        let dist = distances.get(&(at, next)).unwrap();
        if !open.contains(next) && minute + dist < max_minute {
            let result = search(
                minute + dist + 1,
                max_minute,
                next,
                have_pressure,
                pressures,
                distances,
                open,
            );

            max = max.max(result)
        }
    }

    open.remove(at);
    max + pressure * (max_minute - minute)
}

fn distances(graph: &HashMap<String, Vec<String>>) -> HashMap<(&String, &String), usize> {
    let mut node_dists = HashMap::new();
    for start in graph.keys() {
        let mut queue: HashSet<&String> = HashSet::new();
        let mut dists = HashMap::new();

        for node in graph.keys() {
            dists.insert(node, usize::MAX);
            queue.insert(node);
        }
        dists.insert(start, 0);

        while !queue.is_empty() {
            let node = queue.iter()
                .min_by(|v1, v2| dists.get(**v1).unwrap().cmp(dists.get(**v2).unwrap()))
                .unwrap().clone();
            queue.remove(node);

            for neighbor in graph.get(node).unwrap() {
                if queue.contains(neighbor) {
                    let maybe = dists.get(node).unwrap() + 1;
                    if &maybe < dists.get(neighbor).unwrap() {
                        dists.insert(neighbor, maybe);
                    }
                }
            }
        }

        for (other_node, dist) in dists {
            node_dists.insert((start, other_node), dist);
        }
    }

    node_dists
}

fn parse_input(lines: Vec<String>) -> (HashMap<String, Vec<String>>, HashMap<String, usize>) {
    let regex = Regex::new("Valve (..) has flow rate=(\\d+); tunnels? leads? to valves? (.*)").unwrap();
    let mut graph = HashMap::new();
    let mut pressures = HashMap::new();
    for line in lines {
        let captures = regex.captures(line.as_str()).unwrap();
        let name = String::from(captures.get(1).unwrap().as_str());
        let flow_rate = usize::from_str(captures.get(2).unwrap().as_str()).unwrap();
        let connected = captures.get(3).unwrap().as_str().split(", ").map(String::from).collect();

        graph.insert(name.clone(), connected);
        pressures.insert(name, flow_rate);
    }

    (graph, pressures)
}
