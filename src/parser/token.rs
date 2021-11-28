use logos::Logos;
// use std::fmt;

#[derive(Debug, Logos, PartialEq, Copy, Clone)]
pub enum LogosToken<'source> {
    #[error]
    Error,

    // 关键词
    #[token("fn")]
    Func,
    #[token("type")]
    Type,
    #[token("struct")]
    Struct,
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("if")]
    If,
    #[token("elif")]
    Elif,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("return")]
    Return,
    // 内置类型 关键词
    // #[token("i8")]
    // Int8,
    // #[token("i16")]
    // Int16,
    // #[token("i32")]
    // Int32,
    // #[token("i64")]
    // Int64,
    // #[token("i128")]
    // Int128,
    // #[token("u8")]
    // Uint8,
    // #[token("u16")]
    // Uint16,
    // #[token("u32")]
    // Uint32,
    // #[token("u64")]
    // Uint64,
    // #[token("u128")]
    // Uint128,
    // #[token("f32")]
    // Float32,
    // #[token("f64")]
    // Float64,
    // #[token("bool")]
    // Bool,
    // #[token("str")]
    // Str,
    // #[token("any")]
    // Any,
    #[token("null")]
    Null,
    #[token("unit")]
    Unit,
    #[token("true")]
    True,
    #[token("false")]
    False,
    // 操作符
    #[token(".")]
    Dot,
    #[token("=")]
    Assign,
    #[token(":=")]
    Move,
    #[token("@")]
    At,
    #[token("||")]
    Or,
    #[token("&&")]
    And,
    #[token("!")]
    Not,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("%")]
    Mod,
    #[token("**")]
    Pow,
    #[token(">")]
    Gt,
    #[token("<")]
    Lt,
    #[token(">=")]
    Geq,
    #[token("<=")]
    Leq,
    #[token("!=")]
    Noteq,
    #[token("==")]
    Equal,
    #[token("|")]
    BitOr,
    #[token("&")]
    BitAnd,
    #[token("~")]
    BitNot,
    #[token("^")]
    BitXor,
    #[token(">>")]
    Rshift,
    #[token("<<")]
    Lshift,
    // 连等号
    #[token("+=")]
    AddAssign,
    #[token("-=")]
    SubAssign,
    #[token("*=")]
    MulAssign,
    #[token("/=")]
    DivAssign,
    #[token("%=")]
    ModAssign,
    #[token("|=")]
    BitOrAssign,
    #[token("&=")]
    BitAndAssign,
    #[token("^=")]
    BitXorAssign,
    #[token("<<=")]
    LshiftAssign,
    #[token(">>=")]
    RshiftAssign,
    #[token("**=")]
    PowAssign,

    // 符号
    #[regex("(\n)|(\r\n)|(\r)")]
    Newline,
    #[token(";")]
    Semi,
    #[token(":")]
    Colon,
    #[token("->")]
    Arrow,
    #[token(",")]
    Comma,
    #[token("?")]
    Question,
    #[token("(")]
    Lpar,
    #[token(")")]
    Rpar,
    #[token("{")]
    Lbrace,
    #[token("}")]
    Rbrace,
    #[token("[")]
    Lsqb,
    #[token("]")] //"
    Rsqb,
    // 字面量
    #[regex(r"[0-9]+")] // "
    Integer(&'source str),
    #[regex("[0-9]+\\.[0-9]+")] // "
    Float(&'source str),
    #[regex("(\"[^\"]*\")|('[^\']*')")] // "
    String(&'source str),
    // 变量
    #[regex("[_a-zA-Z][_a-zA-Z0-9]*")] //"
    VarName(&'source str),
    #[regex(r#"//[^\r\n]*"#)] //"
    Comment,
    #[regex(r#"[ \t\v]+"#)] //"
    Blank,
}

// 获取语法分析得到的类型参数
/*
#[derive(Debug, Clone, PartialEq)]
pub enum TypeToken {
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Float32,
    Float64,
    Bool,
    Str,
    Null,
    Any,
    Unit,
    Alias(String),
    Tuple(Vec<Box<TypeToken>>),
    Array(Box<TypeToken>),
}

impl fmt::Display for TypeToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeToken::Int8 => f.write_str("i8"),
            TypeToken::Int16 => f.write_str("i16"),
            TypeToken::Int32 => f.write_str("i32"),
            TypeToken::Int64 => f.write_str("i64"),
            TypeToken::Int128 => f.write_str("i128"),
            TypeToken::Uint8 => f.write_str("u8"),
            TypeToken::Uint16 => f.write_str("u16"),
            TypeToken::Uint32 => f.write_str("u32"),
            TypeToken::Uint64 => f.write_str("u64"),
            TypeToken::Uint128 => f.write_str("u128"),
            TypeToken::Float32 => f.write_str("f32"),
            TypeToken::Float64 => f.write_str("f64"),
            TypeToken::Bool => f.write_str("bool"),
            TypeToken::Str => f.write_str("str"),
            TypeToken::Null => f.write_str("null"),
            TypeToken::Unit => f.write_str("unit"),
            TypeToken::Any => f.write_str("any"),
            TypeToken::Alias(s) => write!(f, "alias({})", s),
            TypeToken::Array(s) => write!(f, "array({})", s.to_string()),
            TypeToken::Tuple(v) => write!(f, "tuple({:?})", v),
        }
    }
}

impl TypeToken {
    // 判断原类型是否兼容目标类型
    pub fn is_compatible(source: &Self, target: &Self) -> bool {
        if (source == target)
            || (Self::is_integer(source) && Self::is_integer(target))
            || (Self::is_float(source) && Self::is_float(target))
        {
            return true;
        }
        use self::TypeToken::*;
        match (source, target) {
            (Any, _) | (_, Null) | (Null, _) => true,
            _ => false,
        }
    }

    pub fn is_integer(value: &Self) -> bool {
        match value {
            TypeToken::Int8
            | TypeToken::Int16
            | TypeToken::Int32
            | TypeToken::Int64
            | TypeToken::Int128 => true,
            TypeToken::Uint8
            | TypeToken::Uint16
            | TypeToken::Uint32
            | TypeToken::Uint64
            | TypeToken::Uint128 => true,
            _ => false,
        }
    }

    pub fn is_float(value: &Self) -> bool {
        match value {
            TypeToken::Float32 | TypeToken::Float64 => true,
            _ => false,
        }
    }
}
*/
