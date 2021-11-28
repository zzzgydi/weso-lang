use crate::base::func::FuncManager;
use crate::base::func::Function;
use crate::base::object;
use crate::base::object::WesoObject;
use crate::base::opcode::{InnerInstruction, Instruction, Operand};
use crate::base::scope::Scope;
// use crate::parser::token::TypeToken;
use crate::vm::global::WesoSTD;
use std::cell::Cell;
use std::cell::RefCell;
// use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum StackValue {
    // 一个值
    Object(WesoObject),
    // 一个指针，dot的时候使用
    Pointer(WesoObject, String),
}

pub struct Runtime {
    stack: RefCell<Vec<StackValue>>,
    func: Arc<Function>,
    pc: Cell<usize>,
    scope: Rc<RefCell<Scope>>,
}

impl Runtime {
    pub fn new(
        parent: Option<Rc<RefCell<Scope>>>,
        func_id: usize,
        params: Vec<WesoObject>,
    ) -> Self {
        let scope = Rc::new(RefCell::new(Scope::new(parent)));
        let func = FuncManager::get_ins().get_func(func_id).unwrap();
        let args = &func.args;

        // 将参数全部加入域中
        for i in 0..args.len() {
            let arg = &args[i];
            let param = (&params[i]).clone();
            scope.borrow_mut().define_variable(&arg.0, false, &arg.1);
            scope.borrow_mut().set_variable(&arg.0, param);
        }

        Runtime {
            stack: RefCell::new(Vec::new()),
            pc: Cell::new(0),
            func,
            scope,
        }
    }

    #[inline]
    fn goto(&self, pc: usize) {
        self.pc.set(pc - 1); // 每次循环都会执行一次next加一
    }

    #[inline]
    fn next(&self) {
        let idx = self.pc.get() + 1;
        self.pc.set(idx);
    }

    #[inline]
    fn fetch(&self) -> Option<&Instruction> {
        self.func.codes.get(self.pc.get())
    }

    // 对栈进行操作
    fn push_obj(&self, obj: WesoObject) {
        self.stack.borrow_mut().push(StackValue::Object(obj));
    }

    // 对栈进行操作
    fn pop(&self) -> Result<StackValue, String> {
        match self.stack.borrow_mut().pop() {
            Some(value) => Ok(value),
            None => Err(format!("Runtime Error: Stack damage.")),
        }
    }

    // 根据stackvalue获取对应的object的值
    pub fn get_stack_obj(&self, value: &StackValue) -> Result<WesoObject, String> {
        match value {
            StackValue::Object(obj) => Ok(obj.clone()),
            StackValue::Pointer(name, key) => match name.get_attr(key) {
                Some(o) => Ok(o),
                None => Err(format!("Attribute Error: Struct does not contain {}.", key)),
            },
        }
    }

    // 根据类型获取函数的签名
    fn func_sign(objs: &Vec<WesoObject>) -> String {
        let typ = objs
            .iter()
            .map(|obj| obj.get_typ().to_string())
            .collect::<Vec<String>>()
            .join(",");
        format!("({})", typ)
    }

    // 通过操作数获取具体的对象
    fn get_value(&self, op: &Operand) -> Result<WesoObject, String> {
        match op {
            Operand::Stack => match self.stack.borrow().last() {
                Some(obj) => self.get_stack_obj(obj),
                None => Err(format!("Stack Error: Invalid operation.")),
            },
            Operand::Var(name) => match self.scope.borrow().get_object(name) {
                Ok(obj) => Ok(obj.clone()),
                Err(e) => Err(e),
            },
            _ => match object::create_literal(op) {
                Some(obj) => Ok(obj),
                None => Err(format!("Runtime Error: Cannot create literal.")),
            },
        }
    }

