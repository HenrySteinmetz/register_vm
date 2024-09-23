use std::ops::{Add, Div, Mul, Sub};

use super::RegisterValue;

pub enum Number {
    Int(i64),
    Float(f64),
}

impl Add for Number {
    type Output = (RegisterValue, u8);

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(int1) => match rhs {
                Number::Int(int2) => (RegisterValue::Int(int1 + int2), 0),
                Number::Float(float1) => (RegisterValue::Float(float1 + int1 as f64), 1),
            },
            Number::Float(float1) => match rhs {
                Number::Int(int1) => (RegisterValue::Float(float1 + int1 as f64), 1),
                Number::Float(float2) => (RegisterValue::Float(float1 + float2), 1),
            },
        }
    }
}

impl Sub for Number {
    type Output = (RegisterValue, u8);

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(int1) => match rhs {
                Number::Int(int2) => (RegisterValue::Int(int1 - int2), 0),
                Number::Float(float1) => (RegisterValue::Float(float1 - int1 as f64), 1),
            },
            Number::Float(float1) => match rhs {
                Number::Int(int1) => (RegisterValue::Float(float1 - int1 as f64), 1),
                Number::Float(float2) => (RegisterValue::Float(float1 - float2), 1),
            },
        }
    }
}

impl Mul for Number {
    type Output = (RegisterValue, u8);

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(int1) => match rhs {
                Number::Int(int2) => (RegisterValue::Int(int1 * int2), 0),
                Number::Float(float1) => (RegisterValue::Float(float1 * int1 as f64), 1),
            },
            Number::Float(float1) => match rhs {
                Number::Int(int1) => (RegisterValue::Float(float1 * int1 as f64), 1),
                Number::Float(float2) => (RegisterValue::Float(float1 * float2), 1),
            },
        }
    }
}

impl Div for Number {
    type Output = (RegisterValue, u8);

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(int1) => match rhs {
                Number::Int(int2) => (RegisterValue::Int(int1 / int2), 0),
                Number::Float(float1) => (RegisterValue::Float(float1 / int1 as f64), 1),
            },
            Number::Float(float1) => match rhs {
                Number::Int(int1) => (RegisterValue::Float(float1 / int1 as f64), 1),
                Number::Float(float2) => (RegisterValue::Float(float1 / float2), 1),
            },
        }
    }
}
