#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    let mut prog = Vec::new();
    for line in input.lines() {
        for part in line.split(',') {
            prog.push(part.parse::<isize>().unwrap());
        }
    }

    let in_ = 2;

    let mut ip = 0;
    let mut rb = 0;
    loop {
        let (new_ip, did_halt, _did_input, output, new_rb) = step(&mut prog, ip, in_, rb);
        if did_halt {
            break;
        }
        if let Some(output) = output {
            println!("output: {}", output);
        }
        ip = new_ip;
        rb = new_rb;
    }
}

#[inline]
fn maybe_extend(prog: &mut Vec<isize>, idx: usize) {
    if idx < prog.len() {
        return;
    }
    prog.resize(prog.len() * 2, 0);
}

#[inline]
fn index(prog: &mut Vec<isize>, ip: usize, rb: isize, mode: isize, offset: usize) -> usize {
    if mode == 0 {
        let i = prog[ip + offset] as usize;
        maybe_extend(prog, i as usize);
        i
    } else if mode == 2 {
        let i = (rb + prog[ip + offset]) as usize;
        maybe_extend(prog, i as usize);
        i
    } else {
        ip + offset
    }
}

#[inline]
fn read(prog: &mut Vec<isize>, ip: usize, rb: isize, mode: isize, offset: usize) -> isize {
    let i = index(prog, ip, rb, mode, offset);
    prog[i]
}

// (new_ip, did_halt, _did_input, output, new_rb)
fn step(
    prog: &mut Vec<isize>,
    ip: usize,
    in_: isize,
    rb: isize,
) -> (usize, bool, bool, Option<isize>, isize) {
    let (inst, a, b, c) = parse(prog[ip]);
    if inst == 99 {
        return (ip, true, false, None, rb);
    }
    let xi = index(prog, ip, rb, a, 1);
    let x = prog[xi];
    match inst {
        1 => {
            let y = read(prog, ip, rb, b, 2);
            let z = index(prog, ip, rb, c, 3);
            prog[z] = x + y;
            (ip + 4, false, false, None, rb)
        }
        2 => {
            let y = read(prog, ip, rb, b, 2);
            let z = index(prog, ip, rb, c, 3);
            prog[z] = x * y;
            (ip + 4, false, false, None, rb)
        }
        3 => {
            prog[xi] = in_;
            (ip + 2, false, true, None, rb)
        }
        4 => (ip + 2, false, false, Some(x), rb),
        5 => {
            let y = read(prog, ip, rb, b, 2);
            if x != 0 {
                (y as usize, false, false, None, rb)
            } else {
                (ip + 3, false, false, None, rb)
            }
        }
        6 => {
            let y = read(prog, ip, rb, b, 2);
            if x == 0 {
                (y as usize, false, false, None, rb)
            } else {
                (ip + 3, false, false, None, rb)
            }
        }
        7 | 8 => {
            let y = read(prog, ip, rb, b, 2);
            let z = index(prog, ip, rb, c, 3);
            if (x < y && inst == 7) || (x == y && inst == 8) {
                prog[z] = 1;
            } else {
                prog[z] = 0;
            }
            (ip + 4, false, false, None, rb)
        }
        9 => (ip + 2, false, false, None, rb + x),
        _ => panic!("unk::{}", inst),
    }
}

fn parse(mut inst: isize) -> (isize, isize, isize, isize) {
    let x = inst % 100;
    inst /= 100;
    let a = inst % 10;
    inst /= 10;
    let b = inst % 10;
    inst /= 10;
    let c = inst % 10;
    (x, a, b, c)
}
