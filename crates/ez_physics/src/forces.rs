//! Common force types.

use bevy::prelude::*;

/// Uniform gravitational field (e.g., near Earth's surface).
#[derive(Resource, Clone, Copy)]
pub struct UniformGravity {
    pub acceleration: Vec3,
}

impl Default for UniformGravity {
    fn default() -> Self {
        Self {
            acceleration: Vec3::new(0.0, -9.8, 0.0),
        }
    }
}

/// Point mass gravity (inverse square law).
pub fn gravitational_force(
    mass1: f32,
    mass2: f32,
    position1: Vec3,
    position2: Vec3,
    g: f32,
) -> Vec3 {
    let direction = position2 - position1;
    let distance_sq = direction.length_squared().max(0.01); // Avoid division by zero
    let magnitude = g * mass1 * mass2 / distance_sq;
    direction.normalize() * magnitude
}

/// Hooke's law spring force.
pub fn spring_force(
    position: Vec3,
    anchor: Vec3,
    rest_length: f32,
    stiffness: f32,
) -> Vec3 {
    let displacement = position - anchor;
    let current_length = displacement.length();
    if current_length < 0.001 {
        return Vec3::ZERO;
    }
    let extension = current_length - rest_length;
    let direction = displacement.normalize();
    -stiffness * extension * direction
}

/// Damping force (velocity-dependent).
pub fn damping_force(velocity: Vec3, coefficient: f32) -> Vec3 {
    -coefficient * velocity
}
