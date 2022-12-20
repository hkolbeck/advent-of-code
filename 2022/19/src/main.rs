use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;
use regex::Regex;
use crate::Resource::{Clay, Geodes, Obsidian, Ore};

fn main() {
    let raw_input = fs::read_to_string("mini-input.txt").unwrap();
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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
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
            let paths = make_paths(bp);
            println!("Generated {} paths", paths.len());
            let mut max_geodes = 0;
            for path in paths {
                let geodes = test_strategy(bp, &path, 24);
                if geodes > max_geodes {
                    println!("{}: New Max: {} - {:?}", bp.number, geodes, path);
                }
                max_geodes = max_geodes.max(geodes);
            }
            // println!("{}: {} ({})- {:?} in {}us", bp.number, max_geodes, checked, path, Instant::now().duration_since(start).as_micros());

            bp.number * max_geodes
        }).sum()
}

fn make_paths(bp: &BluePrint) -> Vec<Vec<Resource>> {
    let mut paths = vec![];

    for ore in 1..*bp.maxes.get(&Ore).unwrap() {
        for clay in 1..*bp.maxes.get(&Clay).unwrap() {
            for obsidian in 1..*bp.maxes.get(&Obsidian).unwrap() {
                let mut path = vec![];
                (0..ore).for_each(|_|path.push(Ore));
                (0..clay).for_each(|_|path.push(Clay));
                (0..obsidian).for_each(|_|path.push(Obsidian));
                (0..24).for_each(|_|path.push(Geodes));
                paths.push(path);
            }
        }
    }

    paths
}

fn solve_2(_blueprints: Vec<BluePrint>) -> usize {
    0
}


// fn paths(bp: &BluePrint, minutes: usize) -> (usize, Vec<Resource>) {
//     let on_hand: HashMap<Resource, usize> = [(Ore, 0), (Clay, 0), (Obsidian, 0), (Geodes, 0)].into_iter().collect();
//     let robots: HashMap<Resource, usize> = [(Ore, 1), (Clay, 0), (Obsidian, 0), (Geodes, 0)].into_iter().collect();
//     let stages = vec![vec![Ore, Clay], vec![Obsidian, Clay], vec![Geodes]];
//
//     gen_paths(bp, on_hand, robots, &stages, 0, minutes, 1, vec![])
// }

// fn gen_paths(
//     bp: &BluePrint,
//     on_hand: HashMap<Resource, usize>,
//     robots: HashMap<Resource, usize>,
//     stages: &Vec<Vec<Resource>>,
//     stage: usize,
//     minutes: usize,
//     minute: usize,
//     mut path: Vec<Resource>
// ) -> (usize, Vec<Resource>) {
//     if minute >= minutes {
//         let geodes = *on_hand.get(&Geodes).unwrap();
//         return (geodes, path);
//     }
//
//     let stage_done = stages[stage].iter()
//         .all(|r| robots.get(r).unwrap() >= bp.maxes.get(r).unwrap());
//     if stage < 2 && stage_done {
//         return gen_paths(
//             bp,
//             on_hand,
//             robots,
//             stages,
//             stage + 1,
//             minutes,
//             minute,
//             path,
//         );
//     }
//
//     let mut max_geodes = 0;
//     let mut best_path = vec![];
//     for resource in &stages[stage] {
//         if resource != &Geodes && robots.get(resource).unwrap() >= bp.maxes.get(resource).unwrap() {
//             continue;
//         }
//
//         let mut min = minute;
//         let mut ro = robots.clone();
//         let mut oh = on_hand.clone();
//
//         while min <= minutes && !can_buy(bp, &oh, resource) {
//             for res in &RESOURCES {
//                 let extracted = ro.get(res).unwrap();
//                 let has = oh.get_mut(res).unwrap();
//                 *has += extracted;
//             }
//             min += 1;
//         }
//
//         if min <= minutes {
//             *ro.get_mut(resource).unwrap() += 1;
//             for (r, cost) in bp.costs.get(resource).unwrap() {
//                 *oh.get_mut(r).unwrap() -= cost;
//             }
//
//             let can_advance = stage < stages.len() - 1 &&
//                 stages[stage].iter().all(|r| ro.get(r).unwrap() > &0);
//
//             path.push(resource.clone());
//             if can_advance {
//                 let (geodes, path) = gen_paths(
//                     bp,
//                     oh.clone(),
//                     ro.clone(),
//                     stages,
//                     stage + 1,
//                     minutes,
//                     min,
//                     path.clone()
//                 );
//
//                 if geodes > max_geodes {
//                     max_geodes = geodes;
//                     best_path = path;
//                 }
//             }
//
//             let (geodes, path) = gen_paths(
//                 bp,
//                 oh.clone(),
//                 ro.clone(),
//                 stages,
//                 stage,
//                 minutes,
//                 min,
//                 path.clone()
//             );
//
//             if geodes > max_geodes {
//                 max_geodes = geodes;
//                 best_path = path;
//             }
//         }
//     }
//
//     (max_geodes, best_path)
// }

fn test_strategy(bp: &BluePrint, build_order: &Vec<Resource>, minutes: usize) -> usize {
    let mut on_hand: HashMap<Resource, usize> = [(Ore, 0), (Clay, 0), (Obsidian, 0), (Geodes, 0)].into_iter().collect();
    let mut robots: HashMap<Resource, usize> = [(Ore, 1), (Clay, 0), (Obsidian, 0), (Geodes, 0)].into_iter().collect();

    let mut minute = 1;
    for build_next in build_order {
        while minute <= minutes {
            if can_buy(bp, &on_hand, build_next) {
                for resource in &RESOURCES {
                    let extracted = robots.get(resource).unwrap();
                    let has = on_hand.get_mut(resource).unwrap();
                    *has += extracted;
                }

                // println!("{}: Buying {:?}", minute, build_next);
                *robots.get_mut(build_next).unwrap() += 1;
                for (resource, cost) in bp.costs.get(build_next).unwrap() {
                    *on_hand.get_mut(resource).unwrap() -= cost;
                }
                // println!("{}: ({:?}) R: {:?} O: {:?}", minute, build_next, robots, on_hand);
                minute += 1;
                break;
            } else {
                for resource in &RESOURCES {
                    let extracted = robots.get(resource).unwrap();
                    let has = on_hand.get_mut(resource).unwrap();
                    *has += extracted;
                }
                // println!("{}: ({:?}) R: {:?} O: {:?}", minute, build_next, robots, on_hand);
                minute += 1;
            }
        }

        if minute > minutes {
            break;
        }
    }

    *on_hand.get(&Geodes).unwrap()
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
        .map(|l| regex.find_iter(l.as_str()).map(|m| usize::from_str(m.as_str()).unwrap()))
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
