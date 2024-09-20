use crate::{operands::Operand, vm::VM};

impl VM {
    pub fn jl(&mut self, operands: Vec<Operand>) {
        let label = match &operands[0] {
            Operand::Literal(lit) => lit.get_string(),
            _ => unreachable!(),
        };

        match self.labels.get(&label) {
            Some(idx) => self.program_counter = *idx,
            None => panic!("A label woth the name `{}` does not exist", label),
        }
    }
}
