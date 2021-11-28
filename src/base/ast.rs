// use crate::base::func::Function;
use crate::base::opcode::Operand;
// use crate::base::scope::Scope;
use crate::base::types::NewTypeKind;
use crate::parser::lexer;
use crate::parser::liter::Literal;
// use std::cell::RefCell;
use std::fmt;
// use std::rc::Rc;

/*
类型：
在解析ast树时，提取别名和struct，用map保存键值，其中别名需要检查。

函数：
每一个定义的函数都放到表中
*/

#[derive(Debug, PartialEq, Clone)]
pub struct Located<T> {
    pub location: lexer::Location,
    pub node: T,
}

impl<T> fmt::Display for Located<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.node, self.location)
    }
}

pub type Expression = Located<ExpressionKind>;
pub type Statement = Located<StatementKind>;

pub type StmtList = Vec<Statement>;

pub enum StatementKind {
    Break,

    Continue,

    Return {
        value: Box<Expression>,
    },

    // 赋值 a = b
    Assign {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    // 移动 a := b
    Move {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    // 赋值缩写 a += 1
    AugAssign {
        op: String,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    // 变量声明 let|const a = b
    VarDef {
        mutable: bool,
        name: String,
        typ: NewTypeKind,
        assign: Option<Box<Expression>>,
    },

    // 类型别名 type a = b
    TypeDef {
        left: String,
        right: NewTypeKind,
    },

    // 结构体定义 struct {}
    // StructDef {
    //     name: String,
    //     value: Vec<(String, TypeToken)>,
    // },

    // 函数定义 fn xx () -> type {}
    // FuncDef {
    //     name: String,
    //     args: Vec<(String, TypeToken)>,
    //     rettyp: TypeToken,
    //     block: StmtList,
    // },
    FuncDef {
        name: String,
        args: Vec<(String, NewTypeKind)>,
        rettyp: NewTypeKind,
        block: StmtList,
    },

    // single expression
    Expression {
        expr: Box<Expression>,
    },

    // if语句
    If {
        test: Box<Expression>,
        then: StmtList,
        orelse: StmtList,
    },

    // whlie语句
    While {
        test: Box<Expression>,
        then: StmtList,
    },

    // for语句
    For {
        name: String,
        iter: Box<Expression>,
        then: StmtList,
    },
}

impl Statement {
    // lalrpop中使用
    pub fn _set_elif(&mut self, stmts: StmtList) {
        match &mut self.node {
            StatementKind::If { orelse, .. } => {
                *orelse = stmts;
            }
            _ => (),
        }
    }
}

pub enum ExpressionKind {
    // 字面量
    Liter {
        value: Literal,
    },

    // 变量
    Ident {
        value: String,
    },

    // 属性访问 a.b 或者 a[c]
    Dot {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    // 函数调用 a()
    Call {
        callee: Box<Expression>,
        args: Vec<Box<Expression>>,
    },

    // 逻辑与 —— 支持短路运算
    And {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    // 逻辑或 —— 支持短路运算
    Or {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    // 逻辑非 !x
    Not {
        expr: Box<Expression>,
    },

    // 构造结构体字面量 T @ {a: 1, b: 2}
    Struct {
        name: String,
        args: Vec<(String, Box<Expression>)>,
    },

    // 三元运算 ?:
    Question {
        test: Box<Expression>,
        then: Box<Expression>,
        orelse: Box<Expression>,
    },
    // 类型转换 as // 应该属于一个表达式
    // TypeMark {},
}

impl Expression {
    pub fn is_simple(&self) -> bool {
        match &self.node {
            ExpressionKind::Liter { value: _ } => true,
            ExpressionKind::Ident { value: _ } => true,
            _ => false,
        }
    }

    pub fn not_simple(&self) -> bool {
        !self.is_simple()
    }

    pub fn to_operand(&self) -> Operand {
        match &self.node {
            ExpressionKind::Liter { value } => value.to_operand(),
            ExpressionKind::Ident { value } => Operand::Var(value.to_string()),
            _ => Operand::Stack,
        }
    }
}
