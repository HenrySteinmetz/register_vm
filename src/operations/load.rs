use crate::{
    operands::literals::Literal, operands::register_values::RegisterValue, operands::Operand,
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
            Literal::Int(int) => self.registers[*register_idx] = RegisterValue::Int(*int),
            Literal::Float(float) => self.registers[*register_idx] = RegisterValue::Float(*float),
            Literal::String(string) => {
                self.registers[*register_idx] =
                    RegisterValue::String(self.alloc_string(string.clone()))
            }
            Literal::Bool(bool) => self.registers[*register_idx] = RegisterValue::Bool(*bool),
        }
    }
}
