#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

const N: usize = 20000;

fn main() {
    let input = include_str!("../input.txt");
    let mut thing = Vec::with_capacity(N);
    for r in 0..N {
        thing.push(Vec::with_capacity(N));
        for _ in 0..N {
            thing[r].push((0, 0));
        }
    }
    let mut wires = input.lines();
    let wire1 = wires.next().unwrap();
    let wire2 = wires.next().unwrap();
    let mut r = N / 2;
    let mut c = N / 2;
    let mut intersections = Vec::new();
    let mut steps = 0;
        for part in wire1.split(',') {
            let dir = part.chars().next().unwrap();
            let amt = *(&part[1..].parse::<usize>().unwrap());
            match dir {
                'R' => {
                    for _ in 0..amt {
                        steps += 1;
                        c += 1;
                        if thing[r][c].0 == 0 {
                            thing[r][c] = (steps, 0);
                        }
                    }
                },
                'U' => {
                    for _ in 0..amt {
                        steps += 1;
                        r -= 1;
                        if thing[r][c].0 == 0 {
                            thing[r][c] = (steps, 0);
                        }
                    }
                },
                'L' => {
                    for _ in 0..amt {
                        steps += 1;
                        c -= 1;
                        if thing[r][c].0 == 0 {
                            thing[r][c] = (steps, 0);
                        }
                    }
                },
                'D' => {
                    for _ in 0..amt {
                        steps += 1;
                        r += 1;
                        if thing[r][c].0 == 0 {
                            thing[r][c] = (steps, 0);
                        }
                    }
                },
                _ => {},
            }
        }
    r = N / 2;
    c = N / 2;
    steps = 0;
        for part in wire2.split(',') {
            let dir = part.chars().next().unwrap();
            let amt = part[1..].parse::<usize>().unwrap();
            match dir {
                'R' => {
                    for _ in 0..amt {
                        steps += 1;
                        c += 1;
                        if thing[r][c].1 == 0 {
                            thing[r][c] = (thing[r][c].0, steps);
                        }
                        if thing[r][c].0 != 0 {
                            intersections.push((r, c));
                        }
                    }
                },
                'U' => {
                    for _ in 0..amt {
                        steps += 1;
                        r -= 1;
                        if thing[r][c].1 == 0 {
                            thing[r][c] = (thing[r][c].0, steps);
                        }
                        if thing[r][c].0 != 0 {
                            intersections.push((r, c));
                        }
                    }
                },
                'L' => {
                    for _ in 0..amt {
                        steps += 1;
                        c -= 1;
                        if thing[r][c].1 == 0 {
                            thing[r][c] = (thing[r][c].0, steps);
                        }
                        if thing[r][c].0 != 0 {
                            intersections.push((r, c));
                        }
                    }
                },
                'D' => {
                    for _ in 0..amt {
                        steps += 1;
                        r += 1;
                        if thing[r][c].1 == 0 {
                            thing[r][c] = (thing[r][c].0, steps);
                        }
                        if thing[r][c].0 != 0 {
                            intersections.push((r, c));
                        }
                    }
                },
                _ => {},
            }
        }

        let mut best = std::i64::MAX;
        for (r, c) in intersections {
            let amt = thing[r][c].0 + thing[r][c].1;
                if amt < best {
                best = amt;
            }
        }
        println!("{}", best);
}
