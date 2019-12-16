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
        run(&mut real);
    }
    println!("{:?}", &real[(orig_offset - offset)..(orig_offset - offset + 8)]);
}

fn run(input: &mut Vec<i32>) {
    let mut last = input[0];
    for i in 1..input.len() {
        input[0] += input[i];
    }

    for i in 1..input.len() {
        let temp = input[i];
        input[i] = input[i-1] - last;
        last = temp;
        input[i-1] = (input[i-1].abs()) % 10;
    }
    let l = input.len() - 1;
    input[l] = (input[l].abs()) % 10;
}
