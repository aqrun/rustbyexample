//! # 自定义类型
//! Rust 自定义类型主要是通过下面两个关键字来创建：
//! 
//! * `struct`: 定义一个结构体
//! * `enum`: 定义一个枚举类型
//! 
//! 而常量创建可以通过 `const` 和 `static` 关键字来创建。

mod structs;
mod menum;
mod constants;

pub fn run() {
    structs::run();
    println!("==========menum==========");
    menum::init();
    println!("========constants==========");
    constants::run();
}