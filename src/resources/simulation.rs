use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationConfig {
    pub particle_count: usize,
    pub gravity: Vec3,
    pub bounds: f32,
    pub speed_multiplier: f32,
    pub paused: bool,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            particle_count: 10_000,
            gravity: Vec3::new(0.0, -9.8, 0.0),
            bounds: 50.0,
            speed_multiplier: 1.0,
            paused: false,
        }
    }
}

#[derive(Resource, Default)]
pub struct SimulationStats {
    pub fps: f32,
    pub particle_count: usize,
}
