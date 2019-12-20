#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Floor,
    Start,
    End,
    Portal(usize),
}

fn main() {
    let input = include_str!("../input.txt");
    let mut prev = ' ';
    let mut grid = HashMap::new();
    let mut portal_mapper = HashMap::new();
    let mut portals = HashMap::new();
    let mut portal_data = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    grid.insert((row, col), Tile::Wall);
                }
                '.' => {
                    grid.insert((row, col), Tile::Floor);
                }
                ' ' => {}
                _ => {
                    if prev == ' ' {
                        if row > 0 {
                            match portal_mapper.get(&(row - 1, col)) {
                                Some(cc) => {
                                    if *cc == 'A' && c == 'A' {
                                        start = (row, col);
                                    } else if *cc == 'Z' && c == 'Z' {
                                        end = (row, col);
                                    } else {
                                        let e =
                                            portals.entry((*cc, c)).or_insert(portal_data.len());
                                        if *e == portal_data.len() {
                                            portal_data.push(((row, col), (0, 0)));
                                        } else {
                                            portal_data[*e].1 = (row, col);
                                        }
                                    }
                                }
                                None => {}
                            }
                        }
                        portal_mapper.insert((row, col), c);
                    } else if prev == '_' || prev == '.' || prev == '#' {
                    } else {
                        if prev == 'A' && c == 'A' {
                            start = (row, col);
                        } else if prev == 'Z' && c == 'Z' {
                            end = (row, col);
                        } else {
                            let e = portals.entry((prev, c)).or_insert(portal_data.len());
                            if *e == portal_data.len() {
                                portal_data.push(((row, col), (0, 0)));
                            } else {
                                portal_data[*e].1 = (row, col);
                            }
                        }
                    }
                }
            }
            prev = c;
        }
        prev = '_';
    }
    println!("parsed...");


    for (i, ((r0, c0), (r1, c1))) in portal_data.iter().enumerate() {
        if let Some(Tile::Floor) = grid.get(&(*r0 + 1, *c0)) {
            grid.insert((*r0 + 1, *c0), Tile::Portal(i));
        }
        if *r0 > 1 {
            if let Some(Tile::Floor) = grid.get(&(*r0 - 2, *c0)) {
                grid.insert((*r0 - 2, *c0), Tile::Portal(i));
            }
        }
        if let Some(Tile::Floor) = grid.get(&(*r0, *c0 + 1)) {
            grid.insert((*r0, *c0 + 1), Tile::Portal(i));
        }
        if *c0 > 1 {
            if let Some(Tile::Floor) = grid.get(&(*r0, *c0 - 2)) {
                grid.insert((*r0, *c0 - 2), Tile::Portal(i));
            }
        }

        if let Some(Tile::Floor) = grid.get(&(*r1 + 1, *c1)) {
            grid.insert((*r1 + 1, *c1), Tile::Portal(i));
        }
        if *r1 > 1 {
            if let Some(Tile::Floor) = grid.get(&(*r1 - 2, *c1)) {
                grid.insert((*r1 - 2, *c1), Tile::Portal(i));
            }
        }
        if let Some(Tile::Floor) = grid.get(&(*r1, *c1 + 1)) {
            grid.insert((*r1, *c1 + 1), Tile::Portal(i));
        }
        if *c1 > 1 {
            if let Some(Tile::Floor) = grid.get(&(*r1, *c1 - 2)) {
                grid.insert((*r1, *c1 - 2), Tile::Portal(i));
            }
        }
    }

    println!("got portals...");

    if let Some(Tile::Floor) = grid.get(&(start.0 + 1, start.1)) {
        grid.insert((start.0 + 1, start.1), Tile::Start);
    }
    if start.0 > 1 {
        if let Some(Tile::Floor) = grid.get(&(start.0 - 2, start.1)) {
            grid.insert((start.0 - 2, start.1), Tile::Start);
        }
    }
    if let Some(Tile::Floor) = grid.get(&(start.0, start.1 + 1)) {
        grid.insert((start.0, start.1 + 1), Tile::Start);
    }
    if start.1 > 1 {
        if let Some(Tile::Floor) = grid.get(&(start.0, start.1 - 2)) {
            grid.insert((start.0, start.1 - 2), Tile::Start);
        }
    }

    if let Some(Tile::Floor) = grid.get(&(end.0 + 1, end.1)) {
        grid.insert((end.0 + 1, end.1), Tile::End);
    }
    if end.0 > 1 {
        if let Some(Tile::Floor) = grid.get(&(end.0 - 2, end.1)) {
            grid.insert((end.0 - 2, end.1), Tile::End);
        }
    }
    if let Some(Tile::Floor) = grid.get(&(end.0, end.1 + 1)) {
        grid.insert((end.0, end.1 + 1), Tile::End);
    }
    if end.1 > 1 {
        if let Some(Tile::Floor) = grid.get(&(end.0, end.1 - 2)) {
            grid.insert((end.0, end.1 - 2), Tile::End);
        }
    }

    let mut real_start = (0, 0);
    let mut real_end = (0, 0);

    for ((row, col), tile) in grid.iter() {
        if *tile == Tile::Start {
            real_start = (*row, *col);
        }
        if *tile == Tile::End {
            real_end = (*row, *col);
        }
    }

    println!("start={:?}, end={:?}", real_start, real_end);

    let mut real_portals = vec![vec![];portal_data.len()];
    for ((r, c), tile) in grid.iter() {
        match *tile {
            Tile::Portal(i) => {
                real_portals[i].push((*r, *c));
            }
            _ => {}
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(real_start);

    let mut path = HashMap::new();
    loop {
        if queue.is_empty() {
            break;
        }
        let current = queue.pop_front().unwrap();
        let tile = grid.get(&current).unwrap();
        if *tile == Tile::End {
            break;
        }
        if current.0 > 0 {
            if let Some(t) = grid.get(&(current.0 - 1, current.1)) {
                if *t != Tile::Wall {
                    if !path.contains_key(&(current.0 - 1, current.1)) {
                        path.insert((current.0 - 1, current.1), current);
                        queue.push_back((current.0 - 1, current.1));
                    }
                }
            }
        }
        if let Some(t) = grid.get(&(current.0 + 1, current.1)) {
            if *t != Tile::Wall {
                if !path.contains_key(&(current.0 + 1, current.1)) {
                    path.insert((current.0 + 1, current.1), current);
                    queue.push_back((current.0 + 1, current.1));
                }
            }
        }
        if current.1 > 0 {
            if let Some(t) = grid.get(&(current.0, current.1 - 1)) {
                if *t != Tile::Wall {
                    if !path.contains_key(&(current.0, current.1 - 1)) {
                        path.insert((current.0, current.1 - 1), current);
                        queue.push_back((current.0, current.1 - 1));
                    }
                }
            }
        }
        if let Some(t) = grid.get(&(current.0, current.1 + 1)) {
            if *t != Tile::Wall {
                if !path.contains_key(&(current.0, current.1 + 1)) {
                    path.insert((current.0, current.1 + 1), current);
                    queue.push_back((current.0, current.1 + 1));
                }
            }
        }
        match tile {
            Tile::Portal(i) => {
                let data = &real_portals[*i];
                if current.0 == data[0].0 && current.1 == data[0].1 {
                    if !path.contains_key(&data[1]) {
                        path.insert(data[1], current);
                        queue.push_back(data[1]);
                    }
                } else {
                    if !path.contains_key(&data[0]) {
                        path.insert(data[0], current);
                        queue.push_back(data[0]);
                    }
                }
            },
            _ => {},
        }
    }

    println!("After loop, doing path");

    let mut counter = 0;
    let mut current = real_end;
    loop {
        counter += 1;
        if let Some(pos) = path.get(&current) {
            if *pos == real_start {
                break;
            }
            current = *pos;
        } else {
            panic!("WAT {:?}", current);
        }
    }
    println!("{}", counter);
}
