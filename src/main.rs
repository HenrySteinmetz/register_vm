#[cfg(test)]
mod tests;

mod operands;
mod operations;
mod vm;

fn main() {
    let program: Vec<u8> = vec![2, 0, 2, 2, 6, 0];
    let mut vm = vm::VM::new();
    vm.run(program);
}
