use crate::base::opcode::Instruction;
// use crate::base::scope::Scope;
use crate::base::types::NewTypeKind;
use std::cell::RefCell;
use std::sync::Arc;

// 用函数管理器
pub struct FuncManager {
    funcs: RefCell<Vec<Arc<Function>>>,
}

impl FuncManager {
    pub fn get_ins() -> &'static FuncManager {
        static mut _FM: Option<Arc<FuncManager>> = None;

        unsafe {
            _FM.get_or_insert_with(|| {
                Arc::new(FuncManager {
                    funcs: RefCell::new(Vec::new()),
                })
            });
            _FM.as_ref().unwrap()
        }
    }

    pub fn register(&self, func: Function) -> usize {
        self.funcs.borrow_mut().push(Arc::new(func));
        self.funcs.borrow().len() - 1
    }

    pub fn get_func(&self, id: usize) -> Option<Arc<Function>> {
        match self.funcs.borrow().get(id) {
            Some(f) => Some(f.clone()),
            None => None,
        }
    }

    pub fn get_sign(args: &Vec<(String, NewTypeKind)>) -> String {
        format!(
            "({})",
            args.iter()
                .map(|item| item.1.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    // pub name: String,
    pub args: Vec<(String, NewTypeKind)>,
    pub rettyp: NewTypeKind,
    pub codes: Vec<Instruction>,
}
