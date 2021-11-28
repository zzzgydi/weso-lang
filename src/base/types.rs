// use crate::parser::token::TypeToken;
// use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
// use std::sync::Arc;

// #[derive(Debug, Clone)]
// pub struct StructManager {
//     v: RefCell<Vec<(String, Vec<(String, TypeToken)>)>>,
// }

// impl StructManager {
//     pub fn get_ins() -> &'static StructManager {
//         static mut _SM: Option<Arc<StructManager>> = None;

//         unsafe {
//             _SM.get_or_insert_with(|| {
//                 Arc::new(StructManager {
//                     v: RefCell::new(Vec::new()),
//                 })
//             });
//             _SM.as_ref().unwrap()
//         }
//     }

//     pub fn register(&self, name: String, fields: Vec<(String, TypeToken)>) {
//         self.v.borrow_mut().push((name, fields));
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum NewTypeKind<'input> {
//     Named(&'input str),

//     Array(Box<NewTypeKind<'input>>),

//     Tuple(Vec<NewTypeKind<'input>>),

//     Struct(HashMap<String, NewTypeKind<'input>>),

//     Function(Vec<NewTypeKind<'input>>, Box<NewTypeKind<'input>>),
// }

#[derive(Debug, Clone, PartialEq)]
pub enum NewTypeKind {
    Named(String),

    Array(Box<NewTypeKind>),

    Tuple(Vec<NewTypeKind>),

    Struct(HashMap<String, NewTypeKind>),

    // 参数和返回值的类型
    Function(Vec<NewTypeKind>, Box<NewTypeKind>),
}

impl fmt::Display for NewTypeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NewTypeKind::Named(value) => f.write_str(value.as_str()),

            NewTypeKind::Array(value) => write!(f, "[{}]", value.to_string()),

            NewTypeKind::Tuple(value) => write!(
                f,
                "({})",
                value
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),

            NewTypeKind::Struct(_value) => f.write_str("struct"),

            NewTypeKind::Function(_arr, _typ) => f.write_str("function"),
        }
    }
}

impl NewTypeKind {
    pub fn name(value: &str) -> NewTypeKind {
        NewTypeKind::Named(value.to_string())
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum TypeKind {
//     Token(TypeToken),

//     Struct(Vec<(String, TypeToken)>),
// }

// #[derive(Debug)]
// pub struct TypeTable {
//     types: Vec<TypeKind>,

//     // 根据名称记录到types具体值的映射
//     // 包括记录别名，结构体
//     // type xx = xx; struct name {}
//     hashs: HashMap<String, usize>,
// }

// impl TypeTable {
//     pub fn new() -> TypeTable {
//         TypeTable {
//             types: vec![],
//             hashs: HashMap::new(),
//         }
//     }

//     // 根据TypeToken获得一个usize数据
//     pub fn insert(&mut self, typ: &TypeToken) -> Option<usize> {
//         match typ {
//             TypeToken::Alias(s) => match self.hashs.get(s) {
//                 // 可能找不到值
//                 Some(a) => Some(*a),
//                 None => None,
//             },
//             TypeToken::Tuple(_) | TypeToken::Array(_) => {
//                 self.types.push(TypeKind::Token(typ.clone()));
//                 Some(self.types.len() - 1)
//             }
//             _ => match self.hashs.get(&typ.to_string()) {
//                 Some(num) => Some(*num),
//                 None => {
//                     self.types.push(TypeKind::Token(typ.clone()));
//                     self.hashs.insert(typ.to_string(), self.types.len() - 1);
//                     Some(self.types.len() - 1)
//                 }
//             },
//         }
//     }

//     // type a = b; 其中先执行insert(b)再insert_alias
//     pub fn insert_alias(&mut self, key: &String, value: usize) -> Option<usize> {
//         if value >= self.types.len() {
//             return None;
//         }
//         self.hashs.insert(key.clone(), value);
//         Some(value)
//     }

//     // 当遇到struct定义的时候
//     pub fn insert_struct(&mut self, name: &String, fields: Vec<(String, TypeToken)>) {
//         self.types.push(TypeKind::Struct(fields));
//         self.hashs.insert(name.clone(), self.types.len() - 1);
//     }

//     pub fn insert_kind(&mut self, item: &TypeKind) -> usize {
//         self.types.push(item.clone());
//         self.types.len() - 1
//     }

//     // 通过key获取具体的类型值
//     pub fn get_item_by_key(&self, key: &String) -> Option<&TypeKind> {
//         println!("get_item_by_key {}", key);
//         if let Some(id) = self.hashs.get(key) {
//             println!("get_item_by_key {}", id);
//             self.get_item_by_id(*id)
//         } else {
//             println!("get_item_by_key {:?}", self.hashs);
//             None
//         }
//     }

//     // 通过id获取具体的类型值
//     pub fn get_item_by_id(&self, id: usize) -> Option<&TypeKind> {
//         println!("get_item_by_id {}", id);
//         if id >= self.types.len() {
//             None
//         } else {
//             Some(&self.types[id])
//         }
//     }
// }
