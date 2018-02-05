
macro_rules! say_hello {
    () => {
        println!("hello");
    }
}


pub fn mod_init(){
    println!("say_hello! macro start:");
    say_hello!();
}