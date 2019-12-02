#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    let mut progx: Vec<usize> = Vec::new();
    for line in input.lines() {
        for part in line.split(',') {
            let value = part.parse::<usize>().unwrap();
            progx.push(value);
        }
    }

    let goal = 19690720;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut prog = progx.clone();
    prog[1] = noun;
    prog[2] = verb;
    let mut pc = 0;
    loop {
        let more = run_once(&mut prog, pc);
        if !more {
            break;
        }
        pc += 4;
    }
    if prog[0] == goal {
        println!("{}, {}, ans = {}", noun, verb, 100*noun + verb);
    }
        }
    }
}

fn run_once(prog: &mut Vec<usize>, pc: usize) -> bool {
    match prog[pc] {
        1 => {
            let (a, b) = (prog[pc + 1], prog[pc + 2]);
            let idx = prog[pc+3];
            prog[idx] = prog[a] + prog[b];
            true
        },
        2 => {
            let (a, b) = (prog[pc + 1], prog[pc + 2]);
            let idx = prog[pc+3];
            prog[idx] = prog[a] * prog[b];
            true
        },
        99 => false,
        _ => panic!("bad opcode"),
    }
}
