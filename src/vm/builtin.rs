use crate::base::object;
use crate::base::object::{ObjectValue, WesoObject};
use std::sync::Arc;
use wasm_bindgen::prelude::*;

macro_rules! weso_obj_not {
    ($ex:expr) => {
        match $ex {
            Ok(v) => {
                if Arc::ptr_eq(&v, &object::OBJ_TRUE) {
                    Ok(object::OBJ_FALSE.clone())
                } else {
                    Ok(object::OBJ_TRUE.clone())
                }
            }
            Err(e) => Err(e),
        }
    };
}

// 基础功能支持
// ==
pub fn std_equal(args: &Vec<WesoObject>) -> Result<WesoObject, String> {
    if args.len() != 2 {
        return Err(format!(
            "Runtime Error: Expected 2 arguments, get {}",
            args.len()
        ));
    } else {
        let lhs = &args[0];
        let rhs = &args[1];
        if lhs.get_typ() == rhs.get_typ() {
            if lhs.get_value() == rhs.get_value() {
                return Ok(object::OBJ_TRUE.clone());
            }
        }
        return Ok(object::OBJ_FALSE.clone());
    }
}

// !=
pub fn std_noequal(args: &Vec<WesoObject>) -> Result<WesoObject, String> {
    weso_obj_not!(std_equal(args))
}

// <
pub fn std_lt(args: &Vec<WesoObject>) -> Result<WesoObject, String> {
    if args.len() != 2 {
        return Err(format!(
            "Runtime Error: Expected 2 arguments, get {}",
            args.len()
        ));
    }
    let lhs = &args[0];
    let rhs = &args[1];
    match (lhs.get_value(), rhs.get_value()) {
        (ObjectValue::Integer { value: v1 }, ObjectValue::Integer { value: v2 }) => {
            Ok(if v1 < v2 {
                object::OBJ_TRUE.clone()
            } else {
                object::OBJ_FALSE.clone()
            })
        }
        (ObjectValue::Float { value: v1 }, ObjectValue::Float { value: v2 }) => Ok(if v1 < v2 {
            object::OBJ_TRUE.clone()
        } else {
            object::OBJ_FALSE.clone()
        }),
        (ObjectValue::Boolean { value: v1 }, ObjectValue::Boolean { value: v2 }) => {
            Ok(if v1 < v2 {
                object::OBJ_TRUE.clone()
            } else {
                object::OBJ_FALSE.clone()
            })
        }
        (ObjectValue::Integer { value: v1 }, ObjectValue::Float { value: v2 }) => {
            Ok(if f64::from(*v1) < *v2 {
                object::OBJ_TRUE.clone()
            } else {
                object::OBJ_FALSE.clone()
            })
        }
        (ObjectValue::Float { value: v1 }, ObjectValue::Integer { value: v2 }) => {
            Ok(if *v1 < f64::from(*v2) {
                object::OBJ_TRUE.clone()
            } else {
                object::OBJ_FALSE.clone()
            })
        }
        (_, _) => Err(format!(
            "Runtime Error: function lt does not accept type {} and {}.",
            lhs.get_typ(),
            rhs.get_typ()
        )),
    }
}

// >
pub fn std_gt(args: &Vec<WesoObject>) -> Result<WesoObject, String> {
    if args.len() != 2 {
        return Err(format!(
            "Runtime Error: Expected 2 arguments, get {}",
            args.len()
        ));
    }
    let lhs = &args[0];
    let rhs = &args[1];
    match (lhs.get_value(), rhs.get_value()) {
        (ObjectValue::Integer { value: v1 }, ObjectValue::Integer { value: v2 }) => {
            Ok(if v1 > v2 {
                object::OBJ_TRUE.clone()
            } else {
                object::OBJ_FALSE.clone()
            })
        }
        (ObjectValue::Float { value: v1 }, ObjectValue::Float { value: v2 }) => Ok(if v1 > v2 {
            object::OBJ_TRUE.clone()
        } else {
            object::OBJ_FALSE.clone()
        }),
        (ObjectValue::Boolean { value: v1 }, ObjectValue::Boolean { value: v2 }) => {
            Ok(if v1 > v2 {
                object::OBJ_TRUE.clone()
            } else {
                object::OBJ_FALSE.clone()
            })
        }
        (ObjectValue::Integer { value: v1 }, ObjectValue::Float { value: v2 }) => {
            Ok(if f64::from(*v1) > *v2 {
                object::OBJ_TRUE.clone()
            } else {
                object::OBJ_FALSE.clone()
            })
        }
        (ObjectValue::Float { value: v1 }, ObjectValue::Integer { value: v2 }) => {
            Ok(if *v1 > f64::from(*v2) {
                object::OBJ_TRUE.clone()
            } else {
                object::OBJ_FALSE.clone()
            })
        }
        (_, _) => Err(format!(
            "Runtime Error: function gt does not accept type {} and {}.",
            lhs.get_typ(),
            rhs.get_typ()
        )),
    }
}

