use crate::{operands::Operand, vm::VM};

impl VM {
    pub fn jlne(&mut self, operands: Vec<Operand>) {
        if !self.operands_eq([&operands[1], &operands[2]]) {
            self.jl(vec![operands[0].clone()])
        }
    }
}
