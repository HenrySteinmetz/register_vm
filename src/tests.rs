use crate::{operands::register_values::RegisterValue, vm::VM};

#[test]
fn create_vm() {
    let vm = VM::new();
    assert_eq!(vm.registers, [(RegisterValue { int: 0 }, 0); 32]);
}

#[test]
fn assign_register() {
    let mut vm = VM::new();
    let instructions: Vec<u8> = vec![1, 5, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1];
    vm.run(instructions);
    assert_eq!(vm.registers[0], (RegisterValue { int: 1 }, 0));
}