// <=
pub fn std_leq(args: &Vec<WesoObject>) -> Result<WesoObject, String> {
    weso_obj_not!(std_gt(args))
}

// >=
pub fn std_geq(args: &Vec<WesoObject>) -> Result<WesoObject, String> {
    weso_obj_not!(std_lt(args))
}

// +
pub fn std_add(args: &Vec<WesoObject>) -> Result<WesoObject, String> {
    if args.len() != 2 {
        return Err(format!(
            "Runtime Error: Expected 2 arguments, get {}",
            args.len()
        ));
    } else {
        let lhs = &args[0];
        let rhs = &args[1];
        match (lhs.get_value(), rhs.get_value()) {
            (ObjectValue::Integer { value: v1 }, ObjectValue::Integer { value: v2 }) => {
                Ok(object::create_integer(lhs.get_typ(), v1 + v2))
            }
            (ObjectValue::Integer { value: v1 }, ObjectValue::Float { value: v2 }) => {
                Ok(object::create_float(lhs.get_typ(), f64::from(*v1) + v2))
            }
            (ObjectValue::Float { value: v1 }, ObjectValue::Integer { value: v2 }) => {
                Ok(object::create_float(lhs.get_typ(), v1 + f64::from(*v2)))
            }
            (ObjectValue::Float { value: v1 }, ObjectValue::Float { value: v2 }) => {
                Ok(object::create_float(lhs.get_typ(), v1 + v2))
            }
            // 字符串
            (ObjectValue::String { value: v1 }, ObjectValue::Unit) => {
                Ok(object::create_string(format!("{}{}", v1, rhs.to_string())))
            }
            (ObjectValue::String { value: v1 }, ObjectValue::Null) => {
                Ok(object::create_string(format!("{}{}", v1, rhs.to_string())))
            }
            (ObjectValue::String { value: v1 }, ObjectValue::Boolean { value }) => {
                Ok(object::create_string(format!("{}{}", v1, value)))
            }
            (ObjectValue::String { value: v1 }, ObjectValue::Integer { value: v2 }) => {
                Ok(object::create_string(format!("{}{}", v1, v2)))
            }
            (ObjectValue::String { value: v1 }, ObjectValue::Float { value: v2 }) => {
                Ok(object::create_string(format!("{}{}", v1, v2)))
            }
            (ObjectValue::String { value: v1 }, ObjectValue::String { value: v2 }) => {
                Ok(object::create_string(format!("{}{}", v1, v2)))
            }
            (ObjectValue::Unit, ObjectValue::String { value: v1 }) => {
                Ok(object::create_string(format!("{}{}", lhs.to_string(), v1)))
            }
            (ObjectValue::Null, ObjectValue::String { value: v1 }) => {
                Ok(object::create_string(format!("{}{}", lhs.to_string(), v1)))
            }
            (ObjectValue::Integer { value: v1 }, ObjectValue::String { value: v2 }) => {
                Ok(object::create_string(format!("{}{}", v1, v2)))
            }
            (ObjectValue::Float { value: v1 }, ObjectValue::String { value: v2 }) => {
                Ok(object::create_string(format!("{}{}", v1, v2)))
            }
            (ObjectValue::Boolean { value: v1 }, ObjectValue::String { value }) => {
                Ok(object::create_string(format!("{}{}", v1, value)))
            }
            (_, _) => Err(format!(
                "Runtime Error: function add does not accept type {} and {}.",
                lhs.get_typ(),
                rhs.get_typ()
            )),
        }
    }
}

