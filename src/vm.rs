use crate::operations::{Literal, OpCode, Operand};

#[derive(Default, Debug)]
pub struct VM {
    pub registers: [u32; 32],
    pub program: Vec<u8>,
    pub program_counter: usize,
    // Name and location of the label
    pub labels: Vec<(u8, usize)>,
    // Strings
    pub strings: Vec<&'static str>,
}

impl VM {
    pub fn run(&mut self, program: Vec<u8>) {
        loop {
            self.program = program.clone();
            if self.program_counter >= self.program.len() {
                break;
            }
            let opcode = self.decode_opcode();
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
            0 => Operand {
                register_index: operand as u8,
            },
            1 => Operand {
                literal: Literal { int: operand },
            },
            2 => Operand {
                literal: Literal {
                    float: unsafe { std::mem::transmute(operand) },
                },
            },
            3 => Operand {
                literal: Literal {
                    string: unsafe {
                        let str_length = self.read_next_8_bytes();
                        let slice = std::slice::from_raw_parts(
                            operand as *const u8,
                            usize::from_le_bytes(str_length),
                        );
                        std::str::from_utf8_unchecked(slice)
                    },
                },
            },
            _ => panic!("Found invalid operand type"),
        }
    }

    pub fn halt(&self) {
        std::process::exit(0);
    }

    pub fn read_next_byte(&mut self) -> u8 {
        self.read_n_bytes::<1>()[0]
    }

    pub fn read_next_8_bytes(&mut self) -> [u8; 8] {
        self.read_n_bytes::<8>()
    }

    pub fn read_n_bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut bytes = [0u8; N];
        (0..N).map(|x| bytes[x] = self.program[self.program_counter + x]);
        self.program_counter += N;
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