    pub fn run(&mut self) -> Result<WesoObject, String> {
        while let Some(ins) = self.fetch() {
            match &ins.node {
                // 变量定义
                InnerInstruction::DefVar { mutable, name, typ } => {
                    self.scope
                        .borrow_mut()
                        .define_variable(&name.unwrap(), *mutable, typ);
                }

                // 定义函数
                InnerInstruction::DefFunc { name, id, sign } => {
                    self.scope
                        .borrow_mut()
                        .define_func(name.to_string(), sign.to_string(), *id);
                }

                // 对象赋值
                InnerInstruction::Assign { lhs, rhs } => {
                    let rhs_obj = match self.get_value(rhs) {
                        Ok(value) => value,
                        Err(e) => return Err(e),
                    };
                    if lhs.is_variable() {
                        self.scope.borrow_mut().set_variable(lhs.unwrap(), rhs_obj);
                    } else if lhs.is_stack() {
                        // 处理dot的操作
                        let stack_value = match self.pop() {
                            Ok(t) => t,
                            Err(e) => return Err(e),
                        };
                        match &stack_value {
                            StackValue::Object(_) => {
                                return Err(format!(
                                    "Runtime Error: left-hand value could not be modified."
                                ))
                            }
                            StackValue::Pointer(obj, key) => {
                                if obj.is_struct() {
                                    if obj.has_attr(key) {
                                        obj.set_attr(key, rhs_obj);
                                    } else {
                                        return Err(format!(
                                            "Attribute Error: Struct does not contain {}.",
                                            key
                                        ));
                                    }
                                } else {
                                    return Err(format!("Runtime Error: Operand is not a struct."));
                                }
                            }
                        };
                    } else {
                        // 左值不可变的错误
                        return Err(format!(
                            "Runtime Error: left-hand value could not be modified."
                        ));
                    }
                }

                // 函数调用
                InnerInstruction::Call { value, num } => {
                    // 检查操作数是不是变量, 或者在栈上
                    match value {
                        Operand::Stack | Operand::Var(_) => (),
                        _ => return Err(format!("Runtime Error: literal is not callable.")),
                    };
                    // 逆序获取所有参数
                    let mut args = vec![];
                    for _ in 0..*num {
                        let arg = match self.pop() {
                            Ok(v) => match self.get_stack_obj(&v) {
                                Ok(o) => o,
                                Err(e) => return Err(e),
                            },
                            Err(e) => return Err(e),
                        };
                        args.push(arg.clone());
                    }
                    args.reverse();

                    // 创建一个running time
                    let func_name = value.unwrap(); // 获取函数名
                    let func_sign = Self::func_sign(&args);
                    // 优先在作用域内查找函数
                    if let Some(func_id) = self.scope.borrow().get_func(func_name, &func_sign) {
                        // let func = FuncManager::get_ins().get_func(func_id).unwrap();
                        let parent = Some(self.scope.clone());
                        let mut runtime = Runtime::new(parent, func_id, args);
                        match runtime.run() {
                            Ok(res) => self.push_obj(res),
                            Err(e) => return Err(e),
                        }
                    } else {
                        // 在内建函数库中寻找
                        match WesoSTD::get_ins().get_func(func_name) {
                            Some(func) => match func(&args) {
                                Ok(o) => self.push_obj(o),
                                Err(e) => return Err(e),
                            },
                            None => {
                                return Err(format!(
                                    "Variable Error: {} is not defined.",
                                    func_name
                                ))
                            }
                        };
                    }
                }

                // 点操作
                InnerInstruction::Dot { lhs, rhs } => {
                    // 左值是name，右值是key左值
                    // 必须是struct结构体，右值必须是能转换成str类型的
                    // 左值可能是在栈中，左值可能是变量名
                    let left = match self.get_value(lhs) {
                        Ok(obj) => obj,
                        Err(e) => return Err(e),
                    };
                    if !left.is_struct() {
                        return Err(format!("Runtime Error: Operand is not a struct."));
                    }
                    let mut key: String = String::new();
                    if rhs.can_unwrap() {
                        key = rhs.unwrap().clone();
                    } else if rhs.is_stack() {
                        match self.pop() {
                            Ok(t) => match self.get_stack_obj(&t) {
                                Err(e) => return Err(e),
                                Ok(obj) => {
                                    if obj.is_float() || obj.is_integer() || obj.is_string() {
                                        key = obj.to_string();
                                    }
                                }
                            },
                            Err(e) => return Err(e),
                        };
                    } else {
                        return Err(format!("Attribute Error: Invalid attribute."));
                    }
                    // 将一个这样的指针放入栈中
                    self.stack.borrow_mut().push(StackValue::Pointer(left, key));
                }

                // InnerInstruction::Move { lhs, rhs } => {}

                // 将操作数做取反，再放入栈中
                InnerInstruction::Not { value } => {
                    let obj = match self.get_value(value) {
                        Ok(o) => o,
                        Err(e) => return Err(e),
                    };
                    if obj.is_bool() {
                        if Arc::ptr_eq(&obj, &object::OBJ_TRUE.clone()) {
                            self.push_obj(object::OBJ_FALSE.clone());
                        } else {
                            self.push_obj(object::OBJ_TRUE.clone());
                        }
                    } else {
                        return Err(format!("Type Error: Expression should be a boolean."));
                    }
                }

                InnerInstruction::Push { value } => {
                    let obj = match self.get_value(value) {
                        Ok(o) => o,
                        Err(e) => return Err(e),
                    };
                    self.push_obj(obj);
                }

                InnerInstruction::If { value, addr } => {
                    let test = match self.get_value(value) {
                        Ok(o) => o,
                        Err(e) => return Err(e),
                    };
                    if test.is_bool() {
                        if Arc::ptr_eq(&test, &object::OBJ_FALSE.clone()) {
                            self.goto(*addr);
                        }
                    } else {
                        return Err(format!("Type Error: Expression should be a boolean."));
                    }
                }

                InnerInstruction::IfNot { value, addr } => {
                    let test = match self.get_value(value) {
                        Ok(o) => o,
                        Err(e) => return Err(e),
                    };
                    if test.is_bool() {
                        if Arc::ptr_eq(&test, &object::OBJ_TRUE.clone()) {
                            self.goto(*addr);
                        }
                    } else {
                        return Err(format!("Type Error: Expression should be a boolean."));
                    }
                }

                InnerInstruction::Goto { addr } => {
                    self.goto(*addr);
                }

                InnerInstruction::Return { value } => {
                    let obj = match self.get_value(value) {
                        Ok(o) => o,
                        Err(e) => return Err(e),
                    };
                    return Ok(obj);
                }

                InnerInstruction::Repeat => {
                    match self.stack.borrow().last() {
                        Some(value) => self.stack.borrow_mut().push(value.clone()),
                        None => return Err(String::from("Runtime Error: Stack damage.")),
                    };
                }

                // 创建结构体字面量
                InnerInstruction::Struct { value: _, num: _ } => {}

                // 销毁变量
                InnerInstruction::Destroy { value: _ } => {}

                // Instruction::Break => {}
                // Instruction::Continue {} => {}
                _ => return Err(format!("Runtime Error: Unhandled instruction.")),
            };
            self.next();
        }
        Ok(object::OBJ_UNIT.clone())
    }
}
