use std::collections::HashMap;

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

                      //abcdefghijklmnopqrstuvwxyz
const ALL_KEYS: u32 = 0b11111111111111111111111111;

fn main() {
    let input = include_str!("../input.txt");
    let mut tokens = HashMap::new();
    let mut keys = HashMap::new();
    let mut doors = HashMap::new();
    let mut y_max = 0;
    let mut x_max = 0;
    let mut start = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    tokens.insert((x,y), Token::Entrance);
                    start = (x, y);
                },
                '.' => {
                },
                '#' => {
                    tokens.insert((x,y), Token::Wall);
                },
                'a'..='z' => {
                    let t = Token::Key(c);
                    tokens.insert((x,y), t);
                    keys.insert(t, (x,y));
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

    /*
    for y in 0..=y_max {
        for x in 0..=x_max {
            match tokens.get(&(x,y)) {
                Some(Token::Entrance) => print!("@"),
                Some(Token::Wall) => print!("#"),
                Some(Token::Door(c)) => print!("{}", c.to_ascii_uppercase()),
                Some(Token::Key(c)) => print!("{}", c),
                None => print!("."),
            }
        }
        println!("");
    }
    */
    let result = shortest_path(&tokens, &keys, start);
}

fn shortest_path(tokens: &HashMap<(usize, usize), Token>, keys: &HashMap<Token, (usize, usize)>, start: (usize, usize, u32)) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back(start);
    seen.insert(start);
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        match tokens.get(&(current.0, current.1)) {
            None => {// floor
                for (row, col) in dirs.iter() {
                    let pos = ((current.0 as isize + row) as usize, (current.1 as isize + col) as usize, current.2);
                    if seen.insert(pos) {
                        queue.push_back(pos);
                    }
                }
            },
            Some(Token::Wall) => {},
            Some(Token::Entrance) => {},
            Some(Token::Door(c)) => {
                if is_seen(current.2, c) {
                    for (row, col) in dirs.iter() {
                        let pos = ((current.0 as isize + row) as usize, (current.1 as isize + col) as usize, current.2);
                        if seen.insert(pos) {
                            queue.push_back(pos);
                        }
                    }
                }
            },
            Some(Token::Key(c)) => {
                let new_keys = mark_seen(current.2, c);
                for (row, col) in dirs.iter() {
                    let pos = ((current.0 as isize + row) as usize, (current.1 as isize + col) as usize, new_keys);
                    if seen.insert(pos) {
                        queue.push_back(pos);
                    }
                }
            },
        }
    }
}
