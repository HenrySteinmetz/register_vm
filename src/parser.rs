use std::str::FromStr;

use strum::IntoEnumIterator;

use crate::{operations::OpCode, vm::VM};

impl VM {
    pub fn from_file_contents(contents: String) -> Vec<u8> {
        let mut program = vec![];

        for line in contents.lines() {
            let all_op_codes: Vec<String> = OpCode::iter().map(|x| x.into()).collect();
            let mut tokens = line.split(' ');
            let operation = tokens.next().expect("No operation provided!").to_string();

            if !all_op_codes.contains(&operation) {
                panic!("Invalid operation: {}", operation);
            }

            let op_code = OpCode::from_str(&operation).unwrap();
            let operands: Vec<u8> = tokens
                .map(|x| x.parse::<u8>().expect("Failed to parse operand as u8"))
                .collect();
            program.push(op_code as u8);
            program.extend(operands);
        }

        program
    }
}
