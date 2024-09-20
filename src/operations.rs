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

#[derive(Debug)]
pub enum Operand<'a> {
    Literal(Literal<'a>),
    RegisterValue(u32),
    RegisterIndex(usize),
}

impl<'a> Operand<'a> {
    pub fn op_type(&self) -> OperandType {
        match self {
            Operand::Literal(x) => OperandType::Literal(x.l_type()),
            Operand::RegisterValue(_) => OperandType::RegisterValue,
            Operand::RegisterIndex(_) => OperandType::RegisterIndex,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OperandType {
    Literal(LiteralType),
    RegisterIndex,
    RegisterValue,
    Any,
}

impl From<u8> for OperandType {
    fn from(value: u8) -> Self {
        match value {
            0 => OperandType::RegisterValue,
            1 => OperandType::Literal(LiteralType::Int),
            2 => OperandType::Literal(LiteralType::Float),
            3 => OperandType::Literal(LiteralType::String),
            4 => OperandType::Literal(LiteralType::Bool),
            5 => OperandType::RegisterIndex,
            _ => panic!("Invalid operand type: {:?}", value),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Literal<'a> {
    Int(i64),
    Float(f64),
    String(&'a str),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum LiteralType {
    Int,
    Float,
    String,
    Bool,
    Any,
}

impl<'a> Literal<'a> {
    pub fn l_type(&self) -> LiteralType {
        match self {
            Literal::Int(_) => LiteralType::Int,
            Literal::Float(_) => LiteralType::Float,
            Literal::String(_) => LiteralType::String,
            Literal::Bool(_) => LiteralType::Bool,
        }
    }
}

impl OpCode {
    pub fn expected_operands(&self) -> &[OperandType] {
        use OpCode::*;
        use OperandType::*;
        match self {
            STOP => &[],
            LOAD => &[RegisterIndex, Literal(LiteralType::Any)],
            ADD | SUB | MUL | DIV => &[RegisterIndex, Any, Any],
            PRINT | JMP | JMPB | JMPF | JL => &[Any],
            JMPE | JLE => &[Any, Any, Any],
            CL => &[Literal(LiteralType::String)],
        }
    }
    
    pub fn expected_operands_count(&self) -> usize {
        self.expected_operands().len()
    }
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
