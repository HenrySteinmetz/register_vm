use std::ops::{Add, Div, Mul, Sub};

use super::RegisterValue;

pub enum Number {
    Int(i64),
    Float(f64),
}

impl Add for Number {
    type Output = RegisterValue;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(int1) => match rhs {
                Number::Int(int2) => RegisterValue::Int(int1 + int2),
                Number::Float(float1) => RegisterValue::Float(float1 + int1 as f64),
            },
            Number::Float(float1) => match rhs {
                Number::Int(int1) => RegisterValue::Float(float1 + int1 as f64),
                Number::Float(float2) => RegisterValue::Float(float1 + float2),
            },
        }
    }
}

impl Sub for Number {
    type Output = RegisterValue;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(int1) => match rhs {
                Number::Int(int2) => RegisterValue::Int(int1 - int2),
                Number::Float(float1) => RegisterValue::Float(float1 - int1 as f64),
            },
            Number::Float(float1) => match rhs {
                Number::Int(int1) => RegisterValue::Float(float1 - int1 as f64),
                Number::Float(float2) => RegisterValue::Float(float1 - float2),
            },
        }
    }
}

impl Mul for Number {
    type Output = RegisterValue;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(int1) => match rhs {
                Number::Int(int2) => RegisterValue::Int(int1 * int2),
                Number::Float(float1) => RegisterValue::Float(float1 * int1 as f64),
            },
            Number::Float(float1) => match rhs {
                Number::Int(int1) => RegisterValue::Float(float1 * int1 as f64),
                Number::Float(float2) => RegisterValue::Float(float1 * float2),
            },
        }
    }
}

impl Div for Number {
    type Output = RegisterValue;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(int1) => match rhs {
                Number::Int(int2) => RegisterValue::Int(int1 / int2),
                Number::Float(float1) => RegisterValue::Float(float1 / int1 as f64),
            },
            Number::Float(float1) => match rhs {
                Number::Int(int1) => RegisterValue::Float(float1 / int1 as f64),
                Number::Float(float2) => RegisterValue::Float(float1 / float2),
            },
        }
    }
}
