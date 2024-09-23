use std::fmt::{Debug, Display};

pub mod literals;
pub mod numbers;
pub mod register_values;

use literals::{Literal, LiteralType};
use register_values::RegisterValue;

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Literal(Literal),
    RegisterValue(RegisterValue),
    RegisterIndex(usize),
}

impl Operand {
    pub fn op_type(&self) -> OperandType {
        match self {
            Operand::Literal(x) => OperandType::Literal(x.l_type()),
            Operand::RegisterValue(_) => OperandType::RegisterValue,
            Operand::RegisterIndex(_) => OperandType::RegisterIndex,
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(x) => Display::fmt(x, f),
            Self::RegisterValue(x) => Debug::fmt(x, f),
            Self::RegisterIndex(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug)]
pub enum OperandType {
    Literal(LiteralType),
    RegisterIndex,
    RegisterValue,
    Any,
}

impl PartialEq for OperandType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OperandType::Literal(a), OperandType::Literal(b)) => a == b,
            (OperandType::RegisterIndex, OperandType::RegisterIndex) => true,
            (OperandType::RegisterValue, OperandType::RegisterValue) => true,
            (OperandType::Any, _) => true,
            (_, OperandType::Any) => true,
            _ => false,
        }
    }
}

impl From<u8> for OperandType {
    fn from(value: u8) -> Self {
        match value {
            0 => OperandType::RegisterIndex,
            1 => OperandType::Literal(LiteralType::Int),
            2 => OperandType::Literal(LiteralType::Float),
            3 => OperandType::Literal(LiteralType::String),
            4 => OperandType::Literal(LiteralType::Bool),
            5 => OperandType::RegisterValue,
            _ => panic!("Invalid operand type: {:?}", value),
        }
    }
}
