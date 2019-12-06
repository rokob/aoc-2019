#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    let mut graph = HashMap::new();
    let mut sats = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(')').collect();
        let sat = parts[1];
        let orb = parts[0];
        let entry = graph.entry(sat).or_insert(Vec::new());
        sats.push(sat);
        entry.push(orb);
    }
    let mut result = 0;
    for sat in sats.iter() {
        result += count_orbs(&graph, sat);
    }
    println!("{}", result);
}

fn count_orbs(graph: &HashMap<&str, Vec<&str>>, start: &str) -> usize {
    let mut result = 0;
    let mut queue = vec![start];
    let mut seen = HashSet::new();
    loop {
        if queue.is_empty() {
            break;
        }
        let current = queue.pop().unwrap();
        match graph.get(current) {
            Some(orbs) => {
                for orb in orbs {
                    if seen.insert(orb) {
                        result += 1;
                        queue.push(orb);
                    }
                }
            }
            None => {},
        }
    }
    result
}
