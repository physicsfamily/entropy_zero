//! Numerical integrators for physics simulations.

use bevy::prelude::*;

/// Euler integration (first-order, simple but less accurate).
pub fn euler_integrate(
    position: &mut Vec3,
    velocity: &mut Vec3,
    acceleration: Vec3,
    dt: f32,
) {
    *velocity += acceleration * dt;
    *position += *velocity * dt;
}

/// Semi-implicit Euler (symplectic, better energy conservation).
pub fn semi_implicit_euler(
    position: &mut Vec3,
    velocity: &mut Vec3,
    acceleration: Vec3,
    dt: f32,
) {
    *velocity += acceleration * dt;
    *position += *velocity * dt;
}

/// Velocity Verlet integration (second-order, good for physics).
pub fn verlet_integrate(
    position: &mut Vec3,
    velocity: &mut Vec3,
    acceleration: Vec3,
    prev_acceleration: Vec3,
    dt: f32,
) {
    *position += *velocity * dt + 0.5 * prev_acceleration * dt * dt;
    *velocity += 0.5 * (prev_acceleration + acceleration) * dt;
}

/// Fourth-order Runge-Kutta (high accuracy, expensive).
pub fn rk4_integrate<F>(
    position: &mut Vec3,
    velocity: &mut Vec3,
    dt: f32,
    acceleration_fn: F,
) where
    F: Fn(Vec3, Vec3) -> Vec3,
{
    let k1v = acceleration_fn(*position, *velocity);
    let k1x = *velocity;

    let k2v = acceleration_fn(*position + k1x * dt * 0.5, *velocity + k1v * dt * 0.5);
    let k2x = *velocity + k1v * dt * 0.5;

    let k3v = acceleration_fn(*position + k2x * dt * 0.5, *velocity + k2v * dt * 0.5);
    let k3x = *velocity + k2v * dt * 0.5;

    let k4v = acceleration_fn(*position + k3x * dt, *velocity + k3v * dt);
    let k4x = *velocity + k3v * dt;

    *position += (k1x + 2.0 * k2x + 2.0 * k3x + k4x) * dt / 6.0;
    *velocity += (k1v + 2.0 * k2v + 2.0 * k3v + k4v) * dt / 6.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_free_fall() {
        let mut pos = Vec3::ZERO;
        let mut vel = Vec3::ZERO;
        let acc = Vec3::new(0.0, -10.0, 0.0);

        for _ in 0..100 {
            euler_integrate(&mut pos, &mut vel, acc, 0.01);
        }

        // After 1 second: v = -10 m/s, y ≈ -5 m
        assert!((vel.y - (-10.0)).abs() < 0.1);
        assert!((pos.y - (-5.0)).abs() < 0.5);
    }
}
