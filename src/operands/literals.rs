use super::numbers::Number;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

impl Literal {
    pub fn l_type(&self) -> LiteralType {
        match self {
            Literal::Int(_) => LiteralType::Int,
            Literal::Float(_) => LiteralType::Float,
            Literal::String(_) => LiteralType::String,
            Literal::Bool(_) => LiteralType::Bool,
        }
    }
}

impl Literal {
    pub fn get_num(&self) -> Number {
        match self {
            Self::Int(int) => Number::Int(*int),
            Self::Float(f64) => Number::Float(*f64),
            Self::Bool(_) => panic!(
                "Invalid literal type: expected a number (i.e. float or int) but got: `Bool`"
            ),
            Self::String(_) => panic!(
                "Invalid literal type: expected a number (i.e. float or int) but got: `String`"
            ),
        }
    }

    pub fn get_int(&self) -> i64 {
        match self {
            Self::Int(int) => *int,
            Self::Float(_) => panic!("Invalid literal type: expected an integer but got: `Float`"),
            Self::Bool(_) => panic!("Invalid literal type: expected a boolean but got: `Bool`"),
            Self::String(_) => panic!("Invalid literal type: expected a string but got: `String`"),
        }
    }

    pub fn get_bool(&self) -> bool {
        match self {
            Self::Bool(b) => *b,
            Self::Int(_) => panic!("Invalid literal type: expected a boolean but got: `Int`"),
            Self::Float(_) => panic!("Invalid literal type: expected a boolean but got: `Float`"),
            Self::String(_) => panic!("Invalid literal type: expected a boolean but got: `String`"),
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Int(_) => panic!("Invalid literal type: expected a string but got: `Int`"),
            Self::Float(_) => panic!("Invalid literal type: expected a string but got: `Float`"),
            Self::Bool(_) => panic!("Invalid literal type: expected a string but got: `Bool`"),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(float) => write!(f, "{}", float),
            Self::Bool(b) => write!(f, "{}", b),
            Self::String(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum LiteralType {
    Int = 0,
    Float = 1,
    String = 2,
    Bool = 3,
    Any = 255,
}

impl From<u8> for LiteralType {
    fn from(value: u8) -> Self {
        match value {
            0 => LiteralType::Int,
            1 => LiteralType::Float,
            2 => LiteralType::String,
            3 => LiteralType::Bool,
            255 => LiteralType::Any,
            _ => panic!("Invalid literal type: {:?}", value.to_le_bytes()),
        }
    }
}
