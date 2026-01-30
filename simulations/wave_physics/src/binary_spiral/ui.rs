//! UI for the Binary Spiral simulation

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::resources::*;
use super::ParticlePool;

// ══════════════════════════════════════════════════════════════════════════════
// UI Rendering
// ══════════════════════════════════════════════════════════════════════════════

pub fn render_ui(
    mut contexts: EguiContexts,
    mut config: ResMut<BinarySpiralConfig>,
    pool: Res<ParticlePool>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("🌀 Binary Spiral")
        .default_pos([20.0, 20.0])
        .default_width(320.0)
        .resizable(false)
        .show(ctx, |ui| {
            ui.style_mut().spacing.slider_width = 180.0;

            // Instructions
            ui.group(|ui| {
                ui.colored_label(egui::Color32::from_rgb(255, 221, 0), "📖 Instructions");
                ui.label("• Brightness = Particle Density");
                ui.label("• Drag stars to change orbit radius");
                ui.label("• Use mouse to rotate view");
            });

            ui.add_space(10.0);

            // Simulation controls
            ui.horizontal(|ui| {
                if ui.button(if config.paused { "▶ Play" } else { "⏸ Pause" }).clicked() {
                    config.paused = !config.paused;
                }
            });

            ui.add_space(10.0);
            ui.separator();

            // Parameters
            ui.heading("Parameters");

            ui.add_space(5.0);

            // Orbit Speed
            ui.horizontal(|ui| {
                ui.label("Orbit Speed");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{:.1}", config.orbit_speed));
                });
            });
            ui.add(egui::Slider::new(&mut config.orbit_speed, 0.1..=4.0).show_value(false));

            ui.add_space(5.0);

            // Emission Rate
            let mut emission_f32 = config.emission_rate as f32;
            ui.horizontal(|ui| {
                ui.label("Emission Rate");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{}", config.emission_rate));
                });
            });
            if ui
                .add(egui::Slider::new(&mut emission_f32, 200.0..=2000.0).show_value(false))
                .changed()
            {
                config.emission_rate = emission_f32 as usize;
            }

            ui.add_space(5.0);

            // Particle Speed
            ui.horizontal(|ui| {
                ui.label("Particle Speed");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{:.1}", config.particle_speed));
                });
            });
            ui.add(egui::Slider::new(&mut config.particle_speed, 1.0..=5.0).show_value(false));

            ui.add_space(10.0);
            ui.separator();

            // Display options
            ui.heading("Display");
            ui.checkbox(&mut config.show_grid, "Show Grid");
            ui.checkbox(&mut config.show_orbit_ring, "Show Orbit Ring");

            ui.add_space(10.0);
            ui.separator();

            // Statistics
            ui.heading("Statistics");
            let active_count = pool.particles.iter().filter(|p| p.active).count();
            ui.label(format!("Active Particles: {}", active_count));

            ui.add_space(10.0);

            // Legend
            ui.group(|ui| {
                ui.label("Color Legend:");
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(12.0, 12.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().circle_filled(
                        rect.center(),
                        5.0,
                        egui::Color32::from_rgb(0, 255, 255),
                    );
                    ui.label("Leading / High Density");
                });
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(12.0, 12.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().circle_filled(
                        rect.center(),
                        5.0,
                        egui::Color32::from_rgb(255, 0, 85),
                    );
                    ui.label("Trailing / Low Density");
                });
            });
        });
}
