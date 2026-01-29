//! Grid and axis visualization.

use bevy::prelude::*;

/// Plugin for grid visualization.
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, _app: &mut App) {
        // Grid rendering will be added in future updates
    }
}

/// Configuration for grid display.
#[derive(Resource, Clone)]
pub struct GridConfig {
    pub size: f32,
    pub divisions: u32,
    pub color: Color,
    pub visible: bool,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            size: 100.0,
            divisions: 20,
            color: Color::srgba(0.5, 0.5, 0.5, 0.3),
            visible: true,
        }
    }
}
