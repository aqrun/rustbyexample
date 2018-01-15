// 2. 原生类型

mod literals;
mod tuples;
mod array;

pub fn mod_init() {
    println!("===============primitives::run============");
    run();
    println!("===============literals::run============");
    literals::run();
    println!("===============tuples::run============");
    tuples::run();
    println!("===============array::run============");
    array::run();
}

fn run() {
    let _logical: bool = true;

    let _a_float: f64 = 1.0;
    let _an_integer = 5i32;

    let _default_float = 3.0;
    let _default_integer = 7;

    let mut _mutable = 12;

    //mutable = true;
}