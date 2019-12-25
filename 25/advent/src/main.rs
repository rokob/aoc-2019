#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Input,
    Output,
    Running,
    Halt,
}

/*
 * Manually played, this was the final inv that makes it work:
 * Items in your inventory:
    - prime number
    - asterisk
    - sand
    - tambourine
*/
fn main() {
    let input = include_str!("../input.txt");
    let mut data = Vec::new();
    for line in input.lines() {
        for part in line.split(',') {
            data.push(part.parse::<isize>().unwrap());
        }
    }

    let mut grid = HashMap::new();

    let mut pos: (isize, isize) = (0, 0);
    let mut instr_idx = 0;
    let mut input_data = Vec::new();
    let mut in_door_output = false;
    let mut s = String::new();
    let mut prog = Program::new(data);
        prog.start();
            loop {
                let state = prog.state();
                match state {
                    State::Running => prog.run(),
                    State::Input => {
                        if instr_idx < input_data.len() {
                            prog.input(input_data[instr_idx] as isize);
                            instr_idx += 1;
                            if instr_idx == input_data.len() {
                                if input_data[0] == ('n' as u32) {
                                    pos = (pos.0, pos.1 - 1);
                                } else if input_data[0] == ('s' as u32) {
                                    pos = (pos.0, pos.1 + 1);
                                } else if input_data[0] == ('e' as u32) {
                                    pos = (pos.0 + 1, pos.1);
                                } else if input_data[0] == ('w' as u32) {
                                    pos = (pos.0 - 1, pos.1);
                                }
                            }
                        } else {
                            //print_grid(&grid, pos);
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        let in_ = input.trim_end();
                        input_data = in_.chars().map(|c| c as u32).collect::<Vec<_>>();
                        input_data.push(10);
                        prog.input(input_data[0] as isize);
                        instr_idx = 1;
                        }
                    }
                    State::Output =>  {
                if let Some(output) = prog.output()  {
                    match output {
                        10 => {
                            if &s[..] == "Doors here lead:" {
                                in_door_output = true;
                                grid.insert(pos, [1, 0, 0, 0, 0]);
                            } else if in_door_output {
                                let e = grid.entry(pos).or_insert([0;5]);
                                    match &s[..] {
                                        "- north" => {
                                            e[1] = 1;
                                            grid.entry((pos.0, pos.1 - 1)).or_insert([0; 5]);
                                        }
                                        "- east" => {
                                            e[2] = 1;
                                            grid.entry((pos.0 + 1, pos.1)).or_insert([0; 5]);
                                        },
                                        "- south" => {
                                            e[3] = 1;
                                            grid.entry((pos.0, pos.1 + 1)).or_insert([0; 5]);
                                        }
                                        "- west" => {
                                            e[4] = 1;
                                            grid.entry((pos.0 - 1, pos.1)).or_insert([0; 5]);
                                        }
                                        _ => {
                                            in_door_output = false;
                                        }
                                    }
                            }
                            s.push('\n');
                            print!("{}", s);
                            s.clear();
                        },
                        c if c < 127 => {
                            let c = char::from(output as u8);
                            s.push(c);
                        },
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

fn print_grid(grid: &HashMap<(isize, isize), [u32;5]>, pos: (isize, isize)) {
    let mut minx = 0;
    let mut miny = 0;
    let mut maxx = 0;
    let mut maxy = 0;
    for (&(x, y), _) in grid.iter() {
        if x < minx { minx = x; }
        if y < miny { miny = y; }
        if x > maxx { maxx = x; }
        if y > maxy { maxy = y; }
    }

    for y in miny..=maxy {
        for x in minx..=maxx {
            match grid.get(&(x, y)) {
                Some(data) => {
                    if pos.0 == x && pos.1 == y {
                        print!(" @ ");
                    } else if data[0] == 1 {
                        print!(" X ");
                    } else {
                        print!(" _ ");
                    }
                },
                None => {
                    print!(" . ");
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
        State::Running
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
