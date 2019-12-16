#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    let line = input.lines().next().unwrap();
    let mut data = Vec::new();
    for c in line.chars() {
        data.push(c.to_digit(10).unwrap() as i32);
    }

    for _ in 0..100 {
        data = run(data);
    }
    println!("{:?}", &data[0..8]);
}

fn run(input: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; input.len()];

    for i in 0..result.len() {
        for (j, n) in input.iter().enumerate() {
            result[i] += *n * factor(i as i32, j as i32);
        }
        result[i] = (result[i].abs()) % 10;
    }
    result
}

fn factor(i: i32, j: i32) -> i32 {
     let cycle = (i + 1) * 4;

     let pos = (j + 1) % cycle;

     if pos < (i + 1) {
         return 0;
     }
     if pos < 2*(i+1) {
         return 1;
     }
     if pos < 3*(i+1) {
         return 0;
     }
     return -1;
}
