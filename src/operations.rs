use strum::{EnumIter, EnumString};

#[derive(PartialEq, Debug, Clone, EnumString, EnumIter)]
#[repr(u8)]
pub enum OpCode {
    STOP = 0,
    LOAD = 1,
    ADD = 2,
    SUB = 3,
    MUL = 4,
    DIV = 5,
    PRINT = 6,
    JMP = 7,
    JMPB = 8,
    JMPF = 9,
    JMPE = 10,
    CL = 11,
    JL = 12,
    JLE = 13,
}

pub union Operand {
    pub literal: Literal,
    pub register_index: u8,
    pub mem_address: *mut str,
}

#[derive(Copy, Clone)]
pub union Literal {
    pub int: i64,
    pub float: f64,
    pub string: &'static str,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::STOP,
            1 => OpCode::LOAD,
            2 => OpCode::ADD,
            3 => OpCode::SUB,
            4 => OpCode::MUL,
            5 => OpCode::DIV,
            6 => OpCode::PRINT,
            7 => OpCode::JMP,
            8 => OpCode::JMPB,
            9 => OpCode::JMPF,
            10 => OpCode::JMPE,
            11 => OpCode::CL,
            12 => OpCode::JL,
            13 => OpCode::JLE,
            _ => panic!("Invalid OpCode"),
        }
    }
}

impl Into<String> for OpCode {
    fn into(self) -> String {
        match self {
            OpCode::STOP => "STOP".to_string(),
            OpCode::LOAD => "LOAD".to_string(),
            OpCode::ADD => "ADD".to_string(),
            OpCode::SUB => "SUB".to_string(),
            OpCode::MUL => "MUL".to_string(),
            OpCode::DIV => "DIV".to_string(),
            OpCode::PRINT => "PRINT".to_string(),
            OpCode::JMP => "JMP".to_string(),
            OpCode::JMPB => "JMPB".to_string(),
            OpCode::JMPF => "JMPF".to_string(),
            OpCode::JMPE => "JMPE".to_string(),
            OpCode::CL => "CL".to_string(),
            OpCode::JL => "JL".to_string(),
            OpCode::JLE => "JLE".to_string(),
        }
    }
}
