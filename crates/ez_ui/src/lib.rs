//! # EZ UI
//!
//! Shared UI components for Entropy Zero simulations.
//!
//! Provides:
//! - Control panels with automatic parameter binding
//! - Real-time plotting
//! - Common widgets (sliders, toggles, etc.)

pub mod panels;
pub mod widgets;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

/// Plugin that provides shared UI infrastructure.
pub struct EzUiPlugin;

impl Plugin for EzUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
    }
}

/// Prelude for convenient imports.
pub mod prelude {
    pub use crate::panels::*;
    pub use crate::widgets::*;
    pub use crate::EzUiPlugin;
}
