use std::fmt::{Debug, Display};

use primes::is_prime;
use rand::Rng;
use rand::prelude::IteratorRandom;

use super::operation::{OperationConfig, OperationType};
use super::values::Value;

#[derive(Debug, PartialEq, Eq)]
#[allow(unused)]
pub struct Equation {
    pub left: Box<Value>,
    pub right: Box<Value>,
    pub op: OperationType,
    pub answer: i16,
}

impl Equation {
    pub fn new(left: Value, op: OperationType, right: Value, answer: i16) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
            op,
            answer,
        }
    }

    /// Generates an equation with up to 3 operations and 4 numbers ie. (a+b)+(c+d)
    pub fn rnd_compound(
        op_config: &OperationConfig,
        op_count: u8,
        rng: &mut impl Rng,
    ) -> Option<Self> {
        let answer = (op_config.answer_min..=op_config.answer_max)
            .choose(rng)
            .unwrap();
        match Self::rnd_single(answer, op_config, rng) {
            Some(mut eq) => {
                for _ in 1..op_count {
                    if rng.random_bool(0.5) {
                        match Self::rnd_single(eq.left.to_i16(), op_config, rng) {
                            Some(next) => eq.left = Box::new(Value::Equation(next)),
                            None => {}
                        }
                    } else {
                        match Self::rnd_single(eq.right.to_i16(), op_config, rng) {
                            Some(next) => eq.right = Box::new(Value::Equation(next)),
                            None => {}
                        }
                    }
                }
                Some(eq)
            }
            _ => None,
        }
    }

    /// Generates an equation with a single operation
    pub fn rnd_single(
        answer: i16,
        op_config: &OperationConfig,
        rng: &mut impl Rng,
    ) -> Option<Self> {
        let mut attempts = 0;
        let mut eq: Option<Self> = None;

        while attempts < 20 && eq.is_none() {
            eq = Self::rnd_from(answer, op_config, rng);
            attempts += 1;
        }

        eq
    }

    fn rnd_from(answer: i16, op_config: &OperationConfig, rng: &mut impl Rng) -> Option<Equation> {
        let op = if answer == 0 {
            op_config
                .allowed_operations
                .iter()
                .filter(|op| **op != OperationType::Multiply && **op != OperationType::Divide)
                .choose(rng)
        } else if answer > 0 && is_prime(answer as u64) {
            op_config
                .allowed_operations
                .iter()
                .filter(|op| **op != OperationType::Multiply)
                .choose(rng)
        } else {
            op_config.allowed_operations.iter().choose(rng)
        };

        match op {
            Some(OperationType::Add) => {
                let mut attempts = 10;
                while attempts > 0 {
                    let left = op_config.rnd_number(rng);
                    let right = answer - left;
                    if valid_range(left, right, op_config) {
                        return Some(Equation::new(
                            left.into(),
                            OperationType::Add,
                            right.into(),
                            answer,
                        ));
                    }
                    attempts -= 1;
                }
                return None;
            }
            Some(OperationType::Subtract) => {
                let mut attempts = 10;
                while attempts > 0 {
                    let left = op_config.rnd_number(rng);
                    let right = left - answer;
                    if valid_range(left, right, op_config) {
                        return Some(Equation::new(
                            left.into(),
                            OperationType::Subtract,
                            right.into(),
                            answer,
                        ));
                    }
                    attempts -= 1;
                }
                return None;
            }
            Some(OperationType::Multiply) => {
                let mut attempts = 10;
                while attempts > 0 {
                    let left = find_divisible(answer, op_config, rng);
                    let right = left / answer;
                    if right != 0 && valid_range(left, right, op_config) {
                        return Some(Equation::new(
                            left.into(),
                            OperationType::Multiply,
                            right.into(),
                            answer,
                        ));
                    }
                    attempts -= 1;
                }
                return None;
            }
            Some(OperationType::Divide) => {
                let mut attempts = 10;
                while attempts > 0 {
                    let right = op_config.rnd_number(rng);
                    let left = answer * right;
                    if left != 0 && right != 0 && valid_range(right, left, op_config) {
                        return Some(Equation::new(
                            left.into(),
                            OperationType::Divide,
                            right.into(),
                            answer,
                        ));
                    }
                    attempts -= 1;
                }
                return None;
            }
            _ => None,
        }
    }

    pub fn difficulty(&self) -> u16 {
        let left = match self.left.as_ref() {
            Value::Number(_) => self.left.difficulty(),
            Value::Equation(e) => e.difficulty(),
        };
        let right = match self.right.as_ref() {
            Value::Number(_) => self.right.difficulty(),
            Value::Equation(e) => e.difficulty(),
        };
        let op = match self.op {
            OperationType::Add => 1,
            OperationType::Subtract => 2,
            OperationType::Multiply => 4,
            OperationType::Divide => 4,
        };

        (left + right) * op
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", fmt_rec(self, 0), self.answer)
    }
}

fn fmt_rec(eq: &Equation, d: u8) -> String {
    let starting_parens = (0..=d).fold("".to_string(), |acc, _| format!("({acc}"));
    let ending_parens = (0..=d).fold("".to_string(), |acc, _| format!("{acc})"));
    let left = match eq.left.as_ref() {
        Value::Number(n) => format!("{n}"),
        Value::Equation(left) => {
            format!("{starting_parens}{}{ending_parens}", fmt_rec(left, d + 1))
        }
    };
    let right = match eq.right.as_ref() {
        Value::Number(n) => format!("{n}"),
        Value::Equation(right) => {
            format!("{starting_parens}{}{ending_parens}", fmt_rec(right, d + 1))
        }
    };

    format!("{left} {} {right}", eq.op)
}

fn valid_range(left: i16, right: i16, op_config: &OperationConfig) -> bool {
    left >= op_config.value_min
        && left <= op_config.value_max
        && right >= op_config.value_min
        && right <= op_config.value_max
}

fn find_divisible(start: i16, op_config: &OperationConfig, rng: &mut impl Rng) -> i16 {
    let mut attempts = 0;
    let mut left = op_config.rnd_number(rng);
    while (start as f32) % (left as f32) != 0.0 {
        if attempts == 10 {
            left = 1
        } else {
            left = op_config.rnd_number(rng);
            attempts += 1;
        }
    }
    left
}
