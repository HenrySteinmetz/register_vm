use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
pub enum RegisterValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(usize),
}

impl PartialEq for RegisterValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RegisterValue::Int(a), RegisterValue::Int(b)) => a == b,
            (RegisterValue::Float(a), RegisterValue::Float(b)) => a == b,
            (RegisterValue::Bool(a), RegisterValue::Bool(b)) => a == b,
            (RegisterValue::String(a), RegisterValue::String(b)) => a == b,
            _ => false,
        }
    }
}

impl Default for RegisterValue {
    fn default() -> Self {
        Self::Int(0)
    }
}
