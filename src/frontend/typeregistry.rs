use std::collections::HashMap;
use super::*;

pub struct TypeRegistry {
    registry: HashMap<String, Type>
}

#[allow(dead_code)]
impl TypeRegistry {
    pub fn new() -> Self {
        let mut registry: HashMap<String, Type> = HashMap::new();
        registry.insert("i8".to_string(), Type::Int8);
        registry.insert("i16".to_string(), Type::Int16);
        registry.insert("i32".to_string(), Type::Int32);
        registry.insert("i64".to_string(), Type::Int64);
        registry.insert("f32".to_string(), Type::Float32);
        registry.insert("f64".to_string(), Type::Float64);
        registry.insert("u8".to_string(), Type::UInt8);
        registry.insert("u16".to_string(), Type::UInt16);
        registry.insert("u32".to_string(), Type::UInt32);
        registry.insert("u64".to_string(), Type::UInt64);
        registry.insert("string".to_string(), Type::String);
        registry.insert("bool".to_string(), Type::Boolean);
        registry.insert("unit".to_string(), Type::Unit);
        Self {
            registry
        }
    }

    pub fn is_registered(&self, s: &str) -> bool {
        matches!(self.get(s), Some(_))
    }

    pub fn register(&mut self, s: &str, t: Type) {
        self.registry.insert(s.to_string(), t);
    }

    pub fn get(&self, s: &str) -> Option<Type> {
        self.registry.get(s).cloned()
    }

    pub fn remove(&mut self, s: &str) {
        self.registry.remove(s);
    }
}