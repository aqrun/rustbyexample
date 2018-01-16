
#![allow(dead_code)]

mod enum_use;
mod c_like;
mod testcase_linked_list;

pub fn init(){
    run();
    println!("===========enum_use===========");
    enum_use::run();
    println!("===========enum_use===========");
    c_like::run();
    println!("===========enum_use===========");
    testcase_linked_list::run();
}

enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info{ name: String, height: i32 }
}

fn inspect(p: Person) {
    match p {
        Person::Engineer => println!("Is enginee!"),
        Person::Scientist => println!("Is scientist!"),
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a wight of {}.", i),
        Person::Info {name, height} => {
            println!("{} is {} tall!", name, height);
        }
    }
}

fn run(){
    let person = Person::Height(18);
    let amira = Person::Weight(10);
    let dave = Person::Info{ name: "Dave".to_owned(), height:72 };
    let rebecca = Person::Scientist;
    let rohan = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}
