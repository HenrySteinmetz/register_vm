use crate::{operands::Operand, vm::VM};

impl VM {
    pub fn print(&mut self, operands: Vec<Operand>) {
        println!("{}", &operands[0])
    }
}
