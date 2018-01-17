use std::io;
use std::str::FromStr;

pub fn get_input() -> i32 {
    println!("Input a number: ");
    let mut line = String::new();
    let mut num: i32 = 0;
    match io::stdin().read_line(&mut line) {
        Ok(number) => {
            num = match i32::from_str(line.as_slice()) {
                Ok(data) => {data},
                Err(_e) => {
                    println!("{:?}", _e);
                    get_input()
                }
            }
        },
        Err(_e) => {
            println!("Input error!");
            get_input();
        }
    }
    println!("readed {}", num);
    32
}
