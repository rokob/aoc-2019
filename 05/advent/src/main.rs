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

    let mut ip = 0;
    loop {
        let (new_ip, halt) = step(&mut prog, ip);
        if halt {
            break;
        }
        ip = new_ip;
    }
}

const IN: isize = 5;

fn step(prog: &mut Vec<isize>, ip: usize) -> (usize, bool) {
    let (inst, a, b, _c, _d) = parse(prog[ip]);
    match inst {
        1 => {
            let x = if a == 0 { prog[prog[ip + 1] as usize] } else { prog[ip + 1] };
            let y = if b == 0 { prog[prog[ip + 2] as usize] } else { prog[ip + 2] };
            let z = prog[ip + 3] as usize;
            prog[z] = x + y;
            (ip + 4, false)
        },
        2 => {
            let x = if a == 0 { prog[prog[ip + 1] as usize] } else { prog[ip + 1] };
            let y = if b == 0 { prog[prog[ip + 2] as usize] } else { prog[ip + 2] };
            let z = prog[ip + 3] as usize;
            prog[z] = x * y;
            (ip + 4, false)
        },
        3 => {
            let z = prog[ip + 1] as usize;
            prog[z] = IN;
            (ip + 2, false)
        },
        4 => {
            let x = if a == 0 { prog[prog[ip + 1] as usize] } else { prog[ip + 1] };
            println!("{}", x);
            (ip + 2, false)
        },
        5 => {
            let x = if a == 0 { prog[prog[ip + 1] as usize] } else { prog[ip + 1] };
            let y = if b == 0 { prog[prog[ip + 2] as usize] } else { prog[ip + 2] };
            if x != 0 {
                (y as usize, false)
            } else {
                (ip + 3, false)
            }
        }
        6 => {
            let x = if a == 0 { prog[prog[ip + 1] as usize] } else { prog[ip + 1] };
            let y = if b == 0 { prog[prog[ip + 2] as usize] } else { prog[ip + 2] };
            if x == 0 {
                (y as usize, false)
            } else {
                (ip + 3, false)
            }
        }
        7 => {
            let x = if a == 0 { prog[prog[ip + 1] as usize] } else { prog[ip + 1] };
            let y = if b == 0 { prog[prog[ip + 2] as usize] } else { prog[ip + 2] };
            let z = prog[ip + 3] as usize;
            if x < y {
                prog[z] = 1;
            } else {
                prog[z] = 0;
            }
            (ip + 4, false)
        },
        8 => {
            let x = if a == 0 { prog[prog[ip + 1] as usize] } else { prog[ip + 1] };
            let y = if b == 0 { prog[prog[ip + 2] as usize] } else { prog[ip + 2] };
            let z = prog[ip + 3] as usize;
            if x == y {
                prog[z] = 1;
            } else {
                prog[z] = 0;
            }
            (ip + 4, false)
        },
        99 => {
            (ip, true)
        },
        _ => panic!("unk::{}", inst),
    }
}

fn parse(mut inst: isize) -> (isize, isize, isize, isize, isize) {
    let x = inst % 100;
    inst /= 100;
    let a = inst % 10;
    inst /= 10;
    let b = inst % 10;
    inst /= 10;
    let c = inst % 10;
    inst /= 10;
    let d = inst % 10;
    (x, a, b, c, d)
}

