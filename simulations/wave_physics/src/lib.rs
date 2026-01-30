//! # Wave Physics Simulations
//!
//! This crate contains simulations related to wave physics:
//! - Ripple Tank (water wave interference, diffraction, refraction)
//! - Binary Spiral (double star particle emission density field)
//! - Doppler Effect demonstrations
//! - Standing wave patterns

pub mod binary_spiral;
pub mod ripple_tank;

use bevy::prelude::*;
use ez_core::prelude::*;

/// All wave physics simulations.
pub fn all_simulations() -> Vec<Box<dyn Simulation>> {
    vec![
        Box::new(ripple_tank::RippleTankSimulation),
        Box::new(binary_spiral::BinarySpiralSimulation),
    ]
}

/// Plugin that registers all wave physics simulations.
pub struct WavePhysicsPlugin;

impl Plugin for WavePhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Only one simulation plugin should be active at a time
        // Comment/uncomment to switch simulations
        // app.add_plugins(ripple_tank::RippleTankPlugin);
        app.add_plugins(binary_spiral::BinarySpiralPlugin);
    }
}