// -
pub fn std_sub(args: &Vec<WesoObject>) -> Result<WesoObject, String> {
    if args.len() != 2 {
        return Err(format!(
            "Runtime Error: Expected 2 arguments, get {}",
            args.len()
        ));
    } else {
        let lhs = &args[0];
        let rhs = &args[1];
        match (lhs.get_value(), rhs.get_value()) {
            (ObjectValue::Integer { value: v1 }, ObjectValue::Integer { value: v2 }) => {
                Ok(object::create_integer(lhs.get_typ(), v1 - v2))
            }
            (ObjectValue::Integer { value: v1 }, ObjectValue::Float { value: v2 }) => {
                Ok(object::create_float(lhs.get_typ(), f64::from(*v1) - v2))
            }
            (ObjectValue::Float { value: v1 }, ObjectValue::Integer { value: v2 }) => {
                Ok(object::create_float(lhs.get_typ(), v1 - f64::from(*v2)))
            }
            (ObjectValue::Float { value: v1 }, ObjectValue::Float { value: v2 }) => {
                Ok(object::create_float(lhs.get_typ(), v1 - v2))
            }
            (_, _) => Err(format!(
                "Runtime Error: function sub does not accept type {} and {}.",
                lhs.get_typ(),
                rhs.get_typ()
            )),
        }
    }
}

// *
pub fn std_mul(args: &Vec<WesoObject>) -> Result<WesoObject, String> {
    if args.len() != 2 {
        return Err(format!(
            "Runtime Error: Expected 2 arguments, get {}",
            args.len()
        ));
    } else {
        let lhs = &args[0];
        let rhs = &args[1];
        match (lhs.get_value(), rhs.get_value()) {
            (ObjectValue::Integer { value: v1 }, ObjectValue::Integer { value: v2 }) => {
                Ok(object::create_integer(lhs.get_typ(), v1 * v2))
            }
            (ObjectValue::Integer { value: v1 }, ObjectValue::Float { value: v2 }) => {
                Ok(object::create_float(lhs.get_typ(), f64::from(*v1) * v2))
            }
            (ObjectValue::Float { value: v1 }, ObjectValue::Integer { value: v2 }) => {
                Ok(object::create_float(lhs.get_typ(), v1 * f64::from(*v2)))
            }
            (ObjectValue::Float { value: v1 }, ObjectValue::Float { value: v2 }) => {
                Ok(object::create_float(lhs.get_typ(), v1 * v2))
            }
            (_, _) => Err(format!(
                "Runtime Error: function mul does not accept type {} and {}.",
                lhs.get_typ(),
                rhs.get_typ()
            )),
        }
    }
}

// /
pub fn std_div(args: &Vec<WesoObject>) -> Result<WesoObject, String> {
    if args.len() != 2 {
        return Err(format!(
            "Runtime Error: Expected 2 arguments, get {}",
            args.len()
        ));
    } else {
        let lhs = &args[0];
        let rhs = &args[1];
        match (lhs.get_value(), rhs.get_value()) {
            (ObjectValue::Integer { value: v1 }, ObjectValue::Integer { value: v2 }) => {
                Ok(object::create_integer(lhs.get_typ(), v1 / v2))
            }
            (ObjectValue::Integer { value: v1 }, ObjectValue::Float { value: v2 }) => {
                Ok(object::create_float(lhs.get_typ(), f64::from(*v1) / v2))
            }
            (ObjectValue::Float { value: v1 }, ObjectValue::Integer { value: v2 }) => {
                Ok(object::create_float(lhs.get_typ(), v1 / f64::from(*v2)))
            }
            (ObjectValue::Float { value: v1 }, ObjectValue::Float { value: v2 }) => {
                Ok(object::create_float(lhs.get_typ(), v1 / v2))
            }
            (_, _) => Err(format!(
                "Runtime Error: function div does not accept type {} and {}.",
                lhs.get_typ(),
                rhs.get_typ()
            )),
        }
    }
}

// 内建函数库
pub fn std_print(args: &Vec<WesoObject>) -> Result<WesoObject, String> {
    for arg in args {
        print!("{}", arg.to_string());
    }
    Ok(object::OBJ_UNIT.clone())
}

pub fn std_println(args: &Vec<WesoObject>) -> Result<WesoObject, String> {
    for arg in args {
        println!("{}", arg.to_string());
    }
    Ok(object::OBJ_UNIT.clone())
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn std_log(args: &Vec<WesoObject>) -> Result<WesoObject, String> {
    for arg in args {
        log(arg.to_string().as_str());
    }
    Ok(object::OBJ_UNIT.clone())
}
