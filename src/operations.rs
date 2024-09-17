#[derive(PartialEq, Debug, Clone)]
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
