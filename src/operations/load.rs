use crate::{
    operands::literals::{Literal, LiteralType},
    operands::register_values::RegisterValue,
    operands::Operand,
    vm::VM,
};

impl VM {
    pub fn load(&mut self, operands: Vec<Operand>) {
        let register_idx = match &operands[0] {
            Operand::RegisterIndex(idx) => idx,
            _ => unreachable!(),
        };

        let literal = match &operands[1] {
            Operand::Literal(x) => x,
            _ => unreachable!(),
        };

        match literal {
            Literal::Int(int) => {
                self.registers[*register_idx] =
                    (RegisterValue { int: *int }, LiteralType::Int as u8)
            }
            Literal::Float(float) => {
                self.registers[*register_idx] =
                    (RegisterValue { float: *float }, LiteralType::Float as u8)
            }
            Literal::String(string) => {
                self.registers[*register_idx] = (
                    RegisterValue {
                        string_ptr: self.alloc_string(string.clone()),
                    },
                    LiteralType::String as u8,
                )
            }
            Literal::Bool(bool) => {
                self.registers[*register_idx] =
                    (RegisterValue { bool: *bool }, LiteralType::Bool as u8)
            }
        }
    }
}