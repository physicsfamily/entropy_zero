//! # EZ Core
//!
//! Core abstractions, traits, and taxonomy for the Entropy Zero simulation platform.
//!
//! This crate provides:
//! - The `Simulation` trait that all simulations must implement
//! - `SimulationCategory` enum for scientific classification
//! - Parameter definitions for UI generation
//! - Common math utilities

pub mod math;
pub mod parameters;
pub mod taxonomy;
pub mod traits;

pub use parameters::{ParameterDef, ParameterValue};
pub use taxonomy::SimulationCategory;
pub use traits::Simulation;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::math::*;
    pub use crate::parameters::{ParameterDef, ParameterValue};
    pub use crate::taxonomy::*;
    pub use crate::traits::Simulation;
}
