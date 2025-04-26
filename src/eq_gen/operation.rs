use std::cmp;
use std::collections::HashSet;
use std::fmt::Display;

use rand::Rng;
use rand::prelude::IteratorRandom;

use super::{NumberType, Value};

#[derive(Debug, PartialEq, Eq, Hash)]
#[allow(unused)]
pub enum OperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Multiply => write!(f, "×"),
            Self::Divide => write!(f, "÷"),
        }
    }
}

pub struct OperationConfig {
    // the range of the final answer
    pub answer_min: i16,
    pub answer_max: i16,
    // the range of values used to calculate the answer
    pub value_min: i16,
    pub value_max: i16,
    pub allowed_numerics: HashSet<NumberType>,
    pub allowed_operations: HashSet<OperationType>,
}

impl OperationConfig {
    pub fn rnd_value(&self, rng: &mut impl Rng) -> Value {
        self.rnd_number(rng).into()
    }

    pub fn rnd_number(&self, rng: &mut impl Rng) -> i16 {
        if self.allowed_numerics.contains(&NumberType::Negative) && rng.random_bool(0.5) {
            self.rnd_negative(rng)
        } else {
            self.rnd_positive(rng)
        }
    }

    pub fn rnd_positive(&self, rng: &mut impl Rng) -> i16 {
        let min = cmp::max(0, self.value_min);
        let max = cmp::max(0, self.value_max);
        match (min..=max).choose(rng) {
            Some(v) => v,
            None => 1,
        }
    }

    pub fn rnd_negative(&self, rng: &mut impl Rng) -> i16 {
        let min = cmp::min(self.value_min, 0);
        let max = cmp::max(0, self.value_max);
        match (min..=max).choose(rng) {
            Some(v) => v,
            None => -1,
        }
    }
}
