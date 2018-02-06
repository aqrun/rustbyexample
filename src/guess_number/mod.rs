// guess number

mod guess_number_mod;

use self::guess_number_mod::{get_input, generate_random};

pub fn init() {
    let rand = generate_random();
    let mut num: i32;
    loop {
        num = get_input();

        let mut msg = "";
        if num == rand {
            println!("Right");
            break;
        }else{
            if num < rand {
                msg = "               Too small!!";
            } else if num > rand {
                msg = "               Too big!!";
            }
            println!("{}  {}", msg, num);
        }
    }



}
