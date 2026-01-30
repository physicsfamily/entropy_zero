//! Components for the Ripple Tank simulation

use bevy::prelude::*;

use super::MAX_PROBE_HISTORY;

// ══════════════════════════════════════════════════════════════════════════════
// Scene Object Marker
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Component)]
pub struct SceneObject {
    pub id: u32,
    pub selected: bool,
    pub locked: bool,
}

// ══════════════════════════════════════════════════════════════════════════════
// Wave Sources
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum WaveSourceType {
    Point,
    Line,
    PhasedArray { count: u8 },
    Moving,
}

#[derive(Component, Reflect)]
pub struct WaveSource {
    pub source_type: WaveSourceType,
    pub frequency: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub enabled: bool,
    pub waveform: Waveform,
}

impl Default for WaveSource {
    fn default() -> Self {
        Self {
            source_type: WaveSourceType::Point,
            frequency: 2.0,
            amplitude: 1.0,
            phase: 0.0,
            enabled: true,
            waveform: Waveform::Sine,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum Waveform {
    #[default]
    Sine,
    Square,
    Pulse,
}

#[derive(Component, Reflect)]
pub struct MovingSource {
    pub velocity: Vec2,
    pub path: MovementPath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum MovementPath {
    #[default]
    Linear,
    Circular,
    Custom,
}

// ══════════════════════════════════════════════════════════════════════════════
// Obstacles
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum ObstacleType {
    Reflector,
    SingleSlit,
    DoubleSlit,
    RefractionBlock,
}

#[derive(Component, Reflect)]
pub struct Obstacle {
    pub obstacle_type: ObstacleType,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub slit_width: f32,
    pub slit_separation: f32,
    pub refractive_index: f32,
}

impl Default for Obstacle {
    fn default() -> Self {
        Self {
            obstacle_type: ObstacleType::Reflector,
            width: 50.0,
            height: 5.0,
            rotation: 0.0,
            slit_width: 10.0,
            slit_separation: 30.0,
            refractive_index: 1.5,
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Measurement Tools
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Component, Reflect)]
pub struct Probe {
    pub label: String,
    pub color: Color,
    #[reflect(ignore)]
    pub history: Vec<f32>,
}

impl Probe {
    pub fn new(label: &str, color: Color) -> Self {
        Self {
            label: label.to_string(),
            color,
            history: Vec::with_capacity(MAX_PROBE_HISTORY),
        }
    }
}

#[derive(Component, Reflect)]
pub struct Ruler {
    pub start: Vec2,
    pub end: Vec2,
}

// ══════════════════════════════════════════════════════════════════════════════
// Visual Markers
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Component)]
pub struct WaveFieldVisual {
    pub texture: Handle<Image>,
}
