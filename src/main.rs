//use std::thread;
//use std::time::Duration;
//use rand::*;
use rand::random_range;
use std::io::{self, BufRead, Write};
use std::thread;
use std::time::Duration;
//use std::fs::File;
use std::fs::{File, OpenOptions};

fn main() {
    let mut times: i32 = 0;

    //println!("");
    loop {
        let rancar: char = random_range('a'..='z') as char;
        let ranber: i32 = random_range(0..=99999);
        times += 1;

        print!("{}{}", ranber, rancar);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        //        thread::sleep(Duration::from_secs(1));

        if times == 3 {
            break;
        }
    }
}
