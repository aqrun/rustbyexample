
use std::collections::HashMap;

fn main(){
    let mut data = HashMap::new();
    data.insert("a", 32);

    println!("{}", data.get("a"));
}