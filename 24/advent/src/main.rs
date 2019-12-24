#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

const N: usize = 5;

fn main() {
    let input = include_str!("../input.txt");
    let mut data = [0u64; N*N];
    let mut idx = 0;
    for line in input.lines() {
        for c in line.chars() {
            if c == '#' {
                data[idx] = 1;
            }
            idx += 1;
        }
    }

    let mut seen = HashSet::new();
    loop {
        let h = compute(&data);
        if !seen.insert(h) {
            break;
        }
        data = update(data);
    }
    println!("{}", compute(&data));
}

fn compute(data: &[u64;N*N]) -> u64 {
    let mut result = 0;
    let mut mult = 1;
    for i in 0..N*N {
        if data[i] == 1 {
            result += mult;
        }
        mult *= 2;
    }
    result
}

fn update(data: [u64;N*N]) -> [u64;N*N] {
    let mut result = [0;N*N];
    for r in 0..N {
        for c in 0..N {
            let adj = if r > 0 { data[(r - 1)*N + c] } else { 0 }
            + if r < N - 1 { data[(r + 1)*N + c] } else { 0 }
            + if c > 0 { data[r*N + c - 1] } else { 0 }
            + if c < N - 1 { data[r*N + c + 1] } else { 0 };
            if data[r*N + c] == 1 {
                result[r*N + c] = if adj == 1 { 1 } else { 0 };
            } else {
                result[r*N + c] = if adj == 1 || adj == 2 { 1 } else { 0 };
            }
        }
    }
    result
}
