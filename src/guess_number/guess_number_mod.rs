use std::io;
use std::str::FromStr;

use rand::{thread_rng, Rng};

pub fn get_input() -> i32 {
    println!("Input a number: ");
    let mut line = String::new();
    let mut num: i32 = 0;
    io::stdin().read_line(&mut line).expect("invalid input");
    line = line.trim().to_string();

    match i32::from_str(line.as_str()) {
        Ok(number) => {
            num = number;
        },
        Err(e) => {
            println!("Invalid number! {:?}", e);
            get_input();
        }
    }

    num
}

pub fn generate_random() -> i32 {
    let mut rng = thread_rng();
    let n: i32 = rng.gen_range(0, 100);
    n
}
