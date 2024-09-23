use crate::{
    operands::{register_values::RegisterValue, Operand},
    vm::VM,
};

impl VM {
    pub fn print(&mut self, operands: Vec<Operand>) {
        self.print_operand(&operands[0]);
    }

    pub fn print_operand(&self, operand: &Operand) {
        match operand {
            Operand::Literal(literal) => println!("{}", literal),
            Operand::RegisterIndex(index) => println!("{}", index),
            Operand::RegisterValue(value) => match value {
                RegisterValue::Int(value) => println!("{}", value),
                RegisterValue::Float(value) => println!("{}", value),
                RegisterValue::Bool(value) => println!("{}", value),
                RegisterValue::String(idx) => println!("{}", self.strings[*idx]),
            },
        }
    }
}
