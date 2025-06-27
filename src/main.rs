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
    let mut samp1 = 5;
    let print_var_closure = || println!("I MUST PRINT {}", samp1);
    print_var_closure(); // This prints
    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);
    samp1 = 10;
    println!("samp1 = {}", samp1);


}
