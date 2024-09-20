use std::fmt::Display;

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
            Self::RegisterValue(x) => Display::fmt(x, f),
            Self::RegisterIndex(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OperandType {
    Literal(LiteralType),
    RegisterIndex,
    RegisterValue,
    Any,
}

impl From<u8> for OperandType {
    fn from(value: u8) -> Self {
        match value {
            0 => OperandType::RegisterValue,
            1 => OperandType::Literal(LiteralType::Int),
            2 => OperandType::Literal(LiteralType::Float),
            3 => OperandType::Literal(LiteralType::String),
            4 => OperandType::Literal(LiteralType::Bool),
            5 => OperandType::RegisterIndex,
            _ => panic!("Invalid operand type: {:?}", value),
        }
    }
}