pub mod hello;
mod primitives;
mod custom_types;
mod variable_bindings;

pub fn module_init(module_name: &str){
    match module_name {
        "hello" => hello::mod_init(),
        "primitives" => primitives::mod_init(),
        "custom_types" => custom_types::run(),
        "variable_bindings" => variable_bindings::init(),
        _ => println!("Module ({}) not exists!", module_name),
    }
}