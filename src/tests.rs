use crate::{operands::register_values::RegisterValue, vm::VM};

#[test]
fn create_vm() {
    let vm = VM::new();
    assert_eq!(vm.registers, [(RegisterValue::default(), 0); 32]);
}
