#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

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

    let mut y = 100;
    let mut last_y = y;
    loop {
        let (_row, beam_count, _beam_start) = get_row(data.clone(), y);
        if beam_count < 300 {
            last_y = y;
            y *= 2;
        } else {
            break;
        }
    }
    let mut lo = last_y;
    let mut hi = y;
    loop {
        if lo >= hi {
            break;
        }
        let mid = (lo + hi) / 2;
        let (_row_0, count_0, start_0) = get_row(data.clone(), mid - 99);
        let (_row_1, _count_1, start_1) = get_row(data.clone(), mid);
        if start_1 + 99 < start_0 + count_0 as isize {
            // good
            hi = mid;
        } else {
            // bad
            lo = mid + 1;
        }
    }
    let (_, _count, start) = get_row(data.clone(), hi);
    println!("{}", 10000 * start + hi - 99);
    //print_square(data.clone(), hi, start as usize, (hi - 99) as isize);
}

#[allow(dead_code)]
fn print_square(data: Vec<isize>, y: isize, sqx: usize, sqy: isize) {
    let (y_row, count, start) = get_row(data.clone(), y);
    let a = start as usize - 5;
    let b = start as usize + count + 1;
    for i in y - 99..y {
        let (row, _count, _start) = get_row(data.clone(), i);
        for k in a..b {
            if k < row.len() {
                if i >= sqy && k >= sqx && row[k] == 1 && k <= sqx + 99 {
                    print!("O");
                } else {
                    if row[k] == 0 {
                        print!(".")
                    } else {
                        print!("#")
                    };
                }
            } else {
                print!(".");
            }
        }
        println!("");
    }
    for k in a..b {
        if k < y_row.len() {
            if y >= sqy && k >= sqx && y_row[k] == 1 && k <= sqx + 99 {
                print!("O");
            } else {
                if y_row[k] == 0 {
                    print!(".")
                } else {
                    print!("#")
                };
            }
        } else {
            print!(".");
        }
    }
    println!("");
}

fn get_row(data: Vec<isize>, y: isize) -> (Vec<isize>, usize, isize) {
    let mut x = 0;
    let mut result = Vec::new();
    let mut seen_beam = false;
    let mut beam_count = 0;
    let mut beam_start = 0;
    loop {
        let mut in_ = x;
        let mut prog = Program::new(data.clone());
        prog.start();
        while prog.running {
            let state = prog.state();
            match state {
                State::Input => {
                    prog.input(in_);
                    in_ = y;
                }
                State::Output => {
                    if let Some(output) = prog.output() {
                        result.push(output);
                        if seen_beam && output == 0 {
                            return (result, beam_count, beam_start);
                        }
                        if output == 1 {
                            if !seen_beam {
                                seen_beam = true;
                                beam_start = x;
                            }
                            beam_count += 1;
                        }
                    }
                }
                State::Halt => {
                    break;
                }
            }
        }
        x += 1;
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
