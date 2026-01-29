use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::resources::simulation::{SimulationConfig, SimulationStats};

pub fn render_ui(
    mut contexts: EguiContexts,
    mut config: ResMut<SimulationConfig>,
    stats: Res<SimulationStats>,
) {
    egui::Window::new("Simulation Controls")
        .default_pos([10.0, 10.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Statistics");
            ui.label(format!("FPS: {:.1}", stats.fps));
            ui.label(format!("Particles: {}", stats.particle_count));

            ui.separator();

            ui.heading("Controls");
            if ui.button(if config.paused { "Resume" } else { "Pause" }).clicked() {
                config.paused = !config.paused;
            }

            ui.add(egui::Slider::new(&mut config.speed_multiplier, 0.1..=5.0).text("Speed"));
            ui.add(egui::Slider::new(&mut config.gravity.y, -20.0..=0.0).text("Gravity"));
            ui.add(egui::Slider::new(&mut config.bounds, 10.0..=100.0).text("Bounds"));
        });
}
