// variable bindings
mod m_mut;
mod m_scope;
mod m_declare;

pub fn init(){
    run();
    m_mut::run();
    m_scope::run();
    m_declare::run();
}

fn run(){
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
}