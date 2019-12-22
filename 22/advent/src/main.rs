#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Step {
    Deal,
    Incr(usize),
    Cut(isize),
}

const N: usize = 10007;

fn main() {
    let input = include_str!("../input.txt");
    let mut steps = Vec::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        match parts[0] {
            "deal" => {
                match parts[1] {
                    "with" => {
                        steps.push(Step::Incr(parts[3].parse::<usize>().unwrap()));
                    },
                    "into" => {
                        steps.push(Step::Deal);
                    },
                    _ => panic!("bad input: {:?}", line),
                }
            }
            "cut" => {
                steps.push(Step::Cut(parts[1].parse::<isize>().unwrap()));
            },
            _ => panic!("bad input: {:?}", line),
        }
    }

    let mut deck = (0..N).collect::<Vec<usize>>();

    for step in steps.iter() {
        take_step(step, &mut deck);
    }
    let mut result = 0;
    let goal = 2019;
    for (i, elem) in deck.iter().enumerate() {
        if *elem == goal {
            result =  i;
            break;
        }
    }
    println!("{}", result);
}

fn take_step(step: &Step, deck: &mut Vec<usize>) {
    match step {
        Step::Deal => {
            deck.reverse();
        },
        Step::Incr(i) => {
            let mut result = vec![0; N];
            let mut idx = 0;
            for elem in deck.iter() {
                result[idx] = *elem;
                idx = (idx + i) % deck.len();
            }
            for (i, elem) in result.into_iter().enumerate() {
                deck[i] = elem;
            }
        },
        Step::Cut(i) => {
            if *i > 0 {
                deck.rotate_left(*i as usize);
            } else {
                deck.rotate_right((-*i) as usize);
            }
        }
    }
}
