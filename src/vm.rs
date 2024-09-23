use std::collections::HashMap;

use crate::operands::{
    literals::{Literal, LiteralType},
    register_values::RegisterValue,
    Operand, OperandType,
};

use crate::operations::OpCode;

#[derive(Default)]
pub struct VM {
    // Value and type of the register
    pub registers: [(RegisterValue, u8); 32],
    pub program: Vec<u8>,
    pub program_counter: usize,
    // Name and location of the label
    pub labels: HashMap<String, usize>,
    pub strings: Vec<String>,
}

impl VM {
    pub fn decode_opcode(&mut self) -> OpCode {
        OpCode::from(self.read_next_byte())
    }

    pub fn decode_operand(&mut self, op_type: &OperandType, verbose: bool) -> Operand {
        if verbose {
            println!("\n  --- Decoding operand of type {:?} ---", op_type);
        }

        match op_type {
            OperandType::RegisterIndex => Operand::RegisterIndex(self.read_next_byte() as usize),
            OperandType::RegisterValue => {
                Operand::RegisterValue(self.registers[self.read_next_byte() as usize].0)
            }
            OperandType::Literal(l_type) => Operand::Literal(self.decode_literal(l_type, verbose)),
            OperandType::Any => unreachable!(),
        }
    }

    pub fn decode_literal(&mut self, l_type: &LiteralType, verbose: bool) -> Literal {
        if verbose {
            println!("\n   --- Decoding literal of type: {:?} ---", l_type);
        }
        match l_type {
            LiteralType::Int => Literal::Int(i64::from_le_bytes(self.read_next_8_bytes())),
            LiteralType::Float => Literal::Float(f64::from_le_bytes(self.read_next_8_bytes())),
            LiteralType::String => {
                let len_bytes = self.read_next_8_bytes();
                let len = usize::from_le_bytes(len_bytes);
                let bytes = self.read_n_bytes(len);

                if verbose {
                    println!("    Length bytes: {:?}", len_bytes);
                    println!("    String length: {}", len);
                    println!("    UTF-8 bytes: {:?}", bytes);
                }

                let string = std::str::from_utf8(bytes.as_slice())
                    .expect("Invalid UTF-8 string!")
                    .to_owned();
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

    pub fn read_next_8_bytes(&mut self) -> [u8; 8] {
        let bytes = self.read_n_bytes(8);

        bytes.try_into().unwrap_or_else(|v: Vec<u8>| {
            panic!("Expected a Vec of length 8 but it was {}", v.len())
        })
    }

    pub fn read_n_bytes(&mut self, num: usize) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(num);
        for _ in 0..num {
            bytes.push(self.read_next_byte());
        }
        bytes
    }

    #[inline(always)]
    pub fn new() -> VM {
        VM::default()
    }

    pub fn alloc_string(&mut self, str: String) -> usize {
        self.strings.push(str.to_string());
        self.strings.len() - 1
    }

    pub fn run(&mut self, program: Vec<u8>, verbose: bool) {
        if verbose {
            println!("---------------------------------\n|\tVM has started...\t|\n---------------------------------\n\n");
        }

        self.program = program;

        loop {
            if verbose {
                println!("--- New Instruction ---");
                println!("Program counter: {}", self.program_counter);
            }

            if self.program_counter >= self.program.len() {
                break;
            }

            let opcode = self.decode_opcode();
            #[cfg(test)]
            if opcode == OpCode::STOP {
                println!("Stop was called!");
            }

            let mut operands: Vec<Operand> = Vec::with_capacity(4);

            if verbose {
                println!(
                    "Opcode: {:?}\nExpected operands: {:?}",
                    opcode,
                    opcode.expected_operands()
                );
            }

            for ex in opcode.expected_operands() {
                let operand_type: OperandType = self.read_next_byte().into();
                if &operand_type != ex {
                    if verbose {
                        println!("Program dump: \n{:?}", self.program);
                    }
                    panic!("Expected operand type {:?}, found {:?}", ex, operand_type);
                }

                operands.push(self.decode_operand(&operand_type, verbose));
            }

            if verbose {
                println!("\nOperands: {:?}\n", operands);
            }

            self.execute(&opcode, operands);
        }

        println!("\n\n Unexpected end of program!");
        if verbose {
            println!("\nProgram dump:\n{:?}", self.program);
        }
    }

    fn execute(&mut self, operation: &OpCode, operands: Vec<Operand>) {
        use OpCode::*;
        match operation {
            STOP => self.stop(),
            LOAD => self.load(operands),
            ADD => self.add(operands),
            SUB => self.sub(operands),
            MUL => self.mul(operands),
            DIV => self.div(operands),
            PRINT => self.print(operands),
            JMP => self.jmp(operands),
            JMPB => self.jmpb(operands),
            JMPF => self.jmpf(operands),
            JMPE => self.jmpe(operands),
            CL => self.cl(operands),
            JL => self.jl(operands),
            JLE => self.jle(operands),
        }
    }
}
