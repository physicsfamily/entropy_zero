//! Resources for the Binary Spiral simulation

use bevy::prelude::*;
use rand::Rng;

use super::{DEFAULT_EMISSION_RATE, DEFAULT_PARTICLE_LIFE, MAX_PARTICLES};

// ══════════════════════════════════════════════════════════════════════════════
// Configuration
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct BinarySpiralConfig {
    pub orbit_speed: f32,
    pub emission_rate: usize,
    pub particle_speed: f32,
    pub particle_life: u32,
    pub paused: bool,
    pub show_grid: bool,
    pub show_orbit_ring: bool,
}

impl Default for BinarySpiralConfig {
    fn default() -> Self {
        Self {
            orbit_speed: 1.5,
            emission_rate: DEFAULT_EMISSION_RATE,
            particle_speed: 2.0,
            particle_life: DEFAULT_PARTICLE_LIFE,
            paused: false,
            show_grid: true,
            show_orbit_ring: true,
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Particle Pool
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Default)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub color: [f32; 3],
    pub life: u32,
    pub active: bool,
}

#[derive(Resource)]
pub struct ParticlePool {
    pub particles: Vec<Particle>,
    pub next_index: usize,
}

impl Default for ParticlePool {
    fn default() -> Self {
        Self {
            particles: vec![Particle::default(); MAX_PARTICLES],
            next_index: 0,
        }
    }
}

impl ParticlePool {
    pub fn emit(&mut self, position: Vec3, velocity: Vec3, color: [f32; 3], life: u32) {
        let p = &mut self.particles[self.next_index];
        p.position = position;
        p.velocity = velocity;
        p.color = color;
        p.life = life;
        p.active = true;
        self.next_index = (self.next_index + 1) % MAX_PARTICLES;
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Drag State
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Resource, Default)]
pub struct DragState {
    pub dragging_source: Option<usize>,
    pub drag_target: Vec3,
}

// ══════════════════════════════════════════════════════════════════════════════
// Pre-computed Random Directions (for spherical emission)
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct RandomDirections {
    pub directions: Vec<Vec3>,
}

impl Default for RandomDirections {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let count = 5000;
        let mut directions = Vec::with_capacity(count);

        for _ in 0..count {
            let u: f32 = rng.gen_range(-1.0..1.0);
            let theta: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
            let r = (1.0 - u * u).sqrt();
            directions.push(Vec3::new(r * theta.cos(), u, r * theta.sin()));
        }

        Self { directions }
    }
}

impl RandomDirections {
    pub fn get(&self, rng: &mut impl Rng) -> Vec3 {
        let idx = rng.gen_range(0..self.directions.len());
        self.directions[idx]
    }
}
