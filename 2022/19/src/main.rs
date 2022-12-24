use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::str::FromStr;
use std::time::Instant;
use regex::Regex;
use crate::Resource::{Clay, Geodes, Obsidian, Ore};

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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geodes,
}

static RESOURCES: [Resource; 4] = [Ore, Clay, Obsidian, Geodes];

#[derive(Debug, Clone)]
struct BluePrint {
    number: usize,
    costs: HashMap<Resource, HashMap<Resource, usize>>,
    maxes: HashMap<Resource, usize>,
}

fn solve_1(blueprints: Vec<BluePrint>) -> usize {
    blueprints.iter()
        .map(|bp| {
            let start = Instant::now();
            let (max_geodes, path) = get_best(bp, 24);
            println!("{}: {} in {}us: {:?}",
                     bp.number, max_geodes, Instant::now().duration_since(start).as_micros(), path);

            bp.number * max_geodes
        }).sum()
}

fn solve_2(blueprints: Vec<BluePrint>) -> usize {
    blueprints.iter().take(3)
        .map(|bp| {
            let start = Instant::now();
            let (max_geodes, path) = get_best(bp, 32);
            println!("{}: {} in {}us: {:?}",
                     bp.number, max_geodes, Instant::now().duration_since(start).as_micros(), path);

            max_geodes
        }).product()
}

fn get_best(bp: &BluePrint, minutes: usize) -> (usize, Vec<(Resource, usize)>) {
    let on_hand: HashMap<Resource, usize> =
[(Ore, 0), (Clay, 0), (Obsidian, 0), (Geodes, 0)].into_iter().collect();
    let robots: HashMap<Resource, usize> =
[(Ore, 1), (Clay, 0), (Obsidian, 0), (Geodes, 0)].into_iter().collect();
    let mut best_found = Box::new(0);
    let mut best_path_found = Box::new(vec![]);

    get_best_re(bp, on_hand, robots, &mut best_found, &mut best_path_found, minutes, 1, vec![]);
    (*best_found, *best_path_found)
}

fn get_best_re(
    bp: &BluePrint,
    on_hand: HashMap<Resource, usize>,
    robots: HashMap<Resource, usize>,
    best_found: &mut Box<usize>,
    best_path_found: &mut Box<Vec<(Resource, usize)>>,
    minutes: usize,
    minute: usize,
    path: Vec<(Resource, usize)>,
) {
    if minute == minutes {
        let geodes = *on_hand.get(&Geodes).unwrap() + *robots.get(&Geodes).unwrap();
        if best_found < &mut Box::new(geodes) {
            **best_found = geodes;
            **best_path_found = path;
        }
        return;
    }

    if get_max_producible(
        *on_hand.get(&Geodes).unwrap(),
        *robots.get(&Geodes).unwrap(),
        minute,
        minutes
    ) < **best_found {
        return;
    }

    for resource in &RESOURCES {
        if robots.get(resource).unwrap() >= bp.maxes.get(resource).unwrap() {
            continue;
        }

        let can_eventually_build = bp.costs.get(resource).unwrap().iter()
            .all(|(r, c)| c == &0 || robots.get(r).unwrap() > &0);
        if !can_eventually_build {
            continue;
        }

        let mut min = minute;
        let mut oh = on_hand.clone();

        while min <= minutes && !can_buy(bp, &oh, resource) {
            for res in &RESOURCES {
                let extracted = robots.get(res).unwrap();
                let has = oh.get_mut(res).unwrap();
                *has += extracted;
            }
            min += 1;
        }

        if min < minutes {
            let mut p = path.clone();
            p.push((resource.clone(), min));
            let mut ro = robots.clone();
            *ro.get_mut(resource).unwrap() += 1;
            for (r, cost) in bp.costs.get(resource).unwrap() {
                *oh.get_mut(r).unwrap() -= cost;
            }

            for res in &RESOURCES {
                let extracted = robots.get(res).unwrap();
                let has = oh.get_mut(res).unwrap();
                *has += extracted;
            }

            get_best_re(
                bp,
                oh,
                ro,
                best_found,
                best_path_found,
                minutes,
                min + 1,
                p,
            );
        } else {
            let geodes = *oh.get(&Geodes).unwrap();
            if best_found < &mut Box::new(geodes) {
                **best_found = geodes;
                **best_path_found = path.clone();
            }
        }
    }
}

fn get_max_producible(on_hand: usize, geode_bots: usize, turn: usize, max_turns: usize) -> usize {
    let remaining = max_turns - turn + 1;
    on_hand + (geode_bots * remaining) + (remaining * remaining + remaining) / 2
}

fn can_buy(
    bp: &BluePrint,
    on_hand: &HashMap<Resource, usize>,
    resource: &Resource,
) -> bool {
    let my_costs = bp.costs.get(resource).unwrap();
    for res in &RESOURCES {
        if my_costs.get(res).unwrap_or(&0) > on_hand.get(res).unwrap() {
            return false;
        }
    }

    true
}

fn parse_input(lines: Vec<String>) -> Vec<BluePrint> {
    let regex = Regex::new("(\\d+)").unwrap();
    lines.iter()
        .map(|l| regex.find_iter(l.as_str())
            .map(|m| usize::from_str(m.as_str()).unwrap()))
        .map(|mut nums| {
            BluePrint {
                number: nums.next().unwrap(),
                costs: [
                    (Ore, [(Ore, nums.next().unwrap())].into_iter().collect()),
                    (Clay, [(Ore, nums.next().unwrap())].into_iter().collect()),
                    (Obsidian, [(Ore, nums.next().unwrap()), (Clay, nums.next().unwrap())].into_iter().collect()),
                    (Geodes, [(Ore, nums.next().unwrap()), (Obsidian, nums.next().unwrap())].into_iter().collect())
                ].into_iter().collect(),
                maxes: [(Ore, 0), (Clay, 0), (Obsidian, 0), (Geodes, 0)].into_iter().collect(),
            }
        })
        .map(|mut bp| {
            bp.costs.iter().for_each(|(_, robot_costs)| {
                for resource in &RESOURCES {
                    let max = bp.maxes.get_mut(resource).unwrap();
                    let mut cost = *robot_costs.get(resource).unwrap_or(&0);
                    *max = *max.max(&mut cost);
                }
            });
            bp.maxes.insert(Geodes, usize::MAX);
            bp
        }).collect()
}
