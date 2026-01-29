//! Common math utilities for simulations.
//!
//! Provides helper functions and types used across simulations.

use bevy::prelude::*;

/// Clamp a value between min and max.
#[inline]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

/// Linear interpolation between two values.
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Linear interpolation between two Vec3 values.
#[inline]
pub fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a + (b - a) * t
}

/// Smooth step function (Hermite interpolation).
#[inline]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Map a value from one range to another.
#[inline]
pub fn map_range(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    let normalized = (value - from_min) / (from_max - from_min);
    to_min + normalized * (to_max - to_min)
}

/// Physical constants (SI units).
pub mod constants {
    /// Speed of light in vacuum (m/s)
    pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

    /// Gravitational constant (m³/(kg·s²))
    pub const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11;

    /// Planck constant (J·s)
    pub const PLANCK_CONSTANT: f64 = 6.626e-34;

    /// Elementary charge (C)
    pub const ELEMENTARY_CHARGE: f64 = 1.602e-19;

    /// Boltzmann constant (J/K)
    pub const BOLTZMANN_CONSTANT: f64 = 1.381e-23;

    /// Vacuum permittivity (F/m)
    pub const VACUUM_PERMITTIVITY: f64 = 8.854e-12;

    /// Vacuum permeability (H/m)
    pub const VACUUM_PERMEABILITY: f64 = 1.257e-6;

    /// Standard gravity (m/s²)
    pub const STANDARD_GRAVITY: f32 = 9.80665;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        assert!((lerp(0.0, 10.0, 0.5) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_map_range() {
        assert!((map_range(5.0, 0.0, 10.0, 0.0, 100.0) - 50.0).abs() < 1e-6);
    }
}
