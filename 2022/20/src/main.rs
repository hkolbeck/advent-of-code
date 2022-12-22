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
        buf.shift(cur_idx);
    }

    // 1, 2, -3, 4, 0, 3, -2
    // println!("{:?}", buf);

    let zero_idx = buf.find_0();
    println!("{}, {}, {}",     buf.nth(zero_idx, 1000), buf.nth(zero_idx, 2000), buf.nth(zero_idx, 3000));
    buf.nth(zero_idx, 1000) + buf.nth(zero_idx, 2000) + buf.nth(zero_idx, 3000)
}

fn solve_2(ring: Vec<i64>) -> i64 {
    let key = 811589153;
    let mut idx_ring = vec![];
    for idx in 0..ring.len() {
        idx_ring.push((idx, ring[idx] * key));
    }

    let mut buf = RingBuffer {
        buf: idx_ring
    };

    // println!("Init: {}", buf.as_str());
    for round in 0..10 {
        for orig_idx in 0..ring.len() {
            // println!("{}: {:?}", orig_idx, buf);
            let cur_idx = buf.find_idx(orig_idx);
            buf.shift(cur_idx);
        }
        // println!("{}: {}", round, buf.as_str());
    }

    //11_085_496_240_827 too low
    // 1, 2, -3, 4, 0, 3, -2
    // println!("{:?}", buf);

    let zero_idx = buf.find_0();
    println!("{}, {}, {}", buf.nth(zero_idx, 1000), buf.nth(zero_idx, 2000), buf.nth(zero_idx, 3000));
    buf.nth(zero_idx, 1000) + buf.nth(zero_idx, 2000) + buf.nth(zero_idx, 3000)
}

#[derive(Debug)]
struct RingBuffer {
    buf: Vec<(usize, i64)>,
}

impl RingBuffer {
    pub fn as_str(&self) -> String {
        let v: Vec<String> = self.buf.iter().map(|(_, n)| n.to_string()).collect();
        format!("[{}]", v.join(", "))
    }

    pub fn find_idx(&self, orig_idx: usize) -> usize {
        for idx in 0..self.buf.len() {
            let (oi, _) = self.buf[idx];
            if oi == orig_idx {
                return idx;
            }
        }

        panic!("Not found!")
    }

    pub fn shift(&mut self, idx: usize) {
        let buf_size = self.buf.len() as i64 - 1 ;
        let (orig_idx, num) = self.buf[idx];
        let mut new_idx = idx as i64 + num;
        new_idx = ((new_idx % buf_size) + buf_size) % buf_size;
        self.buf.remove(idx);
        self.buf.insert(new_idx as usize, (orig_idx, num));
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
