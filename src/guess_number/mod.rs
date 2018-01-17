// guess number

mod guess_number_mod;

use self::guess_number_mod::*;

pub fn init() {
    let num = get_input();
    println!("num is: {}", num);
}
