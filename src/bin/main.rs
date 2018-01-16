extern crate rustbyexample;

use rustbyexample::module_init;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("Please input collect module name, \ne.g.: cargo run hello");
    }else{
        module_init(args[1].as_ref());
    }

}
