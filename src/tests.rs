use crate::vm::VM;

#[test]
fn create_vm() {
    let vm = VM::new();
    assert_eq!(vm.registers, [0; 32]);
}

#[test]
fn assign_register() {
    let mut vm = VM::new();
    // LOAD $0 255
    let program = vec![1, 31, 255, 255];
    vm.run(program);
    assert_eq!(vm.registers[31], u16::MAX as u32);
}

#[test]
fn add_register() {
    let mut vm = VM::new();
    // LOAD $0 100; LOAD $1 100; ADD $0 $1 $2
    let program = vec![1, 1, 0, 100, 1, 2, 0, 100, 2, 0, 1, 2];
    vm.run(program);
    assert_eq!(vm.registers[0], 200);
}

#[test]
fn jump_exact() {
    let mut vm = VM::new();
    // LOAD $0 100; JMP $0
    let program = vec![1, 0, 0, 100, 7, 0];
    vm.run(program);
    assert_eq!(vm.program_counter, 100);
}

#[test]
fn jump_relative() {
    let mut vm = VM::new();
    // LOAD $0 100; JMPB 2; LOAD $1 200; PRINT
    let program = vec![1, 0, 0, 100, 7, 0, 9, 0, 8, 0];
    println!("pc: {:?}", vm.program_counter);
    vm.run(program);
    assert_eq!(vm.program_counter, 100);
}

#[test]
fn parse() {
    let contents = "LOAD 0 0 100\nPRINT 0".to_string();
    let program = VM::from_file_contents(contents);
    let mut vm = VM::new();
    vm.run(program);
    assert_eq!(vm.registers[0], 100);
}
