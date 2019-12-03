#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

const N: usize = 50000;

fn main() {
    let input = include_str!("../input.txt");
    let mut thing = Vec::with_capacity(N);
    for r in 0..N {
        thing.push(Vec::with_capacity(N));
        for c in 0..N {
            thing[r].push(0);
        }
    }
    let mut wires = input.lines();
    let wire1 = wires.next().unwrap();
    let wire2 = wires.next().unwrap();
    let mut r = N / 2;
    let mut c = N / 2;
    let mut intersections = Vec::new();
        for part in wire1.split(',') {
            let dir = part.chars().next().unwrap();
            let amt = *(&part[1..].parse::<usize>().unwrap());
            match dir {
                'R' => {
                    for _ in 0..amt {
                        thing[r][c] = 1;
                        c += 1;
                    }
                },
                'U' => {
                    for _ in 0..amt {
                        thing[r][c] = 1;
                        r -= 1;
                    }
                },
                'L' => {
                    for _ in 0..amt {
                        thing[r][c] = 1;
                        c -= 1;
                    }
                },
                'D' => {
                    for _ in 0..amt {
                        thing[r][c] = 1;
                        r += 1;
                    }
                },
                _ => {},
            }
        }
    r = N / 2;
    c = N / 2;
        for part in wire2.split(',') {
            let dir = part.chars().next().unwrap();
            let amt = part[1..].parse::<usize>().unwrap();
            match dir {
                'R' => {
                    for _ in 0..amt {
                        if thing[r][c] == 1 {
                            intersections.push((r, c));
                            thing[r][c] = 3;
                        } else {
                            thing[r][c] = 2;
                        }
                        c += 1;
                    }
                },
                'U' => {
                    for _ in 0..amt {
                        if thing[r][c] == 1 {
                            intersections.push((r, c));
                            thing[r][c] = 3;
                        } else {
                            thing[r][c] = 2;
                        }
                        r -= 1;
                    }
                },
                'L' => {
                    for _ in 0..amt {
                        if thing[r][c] == 1 {
                            intersections.push((r, c));
                            thing[r][c] = 3;
                        } else {
                            thing[r][c] = 2;
                        }
                        c -= 1;
                    }
                },
                'D' => {
                    for _ in 0..amt {
                        if thing[r][c] == 1 {
                            intersections.push((r, c));
                            thing[r][c] = 3;
                        } else {
                            thing[r][c] = 2;
                        }
                        r += 1;
                    }
                },
                _ => {},
            }
        }

        let mut best = std::i64::MAX;
        println!("{:?}", intersections);
        for (r, c) in intersections {
            let dist = ((N / 2) as i64 - r as i64).abs()
            + ((N / 2) as i64 - c as i64).abs();
            if dist > 0 && dist < best {
                best = dist;
            }
        }
        println!("{}", best);
}
