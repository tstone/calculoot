use super::equation::Equation;

#[derive(Debug, PartialEq, Eq, Hash)]
#[allow(unused)]
pub enum NumberType {
    Whole,
    Negative,
}

#[derive(Debug, PartialEq, Eq)]
#[allow(unused)]
pub enum Value {
    Number(i16),
    Equation(Equation),
}

impl Value {
    pub fn to_i16(&self) -> i16 {
        match self {
            Self::Number(v) => *v,
            Self::Equation(e) => e.answer,
        }
    }

    pub fn difficulty(&self) -> u16 {
        match self {
            Self::Number(n) => {
                let digits = ((*n as f32).log10().floor() as u16) + 1;

                let sign = if *n < 0 { 10 } else { 1 };
                digits * sign
            }
            Self::Equation(e) => e.difficulty(),
        }
    }
}

impl Into<i16> for Value {
    fn into(self) -> i16 {
        self.to_i16()
    }
}

impl Into<Value> for i16 {
    fn into(self) -> Value {
        Value::Number(self)
    }
}
