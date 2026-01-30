//! # Wave Physics Simulations
//!
//! This crate contains simulations related to wave physics:
//! - Ripple Tank (water wave interference, diffraction, refraction)
//! - Doppler Effect demonstrations
//! - Standing wave patterns

pub mod ripple_tank;

use bevy::prelude::*;
use ez_core::prelude::*;

/// All wave physics simulations.
pub fn all_simulations() -> Vec<Box<dyn Simulation>> {
    vec![Box::new(ripple_tank::RippleTankSimulation)]
}

/// Plugin that registers all wave physics simulations.
pub struct WavePhysicsPlugin;

impl Plugin for WavePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ripple_tank::RippleTankPlugin);
    }
}
