#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Kind {
    Wall,
    Empty,
    Oxygen,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn from(val: isize) -> Self {
        match val {
            1 => Dir::North,
            2 => Dir::South,
            3 => Dir::West,
            4 => Dir::East,
            _ => panic!("bad dir input"),
        }
    }
}

fn next_pos(pos: (isize, isize), dir: Dir) -> (isize, isize) {
    match dir {
        Dir::North => (pos.0 - 1, pos.1),
        Dir::South => (pos.0 + 1, pos.1),
        Dir::East => (pos.0, pos.1 + 1),
        Dir::West => (pos.0, pos.1 - 1),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Input,
    Output,
    Halt,
}

fn main() {
    let input = include_str!("../input.txt");
    let mut data = Vec::new();
    for line in input.lines() {
        for part in line.split(',') {
            data.push(part.parse::<isize>().unwrap());
        }
    }

    let mut prog = Program::new(data);
    let mut tiles: HashMap<(isize, isize), Kind> = HashMap::new();
    let pos = (0, 0);
    tiles.insert(pos, Kind::Empty);
    let mut queue = VecDeque::new();
    prog.start();
    queue.push_back((0, pos, prog));
    let mut path = HashMap::new();
    let mut found = (0, 0);

    while !queue.is_empty() {
        let (last_input, pos, mut current) = queue.pop_front().unwrap();
        let state = current.state();
        match state {
            State::Input => {
                for dir in [1,2,3,4].iter() {
                    let p = next_pos(pos, Dir::from(*dir));
                    if !tiles.contains_key(&p) {
                        let mut new_prog = current.clone();
                        new_prog.input(*dir);
                        path.insert(p, pos);
                        queue.push_back((*dir, pos, new_prog));
                    }
                }
            },
            State::Output => {
                let p = next_pos(pos, Dir::from(last_input));
                if let Some(output) = current.output() {
                match output {
                    0 => {
                        // wall
                        tiles.insert(p, Kind::Wall);
                    }
                    1 => {
                        // moved
                        queue.push_back((0, p, current));
                        tiles.insert(p, Kind::Empty);
                    }
                    2 => {
                        // moved and found
                        queue.push_back((0, p, current));
                        tiles.insert(p, Kind::Oxygen);
                        found = p;
                    }
                    _ => panic!("bad output: {}", output),
                }
                }
            },
            State::Halt => {
                break;
            },
        }
    }
    print_tiles(&tiles, (0, 0));
    let mut counter = 0;
    loop {
        found = *path.get(&found).unwrap();
        counter += 1;
        if found == (0, 0) {
            break;
        }
    }
    println!("{}", counter);

    let result = flood(&mut tiles);
    println!("{}", result);
}

fn flood(
    tiles: &mut HashMap<(isize, isize), Kind>,
) -> usize {
    let mut counter = 0;
    loop {
        let mut too_fill = Vec::new();
        for (pos, kind) in tiles.iter() {
            if *kind == Kind::Oxygen {
                let left = (pos.0 - 1, pos.1);
                let right = (pos.0 + 1, pos.1);
                let up = (pos.0, pos.1 - 1);
                let down = (pos.0, pos.1 + 1);
                for dir in [left, right, up, down].into_iter() {
                    if let Some(k) = tiles.get(dir) {
                        if *k == Kind::Empty {
                            too_fill.push(*dir);
                        }
                    }
                }
            }
        }
        if too_fill.is_empty() {
            return counter;
        }
        counter += 1;
        for space in too_fill {
            tiles.insert(space, Kind::Oxygen);
        }
    }
}

#[allow(dead_code)]
fn interactive(mut prog: Program) {
    let mut tiles: HashMap<(isize, isize), Kind> = HashMap::new();
    let mut last_input = 0;
    let mut pos = (0, 0);
    tiles.insert(pos, Kind::Empty);

    prog.start();
    while prog.running {
        let state = prog.state();
        match state {
            State::Input => {
                print_tiles(&tiles, pos);
                let mut user_input = String::new();
                std::io::stdin().read_line(&mut user_input).unwrap();
                let in_ = user_input.trim_end();
                let c = in_.chars().next().unwrap();
                let in_ = match c {
                    'n' => 1,
                    's' => 2,
                    'w' => 3,
                    'e' => 4,
                    _ => panic!("bad input"),
                };
                last_input = in_;
                prog.input(in_);
            }
            State::Output => {
                if let Some(output) = prog.output() {
                    let p = next_pos(pos, Dir::from(last_input));
                    match output {
                        0 => {
                            // wall
                            tiles.insert(p, Kind::Wall);
                        }
                        1 => {
                            // moved
                            pos = p;
                            tiles.insert(p, Kind::Empty);
                        }
                        2 => {
                            // moved and found
                            pos = p;
                            tiles.insert(p, Kind::Oxygen);
                        }
                        _ => panic!("bad output: {}", output),
                    }

                }
            }
            State::Halt => {
                break;
            }
        }
    }
}

fn find_extremes(
    tiles: &HashMap<(isize, isize), Kind>,
) -> (isize, isize, isize, isize) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for (p, _) in tiles.iter() {
    if min_x > p.0 {
        min_x = p.0;
    }
    if min_y > p.1 {
        min_y = p.1;
    }
    if max_x < p.0 {
        max_x = p.0;
    }
    if max_y < p.1 {
        max_y = p.1;
    }
    }
    (min_x, min_y, max_x, max_y)
}

