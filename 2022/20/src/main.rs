use std::fs;
use std::str::FromStr;
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

fn solve_1(ring: Vec<i64>) -> i64 {
    let mut idx_ring = vec![];
    for idx in 0..ring.len() {
        idx_ring.push((idx, ring[idx]));
    }

    let mut buf = RingBuffer {
        buf: idx_ring
    };

    for orig_idx in 0..ring.len() {
        // println!("{}: {:?}", orig_idx, buf);
        let cur_idx = buf.find_idx(orig_idx);
        buf.shift(1, cur_idx);
    }

    // 1, 2, -3, 4, 0, 3, -2
    // println!("{:?}", buf);

    let zero_idx = buf.find_0();
    println!("{}, {}, {}",     buf.nth(zero_idx, 1000), buf.nth(zero_idx, 2000), buf.nth(zero_idx, 3000));
    buf.nth(zero_idx, 1000) + buf.nth(zero_idx, 2000) + buf.nth(zero_idx, 3000)
}

fn solve_2(_ring: Vec<i64>) -> usize {
    0
}

#[derive(Debug)]
struct RingBuffer {
    buf: Vec<(usize, i64)>,
}

impl RingBuffer {
    pub fn find_idx(&self, orig_idx: usize) -> usize {
        for idx in 0..self.buf.len() {
            let (oi, _) = self.buf[idx];
            if oi == orig_idx {
                return idx;
            }
        }

        panic!("Not found!")
    }

    pub fn shift(&mut self, key: i64, idx: usize) {
        let (orig_idx, num) = self.buf[idx];
        let shift = (key * num) % self.buf.len() as i64;
        // println!("Shift: {}", shift);
        self.buf.remove(idx);
        if shift < 0 {
            let shift = (self.buf.len() as i64 + shift) as usize;
            self.buf.rotate_left(shift);
        } else if shift > 0 {
            self.buf.rotate_left(shift as usize);
        }
        self.buf.insert(idx, (orig_idx, num));
    }

    fn find_0(&self) -> usize {
        let mut zero_loc = usize::MAX;
        for idx in 0..self.buf.len() {
            if self.buf[idx].1 == 0 {
                zero_loc = idx;
                break;
            }
        };

        if zero_loc == usize::MAX {
            panic!("No zero!");
        }

        zero_loc
    }

    fn nth(&self, base: usize, n: usize) -> i64 {
        self.buf[(base + n) % self.buf.len()].1
    }
}

fn parse_input(lines: Vec<String>) -> Vec<i64> {
    lines.iter().map(|l| i64::from_str(l.as_str()).unwrap()).collect()
}
