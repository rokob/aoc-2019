#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Input,
    Output,
    Running,
    Halt,
}

const DEBUG: bool = false;

fn main() {
    let input = include_str!("../input.txt");
    let mut data = Vec::new();
    for line in input.lines() {
        for part in line.split(',') {
            data.push(part.parse::<isize>().unwrap());
        }
    }

    let instr = "NOT A J\nAND D J\nNOT B T\nAND D T\nAND H T\nOR T J\nNOT C T\nAND D T\nAND E T\nOR T J\nNOT C T\nAND D T\nAND H T\nOR T J\nRUN\n";
    let instr = instr.chars().map(|c| c as u32).collect::<Vec<_>>();
    let prog = Program::new(data);
    let (win, result) = run(instr, prog);
    if win {
        println!("result: {}", result);
    } else {
        println!("fail!!!");
    }
}

fn run(instr: Vec<u32>, mut prog: Program) -> (bool, isize) {
    let mut instr_idx = 0;
    let mut last = 0;
    let mut s = String::new();

    prog.start();
    loop {
        let state = prog.state();
        match state {
            State::Running => prog.run(),
            State::Input => {
                prog.input(instr[instr_idx] as isize);
                instr_idx += 1;
            }
            State::Output => {
                if let Some(output) = prog.output()  {
                    match output {
                        10 => {
                            if DEBUG {
                            s.push('\n');
                            if last == 10 {
                                print!("{}", s);
                                s.clear();
                            }
                            }
                        },
                        c if c < 127 => {
                            if DEBUG {
                            let c = char::from(output as u8);
                            s.push(c);
                            }
                        },
                        _ => return (true, output),
                    }
                    last = output;
            }
            }
            State::Halt => {
                break;
            }
        }
    }
    (false, 0)
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
