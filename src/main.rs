#[macro_use]
extern crate lazy_static;

mod base;
mod parser;
mod vm;

#[test]
fn test1() {
    use std::fs;

    let code = fs::read_to_string("examples/exp1.weso").unwrap();
    match vm::weso_run(&code) {
        Ok(_) => (),
        Err(why) => panic!("{}", why),
    }
}

fn main() {
    use std::fs;

    let code = fs::read_to_string("examples/exp1.weso").unwrap();
    match vm::weso_run(&code) {
        Ok(_) => (),
        Err(why) => panic!("{}", why),
    }
}
