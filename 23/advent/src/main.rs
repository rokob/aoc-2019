#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{channel, Sender};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Input,
    Output,
    Running,
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

    let mailbox: Arc<Mutex<HashMap<usize, VecDeque<(isize, isize)>>>> = Arc::new(Mutex::new(HashMap::new()));
    let (tx, rx) = channel();
    let prog = Program::new(data);
    for i in 0..50 {
        let (mailbox, tx) = (Arc::clone(&mailbox), tx.clone());
        let mut program = prog.clone();
        program.start();
        thread::spawn(move ||  {
            let mut nic = Nic {
                id: i,
                program,
                mailbox,
                tx,
            };
            let mut given_id = false;
    loop {
        let state = nic.program.state();
        match state {
            State::Running => nic.program.run(),
            State::Input => {
                if !given_id {
                    given_id = true;
                    nic.program.input(nic.id as isize);
                    continue;
                }
                let mut mailbox = nic.mailbox.lock().unwrap();
                let e = mailbox.entry(nic.id).or_insert(VecDeque::new());
                if e.is_empty() {
                    nic.program.input(-1 as isize);
                } else {
                    let in_ = e.pop_front().unwrap();
                    nic.program.input(in_.0);
                    loop {
                        let state_ = nic.program.state();
                        match state_ {
                            State::Running => nic.program.run(),
                            State::Input => {
                                nic.program.input(in_.1);
                                break;
                            },
                            State::Output | State::Halt => panic!("expected second input"),
                        }
                    }
                }
            }
            State::Output => {
                let mut x = 0;
                let mut has_x = false;
                let y: isize;
                let dest = nic.program.output().unwrap();
                loop {
                    let state_ = nic.program.state();
                    match state_ {
                        State::Running => nic.program.run(),
                        State::Output => {
                            if !has_x {
                                x = nic.program.output().unwrap();
                                has_x = true;
                                continue;
                            } 
                            y = nic.program.output().unwrap();
                            break;
                        },
                        State::Input | State::Halt => panic!("expected rest of output"),
                    }
                }
                if dest == 255 {
                    nic.tx.send(y).unwrap();
                } else {
                    let mut mailbox = nic.mailbox.lock().unwrap();
                    let e = mailbox.entry(dest as usize).or_insert(VecDeque::new());
                    e.push_back((x, y));
                }
            }
            State::Halt => {
                break;
            }
        }
    }
        });
    }

    println!("{:?}", rx.recv().unwrap());
}

struct Nic {
    program: Program,
    id: usize,
    mailbox: Arc<Mutex<HashMap<usize, VecDeque<(isize, isize)>>>>,
    tx: Sender<isize>,
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
