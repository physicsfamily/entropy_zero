//! Core traits for simulation definitions.
//!
//! All simulations must implement the `Simulation` trait to integrate
//! with the Entropy Zero platform.

use bevy::prelude::*;

use crate::parameters::ParameterDef;
use crate::taxonomy::SimulationCategory;

/// Core trait that all simulations must implement.
///
/// This trait defines the contract between a simulation and the platform,
/// enabling dynamic discovery, configuration, and execution of simulations.
///
/// # Example
///
/// ```ignore
/// pub struct ProjectileSimulation;
///
/// impl Simulation for ProjectileSimulation {
///     fn id(&self) -> &'static str { "projectile_motion" }
///     fn name(&self) -> &'static str { "Projectile Motion" }
///     fn category(&self) -> SimulationCategory {
///         SimulationCategory::ClassicalMechanics(ClassicalMechanicsSubdomain::Kinematics)
///     }
///     fn description(&self) -> &'static str {
///         "Simulate projectile trajectories under gravity."
///     }
///     fn parameters(&self) -> Vec<ParameterDef> { vec![] }
///     fn build_plugin(&self) -> Box<dyn Fn(&mut App) + Send + Sync> {
///         Box::new(|app| { app.add_plugins(ProjectilePlugin); })
///     }
/// }
/// ```
pub trait Simulation: Send + Sync + 'static {
    /// Unique identifier for this simulation (snake_case).
    ///
    /// Used for routing, persistence, and programmatic access.
    fn id(&self) -> &'static str;

    /// Human-readable display name.
    fn name(&self) -> &'static str;

    /// Scientific category for taxonomy organization.
    fn category(&self) -> SimulationCategory;

    /// Description for UI display and documentation.
    fn description(&self) -> &'static str;

    /// Parameter schema for automatic UI generation.
    ///
    /// The platform will generate control panels based on these definitions.
    fn parameters(&self) -> Vec<ParameterDef>;

    /// Returns a closure that builds the Bevy plugin for this simulation.
    ///
    /// This allows the platform to dynamically load simulations.
    fn build_plugin(&self) -> Box<dyn Fn(&mut App) + Send + Sync>;

    /// Optional: Thumbnail asset path for gallery display.
    fn thumbnail(&self) -> Option<&'static str> {
        None
    }

    /// Optional: Educational difficulty level (1-5).
    fn difficulty(&self) -> u8 {
        3
    }

    /// Optional: Tags for search and filtering.
    fn tags(&self) -> &'static [&'static str] {
        &[]
    }
}

/// Simulation metadata for registry and UI display.
#[derive(Debug, Clone)]
pub struct SimulationMetadata {
    pub id: &'static str,
    pub name: &'static str,
    pub category: SimulationCategory,
    pub description: &'static str,
    pub difficulty: u8,
    pub tags: Vec<&'static str>,
    pub thumbnail: Option<&'static str>,
}

impl<T: Simulation> From<&T> for SimulationMetadata {
    fn from(sim: &T) -> Self {
        Self {
            id: sim.id(),
            name: sim.name(),
            category: sim.category(),
            description: sim.description(),
            difficulty: sim.difficulty(),
            tags: sim.tags().to_vec(),
            thumbnail: sim.thumbnail(),
        }
    }
}
