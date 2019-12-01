extern crate md5;
pub extern crate hex;
extern crate sha1;
extern crate digest;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub use digest::Digest;
pub use md5::Md5;
pub use sha1::Sha1;

pub use std::collections::{HashMap, HashSet};

pub fn get_lines(filename: &str) -> impl Iterator<Item = io::Result<String>> {
    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);
    f.lines()
}

pub fn read_file(filename: &str) -> impl Iterator<Item = String> {
    get_lines(filename).map(|s| s.unwrap())
}

pub fn split_ws(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

pub fn md5(input: &[u8]) -> Vec<u8> {
    Md5::digest(input).iter().cloned().collect()
}

pub fn md5hex<'a, T: Into<&'a [u8]>>(input: T) -> String {
    hex::encode(Md5::digest(input.into()))
}

pub fn sha1(input: &[u8]) -> Vec<u8> {
    Sha1::digest(input).iter().cloned().collect()
}

pub fn sha1hex<'a, T: Into<&'a [u8]>>(input: T) -> String {
    hex::encode(Sha1::digest(input.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let result = md5("The quick brown fox jumps over the lazy dog".as_bytes());
        assert_eq!(hex::encode(result), "9e107d9d372bb6826bd81d3542a419d6");
    }

    #[test]
    fn test_md5hex() {
        let result = md5hex("The quick brown fox jumps over the lazy dog".as_bytes());
        assert_eq!(result, "9e107d9d372bb6826bd81d3542a419d6");
    }

    #[test]
    fn test_sha1() {
        let result = sha1("The quick brown fox jumps over the lazy dog".as_bytes());
        assert_eq!(hex::encode(result), "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
    }

    #[test]
    fn test_sha1hex() {
        let result = sha1hex("The quick brown fox jumps over the lazy dog".as_bytes());
        assert_eq!(result, "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
    }
}