fn print_tiles(
    tiles: &HashMap<(isize, isize), Kind>,
    pos: (isize, isize),
) {
    let (min_x, min_y, max_x, max_y) = find_extremes(tiles);
    let mut screen =
        vec![vec![Kind::Unknown; (max_y - min_y + 1) as usize]; (max_x - min_x + 1) as usize];
    for (p, kind) in tiles.iter() {
        screen[(p.0 - min_x) as usize][(p.1 - min_y) as usize] = *kind;
    }

    let orig = ((0 - min_x) as usize, (0 - min_y) as usize);
    let p = ((pos.0 - min_x) as usize, (pos.1 - min_y) as usize);
    for (r, row) in screen.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            match col {
                Kind::Wall => print!("#"),
                Kind::Empty => {
                    if r == orig.0 && c == orig.1 && orig.0 == p.0 && orig.1 == p.1 {
                        print!("@")
                    } else if r == orig.0 && c == orig.1 {
                        print!("$")
                    } else if r == p.0 && c == p.1 {
                        print!("D")
                    } else {
                        print!(".")
                    }
                }
                Kind::Oxygen => {
                    if r == p.0 && c == p.1 {
                        print!("X")
                    } else {
                        print!("O")
                    }
                }
                Kind::Unknown => {
                    if r == p.0 && c == p.1 {
                        print!("D")
                    } else {
                        print!(" ")
                    }
                }
            }
        }
        println!("");
    }
}

#[derive(Debug, Clone)]
struct Program {
    ip: usize,
    rb: isize,
    data: Vec<isize>,
    needs_input: bool,
    has_output: bool,
    running: bool,
}

impl Program {
    fn new(data: Vec<isize>) -> Self {
        Program {
            ip: 0,
            rb: 0,
            data,
            needs_input: false,
            has_output: false,
            running: true,
        }
    }

    fn start(&mut self) {
        self.run();
    }

    fn state(&self) -> State {
        if !self.running {
            return State::Halt;
        }
        if self.needs_input {
            return State::Input;
        }
        if self.has_output {
            return State::Output;
        }
        panic!("bad state");
    }

    fn input(&mut self, in_: isize) {
        if !self.running {
            return;
        }
        let (_inst, a, _b, _c) = parse(self.data[self.ip]);
        let xi = self.index(a, 1);
        self.data[xi] = in_;
        self.ip += 2;
        self.needs_input = false;
        self.run();
    }

    fn output(&mut self) -> Option<isize> {
        if !self.has_output || !self.running {
            return None;
        }
        let (_inst, a, _b, _c) = parse(self.data[self.ip]);
        let xi = self.index(a, 1);
        let x = self.data[xi];
        self.has_output = false;
        self.ip += 2;
        self.run();
        Some(x)
    }

    fn run(&mut self) {
        if !self.running {
            return;
        }

        let (inst, a, b, c) = parse(self.data[self.ip]);
        if inst == 99 {
            self.running = false;
            return;
        }
        let xi = self.index(a, 1);
        let x = self.data[xi];
        match inst {
            1 => {
                let y = self.read(b, 2);
                let z = self.index(c, 3);
                self.data[z] = x + y;
                self.ip += 4;
            }
            2 => {
                let y = self.read(b, 2);
                let z = self.index(c, 3);
                self.data[z] = x * y;
                self.ip += 4;
            }
            3 => {
                self.needs_input = true;
                return;
            }
            4 => {
                self.has_output = true;
                return;
            }
            5 => {
                let y = self.read(b, 2);
                if x != 0 {
                    self.ip = y as usize;
                } else {
                    self.ip += 3;
                }
            }
            6 => {
                let y = self.read(b, 2);
                if x == 0 {
                    self.ip = y as usize;
                } else {
                    self.ip += 3;
                }
            }
            7 | 8 => {
                let y = self.read(b, 2);
                let z = self.index(c, 3);
                if (x < y && inst == 7) || (x == y && inst == 8) {
                    self.data[z] = 1;
                } else {
                    self.data[z] = 0;
                }
                self.ip += 4;
            }
            9 => {
                self.ip += 2;
                self.rb += x;
            }
            _ => panic!("unk::{}", inst),
        }
        self.run();
    }

    #[inline]
    fn maybe_extend(&mut self, idx: usize) {
        if idx < self.data.len() {
            return;
        }
        self.data.resize(self.data.len() * 2, 0);
    }

    #[inline]
    fn index(&mut self, mode: isize, offset: usize) -> usize {
        if mode == 0 {
            let i = self.data[self.ip + offset] as usize;
            self.maybe_extend(i as usize);
            i
        } else if mode == 2 {
            let i = (self.rb + self.data[self.ip + offset]) as usize;
            self.maybe_extend(i as usize);
            i
        } else {
            self.ip + offset
        }
    }

    #[inline]
    fn read(&mut self, mode: isize, offset: usize) -> isize {
        let i = self.index(mode, offset);
        self.data[i]
    }
}

fn parse(mut inst: isize) -> (isize, isize, isize, isize) {
    let x = inst % 100;
    inst /= 100;
    let a = inst % 10;
    inst /= 10;
    let b = inst % 10;
    inst /= 10;
    let c = inst % 10;
    (x, a, b, c)
}
