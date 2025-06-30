//use std::thread;
//use std::time::Duration;
//use rand::*;
use rand::random_range;
//use std::io::{self, BufRead, Write};
//use std::thread;
//use std::time::Duration;


fn main() {
    let mut times: i32 = 0;
    let mut final_pass = String::new();
    let mut pass_length = 7;

    //println!("");
    loop {
        let ranw:i32 = random_range(0..=1);
        

        times += 1;
        if ranw == 0 {
            let ranber: i32 = random_range(0..=9);
            final_pass.push_str(&ranber.to_string());
        } else if ranw == 1 {
            let rancar: char = random_range('a'..='z') as char;
            final_pass.push(rancar);
        }

//        print!("{}{}", ranber, rancar);
//        println!("{}", ranw);
//        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        //        thread::sleep(Duration::from_secs(1));

        if times >= pass_length {
            println!("{}", final_pass);
            break;
        }
    }
}
