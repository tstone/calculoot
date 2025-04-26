use std::collections::HashSet;

use eq_gen::{Equation, NumberType, OperationConfig, OperationType};
use rand::{prelude::*, random_range};
use rand_chacha::ChaCha8Rng;

mod eq_gen;

fn main() {
    let seed = random_range(0..u64::MAX);
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let cfg = OperationConfig {
        answer_min: 1,
        answer_max: 10,
        value_min: -10,
        value_max: 20,
        allowed_numerics: HashSet::from([NumberType::Whole, NumberType::Negative]),
        allowed_operations: HashSet::from([
            OperationType::Add,
            OperationType::Subtract,
            OperationType::Multiply,
            OperationType::Divide,
        ]),
    };

    for _ in 0..10 {
        let op_count = (1..=3).choose(&mut rng).unwrap();
        if let Some(eq) = Equation::rnd_complex(&cfg, op_count, &mut rng) {
            println!("{} (measured difficulty: {})", eq, eq.difficulty());
        }
    }
}
