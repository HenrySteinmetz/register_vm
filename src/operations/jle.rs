use crate::{operands::Operand, vm::VM};

impl VM {
    pub fn jle(&mut self, operands: Vec<Operand>) {
        if &operands[1] == &operands[2] {
            self.jl(vec![operands[0].clone()])
        }
    }
}
