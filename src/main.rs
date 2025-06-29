//use std::thread;
//use std::time::Duration;
//use rand::*;
use rand::random_range;
use std::thread;
use std::time::Duration;
use std::io::{self, BufRead, Write};
//use std::fs::File;
use std::fs::{File, OpenOptions};



fn main() {
    loop {
    let rancar:char = random_range('a'..='z') as char;
    let ranber:i32 = random_range(0..=99999);

    println!("{} and {}", ranber, rancar);
    thread::sleep(Duration::from_secs(1));
    }
} 
    