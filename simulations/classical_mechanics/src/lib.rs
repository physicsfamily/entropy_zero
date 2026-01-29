//! # Classical Mechanics Simulations
//!
//! This crate contains simulations related to classical/Newtonian mechanics:
//! - Particle systems (gravity, bouncing)
//! - N-body gravitational simulations
//! - Projectile motion
//! - Pendulums (simple, double, chaotic)
//! - Spring-mass systems

pub mod particle_system;

use bevy::prelude::*;
use ez_core::prelude::*;

/// All classical mechanics simulations.
pub fn all_simulations() -> Vec<Box<dyn Simulation>> {
    vec![Box::new(particle_system::ParticleSystemSimulation)]
}

/// Plugin that registers all classical mechanics simulations.
pub struct ClassicalMechanicsPlugin;

impl Plugin for ClassicalMechanicsPlugin {
    fn build(&self, app: &mut App) {
        // Register all simulation plugins
        app.add_plugins(particle_system::ParticleSystemPlugin);
    }
}
