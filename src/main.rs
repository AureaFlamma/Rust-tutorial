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


fn main() {
    const PI: f32 = 3.141592;
    // Shape is a _trait_ - it defines the general data types ( the contract)
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {length: f32, width: f32};
    struct Circle {length: f32, width: f32};
    // Rectangle and Circle are structs (concrete types) - they define the specific mechanics of e.g. how area is calculated.
    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }
    // rec and circ are variables; their values are instances of the struct
    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(5.0, 5.0);

    println!("Area of Rectangle, {}", rec.area());
    println!("Area of Circle: {}", circ.area());
}
