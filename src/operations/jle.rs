use crate::{
    operands::{literals::Literal, register_values::RegisterValue, Operand},
    vm::VM,
};

impl VM {
    pub fn jle(&mut self, operands: Vec<Operand>) {
        if self.operands_eq([&operands[1], &operands[2]]) {
            self.jl(vec![operands[0].clone()])
        }
    }

    pub fn operands_eq(&self, operands: [&Operand; 2]) -> bool {
        match (operands[0], operands[1]) {
            (Operand::Literal(a), Operand::Literal(b)) => a == b,
            (Operand::RegisterValue(a), Operand::RegisterValue(b)) => a == b,
            (Operand::RegisterIndex(a), Operand::RegisterIndex(b)) => a == b,
            (Operand::Literal(a), Operand::RegisterValue(b)) => match (a, b) {
                (Literal::Int(a), RegisterValue::Int(b)) => a == b,
                (Literal::Float(a), RegisterValue::Float(b)) => a == b,
                (Literal::String(a), RegisterValue::String(b)) => a == &self.strings[*b],
                (Literal::Bool(a), RegisterValue::Bool(b)) => a == b,
                _ => false,
            },
            (Operand::RegisterValue(a), Operand::Literal(b)) => match (a, b) {
                (RegisterValue::Int(a), Literal::Int(b)) => a == b,
                (RegisterValue::Float(a), Literal::Float(b)) => a == b,
                (RegisterValue::String(a), Literal::String(b)) => &self.strings[*a] == b,
                (RegisterValue::Bool(a), Literal::Bool(b)) => a == b,
                _ => false,
            },
            _ => false,
        }
    }
}
