use crate::base::object;
use crate::base::object::WesoObject;
use crate::base::types::NewTypeKind;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Scope {
    // 标记父作用域
    parent: Option<Rc<RefCell<Scope>>>,

    // 符号表 保存变量名
    symbol: HashMap<String, WesoObject>,

    // 记录作用域内所有定义的函数 (name,sign)->id
    funcs: HashMap<(String, String), usize>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Scope>>>) -> Self {
        Scope {
            parent: parent.clone(),
            funcs: HashMap::new(),
            symbol: HashMap::new(),
        }
    }

    // 减少构造次数
    fn _get_func(&self, key: &(String, String)) -> Option<usize> {
        if let Some(id) = self.funcs.get(key) {
            Some(*id)
        } else {
            match self.parent.clone() {
                None => None,
                Some(p) => p.borrow()._get_func(key),
            }
        }
    }

    // 获取函数 - 根据'函数签名'获取函数
    pub fn get_func(&self, name: &String, sign: &String) -> Option<usize> {
        let key = &(name.to_string(), sign.to_string());
        self._get_func(key)
    }

    // 在作用域中定义函数
    pub fn define_func(&mut self, name: String, sign: String, id: usize) {
        self.funcs.insert((name, sign), id);
    }

    // 定义变量
    pub fn define_variable(&mut self, name: &String, mutable: bool, typ: &NewTypeKind) {
        self.symbol
            .insert(name.clone(), object::create_object(mutable, typ));
    }

    // 变量赋值
    pub fn set_variable(&mut self, name: &String, value: WesoObject) -> bool {
        if let Some(_) = self.symbol.get(name) {
            self.symbol.insert(name.clone(), value);
            true
        } else {
            match self.parent.clone() {
                None => false,
                Some(p) => p.borrow_mut().set_variable(name, value),
            }
        }
    }

    // 根据名称获取作用域中的对象
    pub fn get_object(&self, name: &String) -> Result<WesoObject, String> {
        match self.symbol.get(name) {
            Some(obj) => Ok(obj.clone()),
            None => match self.parent.clone() {
                Some(parent) => parent.borrow().get_object(name),
                None => Err(format!("Variable Error: {} is not defined.", name)),
            },
        }
    }
}
