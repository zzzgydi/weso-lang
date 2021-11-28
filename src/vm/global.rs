use crate::base::object::WesoObject;
use crate::vm::builtin;
use std::collections::HashMap;
use std::sync::Arc;
pub type WesoFunc = dyn Fn(&Vec<WesoObject>) -> Result<WesoObject, String>;

pub struct WesoSTD {
    hash: HashMap<String, Arc<WesoFunc>>,
}

macro_rules! hash_insert {
    ($hash:expr, $key:expr, $value:expr) => {
        $hash.insert(String::from($key), Arc::new(Box::new($value)));
    };
}

impl WesoSTD {
    pub fn get_ins() -> &'static Self {
        static mut WESO_STD: Option<Arc<WesoSTD>> = None;

        unsafe {
            WESO_STD.get_or_insert_with(|| {
                let mut hash: HashMap<String, Arc<WesoFunc>> = HashMap::new();
                hash_insert!(hash, "print", builtin::std_print);
                hash_insert!(hash, "println", builtin::std_println);
                hash_insert!(hash, "log", builtin::std_log);
                hash_insert!(hash, "equal", builtin::std_equal);
                hash_insert!(hash, "neq", builtin::std_noequal);
                hash_insert!(hash, "lt", builtin::std_lt);
                hash_insert!(hash, "gt", builtin::std_gt);
                hash_insert!(hash, "leq", builtin::std_leq);
                hash_insert!(hash, "geq", builtin::std_geq);
                hash_insert!(hash, "add", builtin::std_add);
                hash_insert!(hash, "sub", builtin::std_sub);
                hash_insert!(hash, "mul", builtin::std_mul);
                hash_insert!(hash, "div", builtin::std_div);

                Arc::new(WesoSTD { hash })
            });
            WESO_STD.as_ref().unwrap()
        }
    }

    pub fn get_func(&self, name: &String) -> Option<Arc<WesoFunc>> {
        match self.hash.get(name) {
            Some(f) => Some(f.clone()),
            None => None,
        }
    }
}
