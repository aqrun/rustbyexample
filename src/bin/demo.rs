use std::rc::Rc;

///////////////
struct Cache<T> {
    age: T
}

//////////////////
struct App{
    cache: Option<Cache<i32>>,
    //address: Option<Box<String>>,
    address: Option<String>,
    age: i32,
}

impl App {
    fn new() -> App{
        App{
            cache: None,
            address: None,
            age: 0,
        }
    }
    fn set_address(&mut self, address:String) -> &mut Self{
        //self.address = Some(Box::new(address))
        self.address = Some(address);
        self
    }
    fn set_age(&mut self, age: i32) -> &mut Self {
        self.age = age;
        self
    }
}


////////////////////
fn main(){
    let mut app = App::new();
    app.set_address("test1".to_string())
        .set_age(23);
    println!("init {} : {}", app.address.unwrap(), app.age);
}