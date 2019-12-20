use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Token {
    Door(char),
    Key(char),
    Entrance,
    Wall,
}

fn main() {
    let input = include_str!("../input.txt");
    let mut tokens = HashMap::new();
    let mut keys = HashMap::new();
    let mut doors = HashMap::new();
    let mut y_max = 0;
    let mut x_max = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    tokens.insert((x,y), Token::Entrance);
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
}
