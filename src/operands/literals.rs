use super::numbers::Number;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
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

#[derive(Debug)]
pub enum LiteralType {
    Int,
    Float,
    String,
    Bool,
    Any,
}

impl PartialEq for LiteralType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LiteralType::Int, LiteralType::Int) => true,
            (LiteralType::Float, LiteralType::Float) => true,
            (LiteralType::String, LiteralType::String) => true,
            (LiteralType::Bool, LiteralType::Bool) => true,
            (LiteralType::Any, _) => true,
            (_, LiteralType::Any) => true,
            _ => false,
        }
    }
}
