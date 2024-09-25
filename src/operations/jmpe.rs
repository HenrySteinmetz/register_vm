use crate::{operands::Operand, vm::VM};

impl VM {
    pub fn jmpe(&mut self, operands: Vec<Operand>) {
        let location = match &operands[0] {
            Operand::Literal(lit) => lit.get_int(),
            _ => unreachable!(),
        };

        if self.operands_eq([&operands[0], &operands[1]]) {
            self.program_counter = location as usize;
        }
    }
}
