#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            if c == '#' {
                row.push(1);
            } else {
                row.push(0);
            }
        }
        grid.push(row);
    }

    let mut counts = grid.clone();
    let mut best = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 1 {
                counts[r][c] = count_visible(&grid, r, c);
            }
            if counts[r][c] > best {
                best = counts[r][c];
            }
        }
    }
    println!("{}", best);
}

fn count_visible(grid: &Vec<Vec<u32>>, r: usize, c: usize) -> u32 {
    let mut result = 0;
    for row in 0..grid.len() {
        let rr = &grid[row];
        for col in 0..rr.len() {
            if r == row && c == col {
                continue;
            }
            let cc = &rr[col];
            if *cc == 1 {
                if is_visible(grid, r, c, row, col) {
                    result += 1;
                }
            }
        }
    }
    result
}

fn is_visible(grid: &Vec<Vec<u32>>, r: usize, c: usize, row: usize, col: usize) -> bool {
    let h = row as isize - r as isize;
    let v = col as isize - c as isize;

    let g = gcd(h.abs(), v.abs());
    let h = h / g;
    let v = v / g;

    let mut idx = 1isize;
    loop {
        let rr = (r as isize + idx*h) as usize;
        let cc = (c as isize + idx*v) as usize;

        if rr == row && cc == col {
            return true;
        }
        if grid[rr][cc] == 1 {
            return false;
        }
        idx += 1;
    }
}

fn gcd(a: isize, b: isize) -> isize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    if a > b {
        return gcd(a - b, b);
    }
    if a < b {
        return gcd(a, b - a);
    }
    if a == b {
        return a;
    }
    panic!("oops");
}
