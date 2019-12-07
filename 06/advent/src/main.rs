#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");
    let mut graph = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(')').collect();
        let sat = parts[1];
        let orb = parts[0];
        {
            let entry = graph.entry(sat).or_insert(Vec::new());
            entry.push(orb);
        }
        {
            let entry = graph.entry(orb).or_insert(Vec::new());
            entry.push(sat);
        }
    }
    let start = graph.get("YOU").unwrap().first().unwrap();
    let end = graph.get("SAN").unwrap().first().unwrap();
    println!("{}", shortest_path(&graph, start, end));
}

fn shortest_path(graph: &HashMap<&str, Vec<&str>>, start: &str, goal: &str) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut seen = HashSet::new();
    let mut path = HashMap::new();
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if current == goal {
            break;
        }
        match graph.get(current) {
            Some(orbs) => {
                for orb in orbs {
                    if seen.insert(orb) {
                        queue.push_back(orb);
                        path.insert(orb, current);
                    }
                }
            }
            None => {}
        }
    }
    let mut count = 0;
    let mut current = goal;
    while current != start {
        count += 1;
        current = path.get(&current).unwrap();
    }
    count
}
