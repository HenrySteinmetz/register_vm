use crate::{operands::Operand, vm::VM};

impl VM {
    pub fn add(&mut self, operands: Vec<Operand>) {
        let register_idx = match &operands[0] {
            Operand::RegisterIndex(idx) => idx,
            _ => unreachable!(),
        };

        let number1 = match &operands[1] {
            Operand::Literal(lit) => lit.get_num(),
            _ => unreachable!(),
        };

        let number2 = match &operands[2] {
            Operand::Literal(lit) => lit.get_num(),
            _ => unreachable!(),
        };

        self.registers[*register_idx] = number1 + number2;
    }
}
