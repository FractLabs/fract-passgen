//use std::ptr::eq;
//use std::thread;
//use std::time::Duration;
//use rand::*;
use rand::random_range;
use std::io::{self, BufRead, Write};
//use std::fs::File;
use std::fs::{File, OpenOptions};



fn main() {

    let filep = "numbers.txt";
    let filer = File::open(filep).expect("error: open");
    let reader =  io::BufReader::new(filer);
    let mut rw_in = String::new();
    println!("Read or Write? (R/W) ");
    io::stdin()
        .read_line(&mut rw_in)
        .expect("Fail in read");

    let mut num_file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(filep)
    .expect("fail");

    let mut times = 0;

    if rw_in.trim() == "W" {

    loop {
//    let ims = 0;
    let ranber: i32 = random_range(1..=1000000);
//    let tofind: i32 = 1;
    times += 1;

//    thread::sleep(Duration::from_millis(ims));
//    println!("{}", &ranber);
//    let content = String::from("{}", &ranber);
//    num_file.write(content).expect("write fail");
    writeln!(num_file, "{}", &ranber).expect("fail in writing");
    if times == 1000000000 {
        break;
    }
    println!("{}", times)
    }
} else if rw_in.trim() == "R" {
    for linha in reader.lines() {
        let linha = linha.expect("Error");
        println!("{}", linha)
    }
} 
    


}


//eq(&ranber, tofind)