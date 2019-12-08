#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

const H: usize = 6;
const W: usize = 25;

fn main() {
    let input = include_str!("../input.txt");
    let line = input.lines().next().unwrap();
    let len = line.len();
    let mut data = Vec::with_capacity(len);
    let layers = len / (W * H);
    for n in line.chars() {
        let v = n.to_digit(10).unwrap() as usize;
        data.push(v);
    }

    let mut best = std::usize::MAX;
    let mut result = 0;
    for layer in 0..layers {
        let (z, o, t) = count(&data, layer);
        if z < best {
            best = z;
            result = o * t;
        }
    }
    println!("{}", result);
}

fn count(data: &Vec<usize>, layer: usize) -> (usize, usize, usize) {
    let mut z = 0;
    let mut o = 0;
    let mut t = 0;
    for i in 0..W*H {
        match data[(W*H) * layer + i] {
            0 => z += 1,
            1 => o += 1,
            2 => t += 1,
            _ => {},
        }
    }
    (z, o, t)
}
