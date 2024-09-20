use crate::{operands::Operand, vm::VM};

impl VM {
    pub fn jmpf(&mut self, operands: Vec<Operand>) {
        match &operands[0] {
            Operand::Literal(lit) => self.program_counter = lit.get_int() as usize,
            _ => unreachable!(),
        }
    }
}
