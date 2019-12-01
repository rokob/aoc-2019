#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    let mut result = 0;
    for line in input.lines() {
        let value = line.parse::<i64>().unwrap();
        result += compute(value);
    }
    println!("{}", result);
}

fn compute(value: i64) -> i64 {
    value
}
