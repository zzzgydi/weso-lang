pub mod builtin;
pub mod convert;
pub mod global;
pub mod runtime;
pub mod wasm;

use crate::base::func::{FuncManager, Function};
use crate::base::scope::Scope;
use crate::base::types::NewTypeKind;
use crate::parser::lexer::Lexer;
use crate::parser::weso::*;
use runtime::Runtime;
use std::cell::RefCell;
use std::rc::Rc;

// 解析获得指令集
pub fn weso_parse(code: &str) -> Result<Vec<String>, String> {
    let lexer = Lexer::new(code);
    let stmts = wesoParser::new().parse(lexer);
    match stmts {
        // 获得一系列语句
        Ok(stmts) => {
            // let global = Rc::new(RefCell::new(Scope::new(None)));
            let result = convert::parse_stmts(&stmts, 0);
            match result {
                Ok(ins) => {
                    let mut v = Vec::new();
                    for (i, item) in ins.clone().iter().enumerate() {
                        v.push(format!(
                            "{:<6} {:<10} {}",
                            i,
                            item.location.short_show(),
                            item.node
                        ));
                    }
                    Ok(v)
                }
                Err(why) => Err(why),
            }
        }
        Err(why) => Err(format!("{:?}", why)),
    }
}

// 直接运行
pub fn weso_run(code: &str) -> Result<(), String> {
    let lexer = Lexer::new(code);
    let stmts = wesoParser::new().parse(lexer);
    match stmts {
        // 获得一系列语句
        Ok(stmts) => {
            let global = Rc::new(RefCell::new(Scope::new(None)));
            let result = convert::parse_stmts(&stmts, 0);
            match result {
                Ok(ins) => {
                    // 构造一个运行函数
                    let main_func = Function {
                        // name: String::from("__main__"),
                        args: Vec::new(),
                        rettyp: NewTypeKind::Named(String::from("unit")),
                        codes: ins,
                    };
                    let parent = Some(global.clone());
                    let func_id = FuncManager::get_ins().register(main_func);
                    // 构造一个运行时
                    let mut runtime = Runtime::new(parent, func_id, vec![]);
                    match runtime.run() {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e),
                    }
                }
                Err(why) => Err(why),
            }
        }
        Err(why) => Err(format!("{:?}", why)),
    }
}
