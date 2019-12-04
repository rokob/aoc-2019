#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

fn main() {
    let mut result = 0;
    for p in 240298..=784956 {
        if check_password(p) {
            result += 1;
        }
    }
    println!("{}", result);
}

/*
 * However, they do remember a few key facts about the password:

It is a six-digit number.
The value is within the range given in your puzzle input.
Two adjacent digits are the same (like 22 in 122345).
Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
Other than the range rule, the following are true:

111111 meets these criteria (double 11, never decreases).
223450 does not meet these criteria (decreasing pair of digits 50).
123789 does not meet these criteria (no double).
How many different passwords within the range given in your puzzle input meet these criteria?

Your puzzle input is 240298-784956.
*/
fn check_password(mut p: u64) -> bool {
    let mut digits = Vec::new();
    while p > 0 {
        digits.push(p % 10);
        p /= 10;
    }
    digits.reverse();
    let mut last = 0;
    let mut has_double = false;
    for d in digits.iter() {
        if *d < last {
            return false;
        }
        if *d == last {
            has_double = true;
        }
        last = *d;
    }
    if has_double {
        if digits[0] == digits[1] && digits[1] != digits[2] {
            return true;
        }
        let l = digits.len() - 1;
        if digits[l] == digits[l-1] && digits[l-1] != digits[l-2] {
            return true;
        }
        for i in 2..digits.len() - 1 {
            if digits[i-2] != digits[i-1] && digits[i-1] == digits[i] && digits[i] != digits[i+1] {
                return true;
            }
        }
        false
    } else {
        false
    }
}
