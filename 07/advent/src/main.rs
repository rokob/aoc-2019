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
    let result = run_with_phases(prog.clone(), [a, b, c, d, e]);
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


fn run_with_phases(mut prog: Vec<isize>, phases: [usize; 5]) -> isize {
    let mut input = 0;
    let mut needs_phase;
    for i in 0..5 {
        let mut ip = 0;
        needs_phase = true;
        let phase = phases[i];
        loop {
            let in_ = if needs_phase { phase as isize } else { input };
            let (new_ip, halt, did_input, output) = step(&mut prog, ip, in_);
            if needs_phase && did_input {
                needs_phase = false;
            }
            if let Some(out) = output {
                input = out;
            }
            if halt {
                break;
            }
            ip = new_ip;
        }
    }
    input
}

fn step(prog: &mut Vec<isize>, ip: usize, in_: isize) -> (usize, bool, bool, Option<isize>) {
    let (inst, a, b, _c, _d) = parse(prog[ip]);
    match inst {
        1 => {
            let x = if a == 0 { prog[prog[ip + 1] as usize] } else { prog[ip + 1] };
            let y = if b == 0 { prog[prog[ip + 2] as usize] } else { prog[ip + 2] };
            let z = prog[ip + 3] as usize;
            prog[z] = x + y;
            (ip + 4, false, false, None)
        },
        2 => {
            let x = if a == 0 { prog[prog[ip + 1] as usize] } else { prog[ip + 1] };
            let y = if b == 0 { prog[prog[ip + 2] as usize] } else { prog[ip + 2] };
            let z = prog[ip + 3] as usize;
            prog[z] = x * y;
            (ip + 4, false, false, None)
        },
        3 => {
            let z = prog[ip + 1] as usize;
            prog[z] = in_;
            (ip + 2, false, true, None)
        },
        4 => {
            let x = if a == 0 { prog[prog[ip + 1] as usize] } else { prog[ip + 1] };
            (ip + 2, false, false, Some(x))
        },
        5 => {
            let x = if a == 0 { prog[prog[ip + 1] as usize] } else { prog[ip + 1] };
            let y = if b == 0 { prog[prog[ip + 2] as usize] } else { prog[ip + 2] };
            if x != 0 {
                (y as usize, false, false, None)
            } else {
                (ip + 3, false, false, None)
            }
        }
        6 => {
            let x = if a == 0 { prog[prog[ip + 1] as usize] } else { prog[ip + 1] };
            let y = if b == 0 { prog[prog[ip + 2] as usize] } else { prog[ip + 2] };
            if x == 0 {
                (y as usize, false, false, None)
            } else {
                (ip + 3, false, false, None)
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
            (ip + 4, false, false, None)
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
            (ip + 4, false, false, None)
        },
        99 => {
            (ip, true, false, None)
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

