mod comment;
mod print;

pub fn mod_init() {
    comment::run();
    print::init();
}