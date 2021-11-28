use wasm_bindgen::prelude::*;

use crate::vm::{weso_parse, weso_run};

#[wasm_bindgen]
#[allow(unused)]
pub fn parse(s: &str) -> String {
    match weso_parse(s) {
        Ok(s) => s.join("\n"),
        Err(e) => e,
    }
}

#[wasm_bindgen]
#[allow(unused)]
pub fn run(s: &str) -> String {
    match weso_run(s) {
        Ok(_) => String::new(),
        Err(e) => e,
    }
}
