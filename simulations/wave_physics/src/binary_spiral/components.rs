//! Components for the Binary Spiral simulation

use bevy::prelude::*;

// ══════════════════════════════════════════════════════════════════════════════
// Orbital Source (Star)
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Component, Reflect)]
pub struct OrbitalSource {
    pub index: usize,
    pub angle: f32,
    pub radius: f32,
    pub base_color: Color,
    pub velocity: Vec3,
    pub last_position: Vec3,
}

impl OrbitalSource {
    pub fn new(index: usize, angle_offset: f32, color: Color, radius: f32) -> Self {
        let pos = Vec3::new(
            angle_offset.cos() * radius,
            0.0,
            angle_offset.sin() * radius,
        );
        Self {
            index,
            angle: angle_offset,
            radius,
            base_color: color,
            velocity: Vec3::ZERO,
            last_position: pos,
        }
    }

    pub fn current_position(&self) -> Vec3 {
        Vec3::new(
            self.angle.cos() * self.radius,
            0.0,
            self.angle.sin() * self.radius,
        )
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Visual Markers
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Component)]
pub struct SourceGlow;

#[derive(Component)]
pub struct ParticleCloud;

#[derive(Component)]
pub struct OrbitRing;

#[derive(Component)]
pub struct GridFloor;
