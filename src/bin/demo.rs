
use std::collections::HashMap;

struct Cache {

}

fn main(){
    let mut data = HashMap::new();
    //data.insert("a", 32);
    data.insert("b", "a string");

    println!("{}", data.get("b").unwrap());
}