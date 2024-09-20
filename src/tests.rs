use crate::vm::VM;

#[test]
fn create_vm() {
    let vm = VM::new();
    assert_eq!(vm.registers, [0; 32]);
}
