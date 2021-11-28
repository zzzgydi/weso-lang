use crate::base::opcode::Operand;
use crate::base::types::NewTypeKind;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

pub type WesoObject = Arc<InnerObject>;

lazy_static! {
    pub static ref OBJ_TRUE: WesoObject = Arc::new(InnerObject {
        mutable: false,
        value: ObjectValue::Boolean { value: true },
        typ: NewTypeKind::name("bool"),
    });
    pub static ref OBJ_FALSE: WesoObject = Arc::new(InnerObject {
        mutable: false,
        value: ObjectValue::Boolean { value: false },
        typ: NewTypeKind::name("bool"),
    });
    pub static ref OBJ_NULL: WesoObject = Arc::new(InnerObject {
        mutable: false,
        value: ObjectValue::Null,
        typ: NewTypeKind::name("any"),
    });
    pub static ref OBJ_UNIT: WesoObject = Arc::new(InnerObject {
        mutable: false,
        value: ObjectValue::Unit,
        typ: NewTypeKind::name("unit"),
    });
}

#[derive(Debug, Clone)]
pub struct InnerObject {
    mutable: bool,      // 变量是否可变
    value: ObjectValue, // 变量实际值
    typ: NewTypeKind,   // 变量绑定的类型
}

impl InnerObject {
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }

    pub fn get_value(&self) -> &ObjectValue {
        &self.value
    }

    pub fn get_typ(&self) -> &NewTypeKind {
        &self.typ
    }

    // 判断该对象是否是结构体，或者说是不是有属性
    pub fn is_struct(&self) -> bool {
        match self.value {
            ObjectValue::Struct { value: _ } => true,
            _ => false,
        }
    }

    // 如果是结构体，则根据传入参数获取对应的对象
    pub fn get_attr(&self, key: &String) -> Option<WesoObject> {
        match &self.value {
            ObjectValue::Struct { value } => match value.borrow().get(key) {
                Some(obj) => Some(obj.clone()),
                None => None,
            },
            _ => None,
        }
    }

    // 判断是否是结构体，是否有属性key
    pub fn has_attr(&self, key: &String) -> bool {
        match &self.value {
            ObjectValue::Struct { value } => match value.borrow().get(key) {
                Some(_) => true,
                None => false,
            },
            _ => false,
        }
    }

    // 直接设置属性值，前提是结构体
    pub fn set_attr(&self, key: &String, value: WesoObject) {
        match &self.value {
            ObjectValue::Struct { value: hash } => {
                hash.borrow_mut().insert(key.clone(), value.clone());
            }
            _ => (),
        }
    }

    //
    pub fn is_integer(&self) -> bool {
        match &self.value {
            ObjectValue::Integer { value: _ } => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match &self.value {
            ObjectValue::Float { value: _ } => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match &self.value {
            ObjectValue::Boolean { value: _ } => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match &self.value {
            ObjectValue::String { value: _ } => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> String {
        match &self.value {
            ObjectValue::Null => String::from("null"),
            ObjectValue::Unit => String::from("unit"),
            ObjectValue::Integer { value } => format!("{}", value),
            ObjectValue::Float { value } => format!("{}", value),
            ObjectValue::String { value } => value.clone(),
            ObjectValue::Boolean { value } => format!("{}", value),
            ObjectValue::Array { value: _ } => format!("array[]"),
            ObjectValue::Tuple { value: _ } => format!("tuple()"),
            ObjectValue::Struct { value: _ } => format!("struct{{}}"),
        }
    }
}

pub fn create_literal(op: &Operand) -> Option<WesoObject> {
    match op {
        Operand::True => Some(OBJ_TRUE.clone()),
        Operand::False => Some(OBJ_FALSE.clone()),
        Operand::Null => Some(OBJ_NULL.clone()),
        Operand::Unit => Some(OBJ_UNIT.clone()),
        Operand::Integer(s) => {
            let tmp = Arc::new(InnerObject {
                mutable: false,
                value: ObjectValue::Integer {
                    value: i32::from_str(&*s).ok().unwrap_or(0_i32),
                },
                typ: NewTypeKind::Named("i32".to_string()),
            });
            Some(tmp)
        }
        Operand::Float(s) => {
            let tmp = Arc::new(InnerObject {
                mutable: false,
                value: ObjectValue::Float {
                    value: f64::from_str(&*s).ok().unwrap_or(0_f64),
                },
                typ: NewTypeKind::Named("f64".to_string()),
            });
            Some(tmp)
        }
        Operand::String(s) => {
            let tmp = Arc::new(InnerObject {
                mutable: false,
                value: ObjectValue::String {
                    value: String::from(&s[1..s.len() - 1]),
                },
                typ: NewTypeKind::Named("str".to_string()),
            });
            Some(tmp)
        }
        _ => None,
    }
}

// 构造初始对象，赋值为空
pub fn create_object(mutable: bool, typ: &NewTypeKind) -> WesoObject {
    Arc::new(InnerObject {
        mutable,
        typ: typ.clone(),
        value: ObjectValue::Null,
    })
}

pub fn create_integer(typ: &NewTypeKind, value: i32) -> WesoObject {
    Arc::new(InnerObject {
        mutable: false,
        typ: typ.clone(),
        value: ObjectValue::Integer { value },
    })
}

pub fn create_float(typ: &NewTypeKind, value: f64) -> WesoObject {
    Arc::new(InnerObject {
        mutable: false,
        typ: typ.clone(),
        value: ObjectValue::Float { value },
    })
}

pub fn create_string(value: String) -> WesoObject {
    Arc::new(InnerObject {
        mutable: false,
        typ: NewTypeKind::Named("str".to_string()),
        value: ObjectValue::String { value },
    })
}

#[derive(Debug, Clone)]
pub enum ObjectValue {
    Null,

    Unit,

    Integer {
        value: i32,
    },

    Float {
        value: f64,
    },

    String {
        value: String,
    },

    Boolean {
        value: bool,
    },

    Array {
        value: RefCell<Vec<WesoObject>>,
    },

    Tuple {
        value: RefCell<Vec<WesoObject>>,
    },

    Struct {
        value: RefCell<HashMap<String, WesoObject>>,
    },
}

impl PartialEq for ObjectValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ObjectValue::Null, ObjectValue::Null) => true,
            (ObjectValue::Unit, ObjectValue::Unit) => true,
            (ObjectValue::Integer { value: v1 }, ObjectValue::Integer { value: v2 }) => v1 == v2,
            (ObjectValue::Float { value: v1 }, ObjectValue::Float { value: v2 }) => v1 == v2,
            (ObjectValue::String { value: v1 }, ObjectValue::String { value: v2 }) => v1 == v2,
            (ObjectValue::Boolean { value: v1 }, ObjectValue::Boolean { value: v2 }) => v1 == v2,
            (_, _) => false,
        }
    }
}

impl fmt::Display for ObjectValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjectValue::Null => f.write_str("null"),
            ObjectValue::Unit => f.write_str("unit"),
            ObjectValue::Integer { value } => write!(f, "{}", value),
            ObjectValue::Float { value } => write!(f, "{}", value),
            ObjectValue::String { value } => write!(f, "{}", value),
            ObjectValue::Boolean { value } => write!(f, "{}", value),
            ObjectValue::Array { value } => write!(
                f,
                "[{}]",
                value
                    .borrow()
                    .iter()
                    .map(|o| o.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            ObjectValue::Tuple { value } => write!(
                f,
                "({})",
                value
                    .borrow()
                    .iter()
                    .map(|o| o.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            ObjectValue::Struct { value } => write!(
                f,
                "[{}]",
                value
                    .borrow()
                    .iter()
                    .map(|(k, v)| format!("{}:{}", k, v.to_string()))
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}

unsafe impl Sync for InnerObject {}
unsafe impl Send for InnerObject {}
unsafe impl Sync for ObjectValue {}
unsafe impl Send for ObjectValue {}
