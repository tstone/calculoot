use std::collections::HashSet;

use bevy::prelude::*;
use rand::{SeedableRng, random_range};

use crate::eq_gen::{Equation, NumberType, OperationConfig, OperationType};
use crate::mode::GameMode;
use crate::seed::RngSeed;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub struct Levels;

impl Plugin for Levels {
    fn build(&self, app: &mut App) {
        app.init_resource::<EquationLevel>();
        app.add_observer(on_equation);
        app.add_systems(OnEnter(GameMode::InGame), setup);
        app.init_state::<GameMode>();
    }
}

fn setup(mut seed: ResMut<RngSeed>, mut level: ResMut<EquationLevel>, mut commands: Commands) {
    seed.0 = random_range(0..u64::MAX);
    let mut rng = ChaCha8Rng::seed_from_u64(seed.0);
    let cfg = OperationConfig {
        answer_min: 1,
        answer_max: 20,
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

    let mut equations: Vec<Equation> = Vec::new();
    for _ in 0..12 {
        let op_count = (1..=3).choose(&mut rng).unwrap();
        if let Some(eq) = Equation::rnd_compound(&cfg, op_count, &mut rng) {
            equations.push(eq);
        }
    }
    equations.sort();
    level.equations = equations.into_iter().take(6).collect::<Vec<_>>();

    commands.spawn(ActiveEquation(0));
}

fn on_equation(
    trigger: Trigger<OnAdd, ActiveEquation>,
    q: Query<&ActiveEquation>,
    level: Res<EquationLevel>,
) {
    let index = q.get(trigger.target()).unwrap();
    match level.equations.get(index.0) {
        None => {}
        Some(equation) => {
            println!("Ready to test user with {}", equation);
        }
    }
}

#[derive(Resource, Default)]
pub struct EquationLevel {
    equations: Vec<Equation>,
}

#[derive(Component)]
pub struct ActiveEquation(pub usize);
