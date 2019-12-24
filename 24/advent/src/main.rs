#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

const N: usize = 5;

fn main() {
    let input = include_str!("../input.txt");
    let mut data = [0u8; N * N];
    let mut idx = 0;
    for line in input.lines() {
        for c in line.chars() {
            if c == '#' {
                data[idx] = 1;
            }
            idx += 1;
        }
    }

    let mut grid = VecDeque::new();
    grid.push_back(data);
    for _ in 0..200 {
        grid = update(grid);
    }
    println!("{}", count(&grid));
}

fn count(grid: &VecDeque<[u8; N * N]>) -> u64 {
    let mut result = 0;
    for g in grid.iter() {
        for elem in g.iter() {
            result += *elem as u64;
        }
    }
    result
}

fn update(grid: VecDeque<[u8; N * N]>) -> VecDeque<[u8; N * N]> {
    let mut result = VecDeque::new();

    for i in 0..grid.len() {
        let empty = [0u8; N * N];
        let outer = if i == 0 { &empty } else { &grid[i - 1] };
        let inner = if i == grid.len() - 1 {
            &empty
        } else {
            &grid[i + 1]
        };
        let (new_grid, _) = one_update(&grid[i], outer, inner);
        result.push_back(new_grid);
    }

    let first = [0u8; N * N];
    let (new_first, any_bugs) = one_update(&first, &[0u8; N * N], &grid[0]);
    if any_bugs {
        result.push_front(new_first);
    }
    let last = [0u8; N * N];
    let (new_last, any_bugs_last) = one_update(&last, &grid[grid.len() - 1], &[0u8; N * N]);
    if any_bugs_last {
        result.push_back(new_last);
    }
    result
}

fn one_update(data: &[u8; N * N], outer: &[u8; N * N], inner: &[u8; N * N]) -> ([u8; N * N], bool) {
    let mut result = [0u8; N * N];
    let mut any = false;
    for r in 0..N {
        for c in 0..N {
            if r == 2 && c == 2 {
                continue;
            }
            let mut adj = if r > 0 {
                data[(r - 1) * N + c]
            } else {
                outer[N + 2]
            } + if r < N - 1 {
                data[(r + 1) * N + c]
            } else {
                outer[3 * N + 2]
            } + if c > 0 {
                data[r * N + c - 1]
            } else {
                outer[2 * N + 1]
            } + if c < N - 1 {
                data[r * N + c + 1]
            } else {
                outer[2 * N + 3]
            };
            if r == 1 && c == 2 {
                for cc in 0..N {
                    adj += inner[cc];
                }
            }
            if r == 2 && c == 1 {
                for rr in 0..N {
                    adj += inner[rr * N];
                }
            }
            if r == 2 && c == 3 {
                for rr in 0..N {
                    adj += inner[rr * N + N - 1];
                }
            }
            if r == 3 && c == 2 {
                for cc in 0..N {
                    adj += inner[(N - 1) * N + cc];
                }
            }
            if data[r * N + c] == 1 {
                result[r * N + c] = if adj == 1 {
                    any = true;
                    1
                } else {
                    0
                };
            } else {
                result[r * N + c] = if adj == 1 || adj == 2 {
                    any = true;
                    1
                } else {
                    0
                };
            }
        }
    }
    (result, any)
}
