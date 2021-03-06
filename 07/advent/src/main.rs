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

    let mut best = 0;
    for a in 0..5 {
        for b in 0..5 {
            if b == a {
                continue;
            }
            for c in 0..5 {
                if c == a || c == b {
                    continue;
                }
                for d in 0..5 {
                    if d == c || d == b || d == a {
                        continue;
                    }
                    for e in 0..5 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let result =
                            run_with_phases(prog.clone(), [a + 5, b + 5, c + 5, d + 5, e + 5]);
                        if result > best {
                            best = result;
                        }
                    }
                }
            }
        }
    }

    println!("{}", best);
}

fn run_with_phases(prog: Vec<isize>, phases: [usize; 5]) -> isize {
    let mut amps = [
        (prog.clone(), 0, true),
        (prog.clone(), 0, true),
        (prog.clone(), 0, true),
        (prog.clone(), 0, true),
        (prog.clone(), 0, true),
    ];
    let mut idx = 0;
    let mut input = 0;
    loop {
        let in_ = if amps[idx].2 {
            phases[idx] as isize
        } else {
            input
        };
        let ip = amps[idx].1;
        let (new_ip, halt, did_input, output) = step(&mut amps[idx].0, ip, in_);
        if did_input && amps[idx].2 {
            amps[idx].2 = false;
        }
        amps[idx].1 = new_ip;
        if let Some(out) = output {
            input = out;
            idx = (idx + 1) % 5;
        }
        if halt {
            if idx == 4 {
                break;
            }
            idx = (idx + 1) % 5;
        }
    }
    input
}

fn step(prog: &mut Vec<isize>, ip: usize, in_: isize) -> (usize, bool, bool, Option<isize>) {
    let (inst, a, b, _c, _d) = parse(prog[ip]);
    match inst {
        1 => {
            let x = if a == 0 {
                prog[prog[ip + 1] as usize]
            } else {
                prog[ip + 1]
            };
            let y = if b == 0 {
                prog[prog[ip + 2] as usize]
            } else {
                prog[ip + 2]
            };
            let z = prog[ip + 3] as usize;
            prog[z] = x + y;
            (ip + 4, false, false, None)
        }
        2 => {
            let x = if a == 0 {
                prog[prog[ip + 1] as usize]
            } else {
                prog[ip + 1]
            };
            let y = if b == 0 {
                prog[prog[ip + 2] as usize]
            } else {
                prog[ip + 2]
            };
            let z = prog[ip + 3] as usize;
            prog[z] = x * y;
            (ip + 4, false, false, None)
        }
        3 => {
            let z = prog[ip + 1] as usize;
            prog[z] = in_;
            (ip + 2, false, true, None)
        }
        4 => {
            let x = if a == 0 {
                prog[prog[ip + 1] as usize]
            } else {
                prog[ip + 1]
            };
            (ip + 2, false, false, Some(x))
        }
        5 => {
            let x = if a == 0 {
                prog[prog[ip + 1] as usize]
            } else {
                prog[ip + 1]
            };
            let y = if b == 0 {
                prog[prog[ip + 2] as usize]
            } else {
                prog[ip + 2]
            };
            if x != 0 {
                (y as usize, false, false, None)
            } else {
                (ip + 3, false, false, None)
            }
        }
        6 => {
            let x = if a == 0 {
                prog[prog[ip + 1] as usize]
            } else {
                prog[ip + 1]
            };
            let y = if b == 0 {
                prog[prog[ip + 2] as usize]
            } else {
                prog[ip + 2]
            };
            if x == 0 {
                (y as usize, false, false, None)
            } else {
                (ip + 3, false, false, None)
            }
        }
        7 => {
            let x = if a == 0 {
                prog[prog[ip + 1] as usize]
            } else {
                prog[ip + 1]
            };
            let y = if b == 0 {
                prog[prog[ip + 2] as usize]
            } else {
                prog[ip + 2]
            };
            let z = prog[ip + 3] as usize;
            if x < y {
                prog[z] = 1;
            } else {
                prog[z] = 0;
            }
            (ip + 4, false, false, None)
        }
        8 => {
            let x = if a == 0 {
                prog[prog[ip + 1] as usize]
            } else {
                prog[ip + 1]
            };
            let y = if b == 0 {
                prog[prog[ip + 2] as usize]
            } else {
                prog[ip + 2]
            };
            let z = prog[ip + 3] as usize;
            if x == y {
                prog[z] = 1;
            } else {
                prog[z] = 0;
            }
            (ip + 4, false, false, None)
        }
        99 => (ip, true, false, None),
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
