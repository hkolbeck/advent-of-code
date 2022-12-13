extern crate core;

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs;
use std::iter::Peekable;
use std::str::{Chars, FromStr};
use std::time::Instant;
use crate::PacketItem::{Int, List};

fn main() {
    let raw_input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<String> = raw_input.split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    let packet_pairs = parse_input(lines);
    // println!();
    // packet_pairs.iter()
    //     .for_each(|(left, right)| println!("{:?}\n{:?}\n", left, right));

    let start = Instant::now();
    let score = solve_1(&packet_pairs);
    let end = Instant::now();
    println!("Part 1: {} in {}us", score, end.duration_since(start).as_micros());

    let start = Instant::now();
    let score = solve_2(&packet_pairs);
    let end = Instant::now();
    println!("Part 2: {} in {}us", score, end.duration_since(start).as_micros());
}

fn solve_1(pairs: &Vec<(PacketItem, PacketItem)>) -> usize {
    let mut correct = 0;
    for idx in 0..pairs.len() {
        let (left, right) = &pairs[idx];
        if cmp_packet_items(left, right) == Ordering::Less {
            correct += idx + 1;
        }
    }

    correct
}

fn solve_2(pairs: &Vec<(PacketItem, PacketItem)>) -> usize {
    let mut all_packets: Vec<PacketItem> = pairs.iter()
        .map(|(left, right)| [left, right])
        .flatten()
        .map(|p| p.clone())
        .collect();
    let marker_1 = List(vec![List(vec![Int(6)])]);
    all_packets.push(marker_1.clone());
    let marker_2 = List(vec![List(vec![Int(2)])]);
    all_packets.push(marker_2.clone());

    all_packets.sort_by(|a, b| cmp_packet_items(a, b));

    let mut key = 1;
    for idx in 0..all_packets.len() {
        if cmp_packet_items(&all_packets[idx], &marker_1) == Ordering::Equal ||
            cmp_packet_items(&all_packets[idx], &marker_2) == Ordering::Equal {
            key *= 1 + idx;
        }
    }

    key
}


#[derive(Clone, Debug)]
enum PacketItem {
    Int(i32),
    List(Vec<PacketItem>),
}

fn cmp_packet_items(a: &PacketItem, b: &PacketItem) -> Ordering {
    match a {
        Int(a_val) => match b {
            Int(b_val) => a_val.cmp(b_val),
            List(_) => cmp_packet_items(&List(vec![Int(*a_val)]), b)
        }
        List(a_vec) => match b {
            Int(b_val) => cmp_packet_items(a, &List(vec![Int(*b_val)])),
            List(b_vec) => {
                for idx in 0..a_vec.len().min(b_vec.len()) {
                    match cmp_packet_items(&a_vec[idx], &b_vec[idx]) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        _ => {}
                    }
                }

                a_vec.len().cmp(&b_vec.len())
            }
        }
    }
}

impl Display for PacketItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Int(val) => f.write_str(val.to_string().as_str())?,
            List(items) => {
                f.write_str("[")?;
                let str_items: Vec<String> = items.iter().map(|i| i.to_string()).collect();
                f.write_str(str_items.join(",").as_str())?;
                f.write_str("]")?;
            }
        }

        Ok(())
    }
}

impl FromStr for PacketItem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_packet(&mut s.chars().peekable()) {
            Int(_) => panic!("wat"),
            List(p) => Ok(p[0].clone()),
        }
    }
}

fn parse_packet(chars: &mut Peekable<Chars>) -> PacketItem {
    let mut list: Vec<PacketItem> = Vec::new();
    while chars.peek().is_some() {
        let mut c = *chars.peek().unwrap();
        if c == ']' {
            chars.next().unwrap();
            return List(list);
        } else if c == '[' {
            chars.next().unwrap();
            list.push(parse_packet(chars));
        } else if c.is_ascii_digit() {
            let mut digits: Vec<char> = Vec::new();
            while c.is_ascii_digit() {
                digits.push(c);
                chars.next().unwrap();
                c = *chars.peek().unwrap();
            }

            let val = i32::from_str(String::from_iter(digits.iter()).as_str()).unwrap();
            list.push(Int(val))
        } else if c == ',' {
            chars.next().unwrap();
        } else {
            panic!("Huh?")
        }
    }

    List(list)
}

fn parse_input(lines: Vec<String>) -> Vec<(PacketItem, PacketItem)> {
    lines.chunks(2)
        .map(|pair|
            (
                PacketItem::from_str(pair[0].as_str()).unwrap(),
                PacketItem::from_str(pair[1].as_str()).unwrap()
            )
        )
        .collect()
}