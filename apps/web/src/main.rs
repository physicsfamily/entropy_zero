//! # Entropy Zero Web Application
//!
//! Main entry point for the web-based simulation platform.

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use classical_mechanics::ClassicalMechanicsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Entropy Zero - Scientific Simulation Platform".into(),
                canvas: Some("#canvas".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(ClassicalMechanicsPlugin)
        .run();
}
