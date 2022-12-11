extern crate core;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::str::FromStr;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    if let Ok(max) = solve() {
        let end = Instant::now();
        println!("{} in {}us", max, end.duration_since(start).as_micros());
    }
}

fn solve() -> Result<u64, Error> {
    let lines = BufReader::new(File::open("input.txt")?).lines();
    let mut heap = BinaryHeap::<Reverse<u64>>::new();
    let mut this_elf: u64 = 0;
    for line in lines {
        let line = line?;
        if line.is_empty() {
            heap.push(Reverse(this_elf));
            if heap.len() > 3 {
                heap.pop();
            }

            this_elf = 0;
        } else {
            let cal = match u64::from_str(line.as_str()) {
                Ok(cal) => cal,
                Err(_) => 0,
            };
            this_elf += cal;
        }
    }

    if this_elf != 0 {
        heap.push(Reverse(this_elf));
        if heap.len() > 3 {
            heap.pop();
        }
    }

    let top_three = heap.pop().unwrap().0 + heap.pop().unwrap().0 + heap.pop().unwrap().0;
    Ok(top_three)
}