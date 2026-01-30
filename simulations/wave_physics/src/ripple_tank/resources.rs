//! Resources for the Ripple Tank simulation

use bevy::prelude::*;

use super::{GRID_SCALE, GRID_WIDTH, GRID_HEIGHT};

// ══════════════════════════════════════════════════════════════════════════════
// Wave Field (FDTD Grid)
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct WaveField {
    pub current: Vec<f32>,
    pub previous: Vec<f32>,
    pub obstacle_map: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

impl Default for WaveField {
    fn default() -> Self {
        let size = GRID_WIDTH * GRID_HEIGHT;
        Self {
            current: vec![0.0; size],
            previous: vec![0.0; size],
            obstacle_map: vec![1.0; size],
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
        }
    }
}

impl WaveField {
    pub fn clear(&mut self) {
        self.current.fill(0.0);
        self.previous.fill(0.0);
    }

    pub fn clear_obstacles(&mut self) {
        self.obstacle_map.fill(1.0);
    }

    #[inline]
    pub fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn sample(&self, world_pos: Vec2) -> f32 {
        let grid_x = ((world_pos.x / GRID_SCALE) + (self.width as f32 / 2.0)) as usize;
        let grid_y = ((world_pos.y / GRID_SCALE) + (self.height as f32 / 2.0)) as usize;
        if grid_x < self.width && grid_y < self.height {
            self.current[self.idx(grid_x, grid_y)]
        } else {
            0.0
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Configuration
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct RippleTankConfig {
    pub wave_speed: f32,
    pub damping: f32,
    pub time_scale: f32,
    pub paused: bool,
    pub show_grid: bool,
    pub color_scheme: ColorScheme,
    pub accumulated_time: f32,
}

impl Default for RippleTankConfig {
    fn default() -> Self {
        Self {
            wave_speed: 1.0,
            damping: 0.995,
            time_scale: 1.0,
            paused: false,
            show_grid: true,
            color_scheme: ColorScheme::DeepOcean,
            accumulated_time: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorScheme {
    #[default]
    DeepOcean,
    Scientific,
    PhaseColor,
    Grayscale,
}

// ══════════════════════════════════════════════════════════════════════════════
// UI State
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Resource, Default)]
pub struct UIState {
    pub selected_tool: ToolType,
    pub selected_entity: Option<Entity>,
    pub dragging: Option<Entity>,
    pub drag_offset: Vec2,
    pub show_data_panel: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolType {
    #[default]
    Select,
    PointSource,
    LineSource,
    PhasedArray,
    MovingSource,
    Reflector,
    SingleSlit,
    DoubleSlit,
    RefractionBlock,
    Probe,
    Ruler,
}

// ══════════════════════════════════════════════════════════════════════════════
// Statistics
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Resource, Default)]
pub struct SimulationStats {
    pub fps: f32,
    pub simulation_time: f32,
    pub wave_energy: f32,
    pub probe_phase_diff: Option<f32>,
}

#[derive(Resource, Default)]
pub struct ObjectIdCounter(pub u32);
