#![allow(unused)]

use core::prelude::v1;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

// macros have an exclamation mark.

//Data types
// Unsigned integers: u8, u16, u32, u64, u128, usize
// Signed integers: i8, i16, i32, i64, i128, isize
// Arrays mustb e of the same size.
// Arrays must be fixed size.  

// This points to the folder in which the module is.
mod restaurant;
use crate::restaurant::order_food;

fn main() {
    let path = "lines.txt";

    // Result has 2 varients Ok and Err
    // enum Result<T, E> {
    // Ok(T),
    // Err(E), }
    // Where T represents the data typeof the value returns and E
    // the type of error

    // CREATE FILE
    let output = File::create(path);
    // One way of handling errors
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file : {:?}", error);
        }
    };

    // WRITE TO FILE
    // Another way of handling errors. Panic with custom message when error occurs.
    write!(output, "Just some \nRandom words").expect("Failed to write to file");

    // READ FILE
    // Third way of handling errors. Unwrap panics with default message if error occurs. 
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

/* 
// These are equivalent:
let input = File::open(path).unwrap();

// Same as:
let input = match File::open(path) {
    Ok(file) => file,
    Err(error) => panic!("called `Result::unwrap()` on an `Err` value: {:?}", error),
};
*/

}
