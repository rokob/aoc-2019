#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

struct Reaction {
    inputs: Vec<(usize, String)>,
    output: (usize, String),
}

fn main() {
    let input = include_str!("../input.txt");
    let mut reactions = HashMap::new();
    for line in input.lines() {
        let parts = line.split(" => ").collect::<Vec<&str>>();
        let result = parts[1].split(' ').collect::<Vec<_>>();
        let result_amt = result[0].parse::<usize>().unwrap();
        let result_name = result[1].to_owned();
        let output = (result_amt, result_name);
        let mut inputs = Vec::new();
        for in_ in parts[0].split(", ") {
            let in_parts = in_.split(' ').collect::<Vec<_>>();
            let amt = in_parts[0].parse::<usize>().unwrap();
            let name = in_parts[1].to_owned();
            inputs.push((amt, name));
        }
        reactions.insert(output.1.clone(), Reaction { inputs, output });
    }
    let result = solve_solve(&reactions);
    println!("{}", result);
}

fn solve_solve(reactions: &HashMap<String, Reaction>) -> usize {
    let max = 1_000_000_000_000;
    let amt = max / solve(&reactions, 1);
    let mut lo = amt;
    let mut hi = amt * 2;
    loop {
        if lo >= hi {
            return hi - 1;
        }
        let mid = (lo + hi) / 2;
        println!("running {}", mid);
        let result = solve(&reactions, mid);
        if result > max {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
}

fn solve(reactions: &HashMap<String, Reaction>, start: usize) -> usize {
    let mut need = Vec::new();
    need.push((start, "FUEL".to_owned()));
    let mut ore = 0;
    let mut excess = HashMap::new();
    loop {
        if need.is_empty() {
            break;
        }
        let current = need.pop().unwrap();
        let reaction = reactions.get(&current.1).unwrap();
        let scale = reaction.output.0;
        let entry = excess.entry(current.1.clone()).or_insert(0);
        if *entry > 0 {
            if *entry >= current.0 {
                *entry -= current.0;
                continue;
            }
        }
        let need_amt = current.0 - *entry;
        let amt = find_amount(need_amt, scale);
        if amt * scale > need_amt {
            *entry = amt * scale - need_amt;
        } else {
            *entry = 0;
        }
        for in_ in reaction.inputs.iter() {
            if in_.1 == "ORE".to_owned() {
                ore += amt*in_.0;
            } else {
                need.push((in_.0*amt, in_.1.clone()));
            }
        }
    }
    ore
}

fn find_amount(need: usize, can_get: usize) -> usize {
    let mut i = 1;
    loop {
        if i * can_get >= need {
            return i;
        }
        i += 1;
    }
}
