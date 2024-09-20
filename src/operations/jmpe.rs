use crate::{operands::Operand, vm::VM};

impl VM {
    pub fn jmpe(&mut self, operands: Vec<Operand>) {
        let location = match &operands[0] {
            Operand::Literal(lit) => lit.get_int(),
            _ => unreachable!(),
        };

        if &operands[1] == &operands[2] {
            self.program_counter = location as usize;
        }
    }
}
