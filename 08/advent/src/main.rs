#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

const H: usize = 6;
const W: usize = 25;

fn main() {
    let input = include_str!("../input.txt");
    let line = input.lines().next().unwrap();
    let len = line.len();
    let mut data = Vec::with_capacity(len);
    let layers = len / (W * H);
    for n in line.chars() {
        let v = n.to_digit(10).unwrap() as usize;
        data.push(v);
    }

    let mut best = std::usize::MAX;
    let mut result = 0;
    for layer in 0..layers {
        let (z, o, t) = count(&data, layer);
        if z < best {
            best = z;
            result = o * t;
        }
    }
    println!("{}", result);

    let image = find_image(&data, layers);
    for r in 0..H {
        for c in 0..W {
            let v = image[r*W + c];
            if v == 1 {
                print!(" * ");
            } else {
                print!("   ");
            }
        }
        println!("");
    }
}

fn find_image(data: &Vec<usize>, layers: usize) -> [usize; W*H] {
    let mut result = [2; W*H];
    for i in 0..W*H {
        for layer in 0..layers {
            let v = data[(W*H)*layer + i];
            if v == 2 {
                continue;
            }
            result[i] = v;
            break;
        }
    }
    result
}

fn count(data: &Vec<usize>, layer: usize) -> (usize, usize, usize) {
    let mut z = 0;
    let mut o = 0;
    let mut t = 0;
    for i in 0..W*H {
        match data[(W*H) * layer + i] {
            0 => z += 1,
            1 => o += 1,
            2 => t += 1,
            _ => {},
        }
    }
    (z, o, t)
}
