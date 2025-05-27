#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// macros have an exclamation mark.

fn main() {
    println!("what is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(buf: &mut name) Result<usize, Error>
        .expect("Failed to read line");
    println!("Hello, world!");
}
