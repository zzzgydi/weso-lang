use crate::base::opcode::Operand;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(String),
    Float(String),
    String(String),
    True,
    False,
    Null,
    Unit,
}

impl Literal {
    pub fn to_operand(&self) -> Operand {
        match &self {
            Literal::Integer(v) => Operand::Integer(v.clone()),
            Literal::Float(v) => Operand::Float(v.clone()),
            Literal::String(v) => Operand::String(v.clone()),
            Literal::True => Operand::True,
            Literal::False => Operand::False,
            Literal::Null => Operand::Null,
            Literal::Unit => Operand::Unit,
        }
    }

    // pub fn from_token(tok: LogosToken) -> Option<Literal> {
    //     match tok {
    //         LogosToken::True => Some(Literal::True),
    //         LogosToken::False => Some(Literal::False),
    //         LogosToken::Null => Some(Literal::Null),
    //         LogosToken::Integer(s) => Some(Literal::Integer(s.to_string())),
    //         LogosToken::Float(s) => Some(Literal::Float(s.to_string())),
    //         LogosToken::String(s) => Some(Literal::String(s.to_string())),
    //         _ => None,
    //     }
    // }
}
