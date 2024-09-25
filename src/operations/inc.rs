use crate::{
    operands::{register_values::RegisterValue, Operand},
    vm::VM,
};

impl VM {
    pub fn inc(&mut self, operands: Vec<Operand>) {
        let register_index = match operands[0] {
            Operand::RegisterIndex(register_index) => register_index,
            _ => unreachable!(),
        };
        let value = self.registers[register_index as usize];
        let int = match value {
            RegisterValue::Int(int) => int + 1,
            _ => panic!("Decrement expected register with integer value!"),
        };
        self.registers[register_index as usize] = RegisterValue::Int(int);
    }
}
