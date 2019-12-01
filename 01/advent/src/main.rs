#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    let mut result = 0;
    for line in input.lines() {
        let mass = line.parse::<i64>().unwrap();
        result += compute(mass);
    }
    println!("{}", result);
}

fn compute(mut mass: i64) -> i64 {
    let mut result = 0;
    loop {
        mass = mass / 3 - 2;
        if mass <= 0 {
            break;
        }
        result += mass;
    }
    result
}
