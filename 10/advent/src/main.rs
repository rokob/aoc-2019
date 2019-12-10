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

    let mut best = 0;
    let mut bestr = 0;
    let mut bestc = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 1 {
                let count = count_visible(&grid, r, c);
                if count > best {
                    best = count;
                    bestr = r;
                    bestc = c;
                }
            }
        }
    }
    println!("{} @ ({}, {})", best, bestc, bestr);
    let mut coords = vec![];
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if r == bestr && c == bestc {
                continue;
            }
            if grid[r][c] == 1 && is_visible(&grid, bestr, bestc, r, c) {
                coords.push((c, r));
            }
        }
    }

    let mut count = 0;
    let mut quad4 = vec![];
    for coord in coords.iter() {
        if coord.0 >= bestc || coord.1 >= bestr {
            count += 1;
        } else {
            quad4.push(coord);
        }
    }
    println!("other count = {}", count);
    println!("quad4 count = {}", quad4.len());
    println!("{} =?= {}", count + quad4.len(), best);

    let mut angles = vec![];
    for coord in quad4.iter() {
        let dx = coord.0 as f64 - bestc as f64;
        let dy = coord.1 as f64 - bestr as f64;
        angles.push((dy.atan2(dx), coord.0, coord.1));
    }
    angles.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (_, c, r) in angles {
        count += 1;
        if count == 200 {
            println!("({}, {}) => result = {}", c, r, c*100 + r);
            break;
        }
    }
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
