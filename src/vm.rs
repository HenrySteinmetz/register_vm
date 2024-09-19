use crate::operations::{Literal, OpCode, Operand, OperandType};

#[derive(Default)]
pub struct VM {
    pub registers: [u32; 32],
    pub program: Vec<u8>,
    pub program_counter: usize,
    // Name and location of the label
    pub labels: Vec<(Literal, usize)>,
}

impl VM {
    pub fn run(&mut self, program: Vec<u8>) {
        loop {
            self.program = program.clone();
            if self.program_counter >= self.program.len() {
                break;
            }
            let opcode = self.decode_opcode();
            if opcode == OpCode::STOP {
                self.halt();
            }

            let operand_type: OperandType = self.read_next_byte().into();

            if operand_type == opcode.expected_operands() {}

            self.execute(&opcode);
        }
    }

    pub fn decode_opcode(&mut self) -> OpCode {
        let opcode = OpCode::from(self.program[self.program_counter]);
        self.program_counter += 1;
        opcode
    }

    pub fn decode_operand(&mut self) -> Operand {
        let operand_type = self.read_next_byte();
        let operand = i64::from_le_bytes(self.read_next_8_bytes());

        match operand_type {
            0 => Operand::RegisterIndex(operand as u8),
            1 => Operand::Literal(Literal::Int(operand)),
            2 => Operand::Literal(Literal::Float(unsafe { std::mem::transmute(operand) })),
            3 => Operand::Literal(Literal::String(unsafe {
                let len: usize = std::mem::transmute(operand);
                let str_content = self.read_n_bytes_vec(len);
                std::mem::transmute(std::str::from_utf8_unchecked(str_content.as_slice()))
            })),
            _ => panic!("Found invalid operand type"),
        }
    }

    pub fn halt(&self) {
        std::process::exit(0);
    }

    pub fn read_next_byte(&mut self) -> u8 {
        let byte = self.program[self.program_counter];
        self.program_counter += 1;
        byte
    }

    pub fn read_n_bytes_vec(&mut self, num: usize) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(num);
        let _ = (0..num).map(|x| bytes[x] = self.read_next_byte());
        bytes
    }

    pub fn read_next_8_bytes(&mut self) -> [u8; 8] {
        self.read_n_bytes::<8>()
    }

    pub fn read_n_bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut bytes = [0u8; N];
        let _ = (0..N).map(|x| bytes[x] = self.read_next_byte());
        bytes
    }

    pub fn new() -> VM {
        VM::default()
    }

    fn execute(&mut self, operation: &OpCode) {
        use OpCode::*;
        match operation {
            STOP => self.halt(),
            LOAD => {}
            ADD => {}
            SUB => {}
            MUL => {}
            DIV => {}
            PRINT => {}
            JMP => {}
            JMPB => {}
            JMPF => {}
            JMPE => {}
            CL => {}
            JL => {}
            JLE => {}
        }
    }
}
