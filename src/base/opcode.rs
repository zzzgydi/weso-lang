// 列举vm执行的操作符和操作数
use crate::base::ast::Located;
use crate::base::types::NewTypeKind;
// use crate::parser::token::TypeToken;
use std::fmt;

pub type Instruction = Located<InnerInstruction>;

#[derive(Debug, Clone, PartialEq)]
pub enum InnerInstruction {
    Assign {
        lhs: Operand,
        rhs: Operand,
    },

    Move {
        lhs: Operand,
        rhs: Operand,
    },

    Call {
        value: Operand,
        num: usize,
    },

    Dot {
        lhs: Operand,
        rhs: Operand,
    },

    Not {
        value: Operand,
    },

    Push {
        value: Operand,
    },

    If {
        value: Operand,
        addr: usize,
    },

    IfNot {
        value: Operand,
        addr: usize,
    },

    Goto {
        addr: usize,
    },

    Return {
        value: Operand,
    },

    DefVar {
        mutable: bool,
        name: Operand,
        typ: NewTypeKind,
    },

    DefFunc {
        name: String,
        sign: String,
        id: usize,
    },

    Destroy {
        value: String,
    },

    Struct {
        value: String,
        num: usize,
    },

    Repeat, // 将栈顶的值拷贝一遍再弹入栈

    Break,

    Continue,
    // End,
}

impl fmt::Display for InnerInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::InnerInstruction::*;
        match self {
            Assign { lhs, rhs } => write!(
                f,
                "{:<10} {:<10} {}",
                "assign",
                lhs.to_string(),
                rhs.to_string()
            ),
            Move { lhs, rhs } => write!(
                f,
                "{:<10} {:<10} {}",
                "move",
                lhs.to_string(),
                rhs.to_string()
            ),
            Call { value, num } => write!(f, "{:<10} {:<10} {}", "call", value.to_string(), num),
            Dot { lhs, rhs } => write!(
                f,
                "{:<10} {:<10} {}",
                "dot",
                lhs.to_string(),
                rhs.to_string()
            ),
            If { value, addr } => write!(f, "{:<10} {:<10} #{}", "if", value.to_string(), addr),
            IfNot { value, addr } => {
                write!(f, "{:<10} {:<10} #{}", "ifnot", value.to_string(), addr)
            }
            Not { value } => write!(f, "{:<10} {}", "not", value.to_string()),
            Push { value } => write!(f, "{:<10} {}", "push", value.to_string()),
            Goto { addr } => write!(f, "{:<10} #{}", "goto", addr),
            Return { value } => write!(f, "{:<10} {}", "return", value.to_string()),
            DefVar {
                mutable,
                name,
                typ: _,
            } => {
                let prefix = if *mutable { "let" } else { "const" };
                write!(f, "{:<10} {:<10}", prefix, name.to_string())
            }
            DefFunc { name, sign: _, id } => {
                write!(f, "{:<10} {:<10} &{}", "def", name.to_string(), id)
            }
            Struct { value, num } => {
                write!(f, "{:<10} {:<10} {}", "struct", value.to_string(), num)
            }
            Destroy { value } => write!(f, "{:<10} {}", "destroy", value.to_string()),
            // End => write!(f, "end"),
            Break => write!(f, "break"),
            Repeat => write!(f, "repeat"),
            Continue => write!(f, "continue"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    True,            // 字面量
    False,           // 字面量
    Unit,            // 字面量
    Null,            // 字面量
    Integer(String), // 字面量
    Float(String),   // 字面量
    String(String),  // 字面量
    Var(String),     // 变量名
    Stack,           // 从栈取值
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::True => f.write_str("true"),
            Operand::False => f.write_str("false"),
            Operand::Null => f.write_str("null"),
            Operand::Unit => f.write_str("unit"),
            Operand::Stack => f.write_str("$0"),
            Operand::Integer(s) => write!(f, "{}", s),
            Operand::Float(s) => write!(f, "{}", s),
            Operand::String(s) => write!(f, "{}", s),
            Operand::Var(s) => write!(f, "{}", s),
        }
    }
}

impl Operand {
    pub fn is_stack(&self) -> bool {
        match *self {
            Operand::Stack => true,
            _ => false,
        }
    }

    pub fn is_variable(&self) -> bool {
        match *self {
            Operand::Var(_) => true,
            _ => false,
        }
    }

    pub fn can_unwrap(&self) -> bool {
        match self {
            Operand::Integer(_) | Operand::Float(_) | Operand::Var(_) | Operand::String(_) => true,
            _ => false,
        }
    }

    pub fn unwrap(&self) -> &String {
        match self {
            Operand::Integer(s) | Operand::Float(s) | Operand::Var(s) | Operand::String(s) => s,
            _ => panic!("Invalid unwrap during Operand."),
        }
    }
}
