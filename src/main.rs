//use std::thread;
//use std::time::Duration;
//use rand::*;
use rand::random_range;
//use std::io::{self, BufRead, Write};
//use std::thread;
//use std::time::Duration;
//use std::fs;


fn main() {
    let spechar: [char; 6] = ['!', '@', '#', '$', '%', '&'];
    let mut times: i32 = 0;
    let mut final_pass = String::new();
    let pass_length: i32 = 9;
// Let it mut when implementing pass length input
    //println!("");
    loop {
        let ranw:i32 = random_range(0..=2);
        

        times += 1;
        if ranw == 0 {
            let ranber: i32 = random_range(0..=9);
            final_pass.push_str(&ranber.to_string());
        } else if ranw == 1 {
            let rancar: char = random_range('a'..='z');
            final_pass.push(rancar);
        } else if ranw == 2 {
            let rannumspe = random_range(0..6);
            final_pass.push(spechar[rannumspe]);
            
        }

//        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        //        thread::sleep(Duration::from_secs(1));

        if times >= pass_length {
//            fs::write("senhas.txt", &final_pass).expect("Error on write");
            println!("{}", final_pass);
            break;
        }
    }
}
