use crate::operations::OpCode;
use crate::utils::ToU16;

#[derive(Default, Debug)]
pub struct VM {
    registers: [u32; 32],
    program: Vec<u8>,
    program_counter: usize,
    // Name and location of the label
    labels: Vec<(u8, usize)>,
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

    pub fn halt(&self) {
        std::process::exit(0);
    }

    pub fn read_next_byte(&mut self) -> u8 {
        let byte = self.program[self.program_counter];
        self.program_counter += 1;
        byte
    }

    pub fn read_next_byte_as_register(&mut self) -> u32 {
        let index = self.read_next_byte() as usize;
        self.registers[index]
    }

    pub fn read_next_two_bytes(&mut self) -> [u8; 2] {
        let bytes = [
            self.program[self.program_counter],
            self.program[self.program_counter + 1],
        ];
        self.program_counter += 2;
        bytes
    }

    pub fn new() -> VM {
        VM::default()
    }

    fn execute(&mut self, operation: &OpCode) {
        use OpCode::*;
        match operation {
            STOP => self.halt(),
            LOAD => {
                let register = self.read_next_byte() as usize;
                let value = self.read_next_two_bytes().to_u16();
                self.registers[register] = value as u32;
            }
            ADD => {
                let output = self.read_next_byte() as usize;
                let input = self.read_next_two_bytes();
                self.registers[output] =
                    (self.registers[input[0] as usize] + self.registers[input[1] as usize]) as u32;
            }
            SUB => {
                let output = self.read_next_byte() as usize;
                let input = self.read_next_two_bytes();
                self.registers[output] =
                    (self.registers[input[0] as usize] - self.registers[input[1] as usize]) as u32;
            }
            MUL => {
                let output = self.read_next_byte() as usize;
                let input = self.read_next_two_bytes();
                self.registers[output] =
                    (self.registers[input[0] as usize] * self.registers[input[1] as usize]) as u32;
            }
            DIV => {
                let output = self.read_next_byte() as usize;
                let input = self.read_next_two_bytes();
                self.registers[output] =
                    (self.registers[input[0] as usize] / self.registers[input[1] as usize]) as u32;
            }
            PRINT => {
                println!("{}", self.read_next_byte_as_register());
            }
            JMP => {
                let position = self.read_next_byte_as_register() as usize;
                self.program_counter = position;
            }
            JMPB => {
                let factor = self.read_next_byte_as_register() as usize;
                self.program_counter -= factor;
            }
            JMPF => {
                let factor = self.read_next_byte_as_register() as usize;
                self.program_counter += factor;
            }
            JMPE => {
                let op1 = self.read_next_byte_as_register();
                let op2 = self.read_next_byte_as_register();
                let pos = self.read_next_byte() as usize;

                if op1 == op2 {
                    self.program_counter = pos;
                }
            }
            CL => {
                let name = self.read_next_byte_as_register() as u8;
                self.labels.push((name, self.program_counter));
            }
            JL => {
                let name = self.read_next_byte_as_register() as u8;
                let label = self
                    .labels
                    .iter()
                    .find(|label| label.0 == name)
                    .expect("Invalid label");

                self.program_counter = label.1;
            }
            JLE => {
                let name = self.read_next_byte_as_register() as u8;
                let op1 = self.read_next_byte_as_register();
                let op2 = self.read_next_byte_as_register();

                if op1 == op2 {
                    let label = self
                        .labels
                        .iter()
                        .find(|label| label.0 == name)
                        .expect("Invalid label");

                    self.program_counter = label.1;
                }
            }
        }
    }
}
