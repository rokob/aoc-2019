#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    let line = input.lines().next().unwrap();
    let mut data = Vec::new();
    for c in line.chars() {
        data.push(c.to_digit(10).unwrap() as i32);
    }

    let mut offset = 0;
    for i in 0..7 {
        offset = offset*10 + data[i];
    }
    let orig_offset = offset as usize;
    let x = orig_offset / data.len();
    let offset = x * data.len();

    let mut real = Vec::with_capacity((10000-x+1)*data.len());
    for _ in x..10000 {
        real.append(&mut data.clone());
    }

    for _ in 0..100 {
        real = run(real);
    }
    println!("{:?}", &real[(orig_offset - offset)..(orig_offset - offset + 8)]);
}

fn run(input: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; input.len()];

    for n in input.iter() {
        result[0] += *n;
    }

    for i in 1..result.len() {
        result[i] = result[i-1] - input[i-1];
        result[i-1] = (result[i-1].abs()) % 10;
    }
    let l = result.len() - 1;
    result[l] = (result[l].abs()) % 10;
    result
}
