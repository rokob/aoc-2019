#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Step {
    Deal,
    Incr(usize),
    Cut(isize),
}

const N: usize = 119315717514047;
const RUNS: usize = 101741582076661;

fn main() {
    let input = include_str!("../input.txt");
    let mut steps = Vec::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        match parts[0] {
            "deal" => match parts[1] {
                "with" => {
                    steps.push(Step::Incr(parts[3].parse::<usize>().unwrap()));
                }
                "into" => {
                    steps.push(Step::Deal);
                }
                _ => panic!("bad input: {:?}", line),
            },
            "cut" => {
                steps.push(Step::Cut(parts[1].parse::<isize>().unwrap()));
            }
            _ => panic!("bad input: {:?}", line),
        }
    }

    let goal = 2020;
    let mut a = 1; let mut b = 0;
    for step in steps.iter().rev() {
        translate(step, N as isize, &mut a, &mut b);
    }
    a = mod_euc(a, N as isize);
    b = mod_euc(b, N as isize);
    /*
    let mut current = goal as isize;
    for step in steps.iter().rev() {
        current = find_prior_index(step, current, N as isize);
    }
    let once = current;
    for step in steps.iter().rev() {
        current = find_prior_index(step, current, N as isize);
    }
    let twice = current;
    */
    let an = mod_pow(a, RUNS as isize, N as isize) as i128;

    let denom = mod_inv(a-1, N as isize) as i128;
    println!("{}*{} + {}*{}*{} mod {}", an, goal, b, an - 1, denom, N);
    println!("from wolfram => {}", 78613970589919i128);
    let first = an * goal as i128;
    let second = (b as i128) * (an - 1) * denom;
    let result = (first + second) % N as i128;
    println!("{}", result);
}

fn mod_pow(mut base: isize, mut exp: isize, modulus: isize) -> isize {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = ((result as i128 * base as i128) % modulus as i128) as isize;
        }
        exp = exp >> 1;
        base = ((base as i128 * base as i128) % modulus as i128) as isize
    }
    result
}

fn mod_euc(a: isize, rhs: isize) -> isize {
    let r = a % rhs;
    if r < 0 {
        if rhs < 0 {
            r - rhs
        } else {
            r + rhs
        }
    } else {
        r
    }
}

fn solve_lce(a: isize, m: isize, b: isize) -> isize {
    if m < 50 {
        for i in 0..m {
            if (a * (i as isize)) % m == b {
                return i as isize;
            }
        }
    }
    let new_a = mod_euc(m, a);
    let new_b = mod_euc(-b, a);
    let new_m = a;
    (m * solve_lce(new_a, new_m, new_b) + b) / a
}

fn mod_inv(mut a: isize, mut m: isize) -> isize {
    let m0 = m;
    let (mut x, mut y) = (1, 0);
    if m == 0 {
        return 0;
    }
    while a > 1 {
        let q = a / m;
        let mut t = m;
        m = mod_euc(a, m);
        a = t;
        t = y;
        y = x - q * y;
        x = t;
    }
    if x < 0 {
        x = x + m0;
    }
    return x;
}

fn translate(step: &Step, size: isize, a: &mut isize, b: &mut isize) {
    match step {
        Step::Deal => {
            *a = -*a;
            *b = (size - 1 - *b) % size;
        },
        Step::Incr(i) => {
            let inv = mod_inv(*i as isize, size);
            *a = ((*a as i128 * inv as i128) % size as i128) as isize;
            *b = ((*b as i128 * inv as i128) % size as i128) as isize;
        },
        Step::Cut(i) => {
            *b = (*b + i) % size;
        },
    }
}

fn find_prior_index(step: &Step, current: isize, size: isize) -> isize {
    match step {
        Step::Deal => size - 1 - current,
        Step::Incr(i) => {
            let a = *i;
            let m = size;
            let b = current;
            solve_lce(a as isize, m, b)
        }
        Step::Cut(i) => {
            if *i > 0 {
                if current < size - *i {
                    current + *i
                } else {
                    current - size + *i
                }
            } else {
                if current < -*i {
                    size + *i - current
                } else {
                    current + *i
                }
            }
        }
    }
}

fn take_step(step: &Step, current: isize, size: isize) -> isize {
    match step {
        Step::Deal => size - 1 - current,
        Step::Incr(i) => (current * *i as isize) % size,
        Step::Cut(i) => {
            if *i > 0 {
                if current < *i {
                    size - *i + current
                } else {
                    current - *i
                }
            } else {
                if current > size - 1 + *i {
                    current - size - *i
                } else {
                    current - *i
                }
            }
        }
    }
}
