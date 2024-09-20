use crate::vm::VM;

impl VM {
    pub fn stop(&self) {
        self.halt();
    }
}
