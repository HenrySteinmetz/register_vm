use crate::operations::{Literal, LiteralType, OpCode, Operand, OperandType};

#[derive(Default)]
pub struct VM<'a> {
    pub registers: [u32; 32],
    pub program: Vec<u8>,
    pub program_counter: usize,
    // Name and location of the label
    pub labels: Vec<(&'a str, usize)>,
}

impl<'a> VM<'a> {
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

            let mut operands: Vec<Operand> = Vec::with_capacity(4);
            for param in opcode.expected_operands() {
                let operand_type: OperandType = self.read_next_byte().into();
                if &operand_type != param {
                    panic!("Expected operand type {:?}, found {:?}", param, operand_type);
                }

                operands.push(self.decode_operand(operand_type));
            }

            self.execute(&opcode, operands);
        }
    }

    pub fn decode_opcode(&mut self) -> OpCode {
        OpCode::from(self.read_next_byte())
    }

    pub fn decode_operand(&mut self, op_type: OperandType) -> Operand {
        match op_type {
            OperandType::RegisterValue => Operand::RegisterValue(self.registers[self.read_next_byte() as usize]),
            OperandType::Literal(l_type) => Operand::Literal(self.decode_literal(l_type)), 
            OperandType::Any => unreachable!()
        }
    }
    
    pub fn decode_literal(&mut self, l_type: LiteralType) -> Literal {
        match l_type {
            LiteralType::Int => Literal::Int(i64::from_le_bytes(self.read_next_8_bytes())),
            LiteralType::Float => Literal::Float(f64::from_le_bytes(self.read_next_8_bytes())),
            LiteralType::String => {
                let len = usize::from_le_bytes(self.read_next_8_bytes());
                let bytes = self.read_n_bytes_vec(len);
                let string = std::str::from_utf8(bytes.as_slice()).expect("Invalid UTF-8 string!");
                Literal::String(string)
            }
            LiteralType::Bool => Literal::Bool(self.read_next_byte() == 1),
            _ => unreachable!(),
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

    pub fn new() -> VM<'static> {
        VM::default()
    }

    fn execute(&mut self, operation: &OpCode, operands: Vec<Operand>) {
        use OpCode::*;
        match operation {
            STOP => self.halt(),
            LOAD => {
            }
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
