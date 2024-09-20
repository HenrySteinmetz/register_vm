use crate::{operands::Operand, vm::VM};

impl VM {
    pub fn cl(&mut self, operands: Vec<Operand>) {
        let location = self.program_counter + 1 as usize;
        let label = match &operands[0] {
            Operand::Literal(lit) => lit.get_string(),
            _ => unreachable!(),
        };

        if self.labels.get(&label).is_some() {
            panic!("A label with the name `{}` already exists", label);
        }

        self.labels.insert(label, location);
    }
}
