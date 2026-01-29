use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod components;
mod plugins;
mod resources;
mod systems;

use plugins::particle_plugin::ParticlePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Entropy Zero - 1M Particle Simulation".into(),
                canvas: Some("#canvas".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(ParticlePlugin)
        .run();
}
