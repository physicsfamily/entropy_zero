//! # EZ Renderer
//!
//! Shared rendering infrastructure for Entropy Zero simulations.
//!
//! Provides:
//! - Camera controllers (orbit, pan, zoom)
//! - Grid and axis visualization
//! - Gizmos (vectors, arrows, coordinate frames)
//! - Common materials and shaders

pub mod camera;
pub mod grid;
pub mod materials;

use bevy::prelude::*;

/// Plugin that provides shared rendering utilities.
pub struct EzRendererPlugin;

impl Plugin for EzRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(camera::CameraControllerPlugin)
            .add_plugins(grid::GridPlugin);
    }
}

/// Prelude for convenient imports.
pub mod prelude {
    pub use crate::camera::*;
    pub use crate::grid::*;
    pub use crate::materials::*;
    pub use crate::EzRendererPlugin;
}
