//! # EZ Physics
//!
//! Shared physics primitives for Entropy Zero simulations.
//!
//! Provides:
//! - Force types (gravity, springs, EM)
//! - Numerical integrators (Euler, RK4, Verlet)
//! - Collision detection primitives

pub mod forces;
pub mod integrators;

/// Prelude for convenient imports.
pub mod prelude {
    pub use crate::forces::*;
    pub use crate::integrators::*;
}
