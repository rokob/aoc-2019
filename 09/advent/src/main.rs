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

fn maybe_extend(prog: &mut Vec<isize>, idx: usize) {
    if idx < prog.len() {
        return;
    }
    prog.resize(prog.len() * 2, 0);
}

fn step(prog: &mut Vec<isize>, ip: usize, in_: isize, rb: isize) -> (usize, bool, bool, Option<isize>, isize) {
    let (inst, a, b, c, _d) = parse(prog[ip]);
    match inst {
        1 => {
            let x = if a == 0 {
                let i = prog[ip + 1] as usize;
            maybe_extend(prog, i as usize);
            prog[i]

            } else if a == 2 {
                let i = (rb + prog[ip + 1]) as usize;
            maybe_extend(prog, i as usize);
            prog[i]

            } else {
                prog[ip + 1]
            };
            let y = if b == 0 {
                let i = prog[ip + 2] as usize;
            maybe_extend(prog, i as usize);
            prog[i]
            } else if b == 2 {
                let i = (rb + prog[ip + 2]) as usize;
            maybe_extend(prog, i as usize);
            prog[i]
            } else {
                prog[ip + 2]
            };
            let z = if c == 0 {
                prog[ip + 3] as usize
            } else {
                (rb + prog[ip + 3]) as usize
            };
            maybe_extend(prog, z);
            prog[z] = x + y;
            (ip + 4, false, false, None, rb)
        }
        2 => {
            let x = if a == 0 {
                let i = prog[ip + 1] as usize;
            maybe_extend(prog, i as usize);
            prog[i]
            } else if a == 2 {
                let i = (rb + prog[ip + 1]) as usize;
            maybe_extend(prog, i as usize);
            prog[i]
            } else {
                prog[ip + 1]
            };
            let y = if b == 0 {
                let i = prog[ip + 2] as usize;
            maybe_extend(prog, i as usize);
            prog[i]
            } else if b == 2 {
                let i = (rb + prog[ip + 2]) as usize;
            maybe_extend(prog, i as usize);
            prog[i]
            } else {
                prog[ip + 2]
            };
            let z = if c == 0 {
                prog[ip + 3] as usize
            } else {
                (rb + prog[ip + 3]) as usize
            };
            maybe_extend(prog, z as usize);
            prog[z] = x * y;
            (ip + 4, false, false, None, rb)
        }
        3 => {
            let z = if a == 0 {
                prog[ip + 1] as usize
            } else {
                (rb + prog[ip + 1]) as usize
            };
            maybe_extend(prog, z as usize);
            prog[z] = in_;
            (ip + 2, false, true, None, rb)
        }
        4 => {
            let x = if a == 0 {
                let i = prog[ip + 1] as usize;
            maybe_extend(prog, i as usize);
            prog[i]

            } else if a == 2 {
                let i = (rb + prog[ip + 1]) as usize;
            maybe_extend(prog, i as usize);
            prog[i]
            } else {
                prog[ip + 1]
            };
            (ip + 2, false, false, Some(x), rb)
        }
        5 => {
            let x = if a == 0 {
                let i = prog[ip + 1] as usize;
                maybe_extend(prog, i);
                prog[i]
            } else if a == 2 {
                let i = (rb + prog[ip + 1]) as usize;
                maybe_extend(prog, i);
                prog[i]

            } else {
                prog[ip + 1]
            };
            let y = if b == 0 {
                let i = prog[ip + 2] as usize;
                maybe_extend(prog, i);
                prog[i]
            } else if b == 2 {
                let i = (rb + prog[ip + 2]) as usize;
                maybe_extend(prog, i);
                prog[i]
            } else {
                prog[ip + 2]
            };
            if x != 0 {
                (y as usize, false, false, None, rb)
            } else {
                (ip + 3, false, false, None, rb)
            }
        }
        6 => {
            let x = if a == 0 {
                let i = prog[ip + 1] as usize;
                maybe_extend(prog, i);
                prog[i]
            } else if a == 2 {
                let i = (rb + prog[ip + 1]) as usize;
                maybe_extend(prog, i);
                prog[i]
            } else {
                prog[ip + 1]
            };
            let y = if b == 0 {
                let i = prog[ip + 2] as usize;
                maybe_extend(prog, i);
                prog[i]
            } else if b == 2 {
                let i = (rb + prog[ip + 2]) as usize;
                maybe_extend(prog, i);
                prog[i]
            } else {
                prog[ip + 2]
            };
            if x == 0 {
                (y as usize, false, false, None, rb)
            } else {
                (ip + 3, false, false, None, rb)
            }
        }
        7 => {
            let x = if a == 0 {
                let i = prog[ip + 1] as usize;
                maybe_extend(prog, i);
                prog[i]
            } else if a == 2 {
                let i = (rb + prog[ip + 1]) as usize;
                maybe_extend(prog, i);
                prog[i]
            } else {
                prog[ip + 1]
            };
            let y = if b == 0 {
                let i = prog[ip + 2] as usize;
                maybe_extend(prog, i);
                prog[i]
            } else if b == 2 {
                let i = (rb + prog[ip + 2]) as usize;
                maybe_extend(prog, i);
                prog[i]
            } else {
                prog[ip + 2]
            };
            let z = if c == 0 {
                prog[ip + 3] as usize
            } else {
                (rb + prog[ip + 3]) as usize
            };
            maybe_extend(prog, z);
            if x < y {
                prog[z] = 1;
            } else {
                prog[z] = 0;
            }
            (ip + 4, false, false, None, rb)
        }
        8 => {
            let x = if a == 0 {
                let i = prog[ip + 1] as usize;
                maybe_extend(prog, i);
                prog[i]
            } else if a == 2 {
                let i = (rb + prog[ip + 1]) as usize;
                maybe_extend(prog, i);
                prog[i]
            } else {
                prog[ip + 1]
            };
            let y = if b == 0 {
                let i = prog[ip + 2] as usize;
                maybe_extend(prog, i);
                prog[i]
            } else if b == 2 {
                let i = (rb + prog[ip + 2]) as usize;
                maybe_extend(prog, i);
                prog[i]
            } else {
                prog[ip + 2]
            };
            let z = if c == 0 {
                prog[ip + 3] as usize
            } else {
                (rb + prog[ip + 3]) as usize
            };
                maybe_extend(prog, z);
            if x == y {
                prog[z] = 1;
            } else {
                prog[z] = 0;
            }
            (ip + 4, false, false, None, rb)
        }
        9 => {
            let x = if a == 0 {
                let i = prog[ip + 1] as usize;
                maybe_extend(prog, i);
                prog[i]
            } else if a == 2 {
                let i = (rb + prog[ip + 1]) as usize;
                maybe_extend(prog, i);
                prog[i]
            } else {
                prog[ip + 1]
            };
            (ip + 2, false, false, None, rb + x)
        }
        99 => (ip, true, false, None, rb),
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
