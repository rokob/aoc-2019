use std::collections::{HashMap, VecDeque, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Token {
    Door(char),
    Key(char),
    Entrance,
    Wall,
}

fn mark_seen(i: u32, key: char) -> u32 {
    let bit = key as u8 - 97u8;
    i | (1 << bit)
}

fn is_seen(i: u32, key: char) -> bool {
    let bit = key as u8 - 97u8;
    i & (1 << bit) > 0
}

const ROBOTS: usize = 4;

                      //abcdefghijklmnopqrstuvwxyz
const ALL_KEYS: u32 = 0b11111111111111111111111111;

fn main() {
    let input = include_str!("../input.txt");
    let mut tokens = HashMap::new();
    let mut doors = HashMap::new();
    let mut y_max = 0;
    let mut x_max = 0;
    let mut starts = [(0, 0, 0); ROBOTS];
    let mut start_idx = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    tokens.insert((x,y), Token::Entrance);
                    starts[start_idx] = (x, y, 0);
                    start_idx += 1;
                },
                '.' => {
                },
                '#' => {
                    tokens.insert((x,y), Token::Wall);
                },
                'a'..='z' => {
                    let t = Token::Key(c);
                    tokens.insert((x,y), t);
                }
                'A'..='Z' => {
                    let t = Token::Door(c.to_ascii_lowercase());
                    tokens.insert((x,y), t);
                    doors.insert(t, (x,y));
                }
                _ => panic!("bad input: '{}'", c),
            }
            x_max = if x > x_max { x } else { x_max };
        }
        y_max = if y > y_max { y } else { y_max };
    }

    for ((x, y), token) in tokens.iter() {
        let q = if *x <= starts[0].0 && *y <= starts[0].1 {
            0
        } else if *x >= starts[1].0 && *y <= starts[1].1 {
            1
        } else if *x <= starts[2].0 && *y >= starts[2].1 {
            2
        } else {
            3
        };
        match token {
            Token::Key(c) => {
                for i in 0..ROBOTS {
                    if i == q {
                        continue;
                    }
                    starts[i].2 = mark_seen(starts[i].2, *c);
                }
            },
            _ => {},
        }
    }

    let mut final_result = 0;
    for start in starts.iter() {
        let result = shortest_path(&tokens, *start);
        final_result += result;
    }
    println!("{}", final_result);
}

fn shortest_path(tokens: &HashMap<(usize, usize), Token>, start: (usize, usize, u32)) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(start);
    seen.insert(start);
    let mut paths = HashMap::new();
    let mut terminals = Vec::new();
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        match tokens.get(&(current.0, current.1)) {
            None | Some(Token::Entrance) => {
                for (row, col) in dirs.iter() {
                    let pos = ((current.0 as isize + row) as usize, (current.1 as isize + col) as usize, current.2);
                    if seen.insert(pos) {
                        queue.push_back(pos);
                        paths.insert(pos, current);
                    }
                }
            },
            Some(Token::Wall) => {},
            Some(Token::Door(c)) => {
                if is_seen(current.2, *c) {
                    for (row, col) in dirs.iter() {
                        let pos = ((current.0 as isize + row) as usize, (current.1 as isize + col) as usize, current.2);
                        if seen.insert(pos) {
                            queue.push_back(pos);
                            paths.insert(pos, current);
                        }
                    }
                }
            },
            Some(Token::Key(c)) => {
                let new_keys = mark_seen(current.2, *c);
                if new_keys == ALL_KEYS {
                    terminals.push(current);
                    continue;
                }
                for (row, col) in dirs.iter() {
                    let pos = ((current.0 as isize + row) as usize, (current.1 as isize + col) as usize, new_keys);
                    if seen.insert(pos) {
                        queue.push_back(pos);
                        paths.insert(pos, current);
                    }
                }
            },
        }
    }
    let mut best = std::usize::MAX;
    for term in terminals {
        let length = path_length(start, term, &paths);
        if length < best {
            best = length;
        }
    }
    best
}

type Pos = (usize, usize, u32);

fn path_length(start: Pos, end: Pos, paths: &HashMap<Pos, Pos>) -> usize {
    let mut current = end;
    let mut counter = 0;
    loop {
        if current == start {
            return counter;
        }
        counter += 1;
        current = *paths.get(&current).unwrap();
    }
}
