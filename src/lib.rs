pub mod hello;
mod primitives;

pub fn module_init(module_name: &str){
    match module_name {
        "hello" => hello::mod_init(),
        "primitives" => primitives::mod_init(),
        _ => println!("Module ({}) not exists!", module_name),
    }
}