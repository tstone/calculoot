use bevy::prelude::*;
#[allow(unused)]
use rand::random_range;

pub struct SeedPlugin;

impl Plugin for SeedPlugin {
    fn build(&self, app: &mut App) {
        // let seed = random_range(0..u64::MAX);
        let seed = 10996829745885157783;
        debug!("Using rng seed: {seed}");
        app.insert_resource(RngSeed(seed));
    }
}

#[derive(Resource)]
pub struct RngSeed(pub u64);
