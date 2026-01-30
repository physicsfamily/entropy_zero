//! UI systems for the Ripple Tank simulation

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::f32::consts::PI;

use super::{components::*, resources::*};

// ══════════════════════════════════════════════════════════════════════════════
// Top Bar
// ══════════════════════════════════════════════════════════════════════════════

pub fn render_top_bar_ui(
    mut contexts: EguiContexts,
    mut config: ResMut<RippleTankConfig>,
    mut wave_field: ResMut<WaveField>,
    stats: Res<SimulationStats>,
) {
    egui::TopBottomPanel::top("top_bar").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.heading("🌊 Ripple Tank");
            ui.separator();

            if ui.button(if config.paused { "▶ Play" } else { "⏸ Pause" }).clicked() {
                config.paused = !config.paused;
            }

            ui.label("Speed:");
            ui.add(egui::Slider::new(&mut config.time_scale, 0.1..=2.0).show_value(false));

            if ui.button("🗑 Clear Waves").clicked() {
                wave_field.clear();
            }

            ui.separator();
            ui.label(format!("FPS: {:.0}", stats.fps));
            ui.label(format!("t = {:.2}s", stats.simulation_time));
        });
    });
}

// ══════════════════════════════════════════════════════════════════════════════
// Toolbox (Left Panel)
// ══════════════════════════════════════════════════════════════════════════════

pub fn render_toolbox_ui(mut contexts: EguiContexts, mut ui_state: ResMut<UIState>) {
    egui::SidePanel::left("toolbox").default_width(super::TOOLBOX_PANEL_WIDTH).show(contexts.ctx_mut(), |ui| {
        ui.heading("🧰 Toolbox");
        ui.separator();

        if ui.selectable_label(ui_state.selected_tool == ToolType::Select, "🖱 Select").clicked() {
            ui_state.selected_tool = ToolType::Select;
        }

        ui.separator();
        ui.label("📡 Wave Sources");

        if ui.selectable_label(ui_state.selected_tool == ToolType::PointSource, "  • Point Source").clicked() {
            ui_state.selected_tool = ToolType::PointSource;
        }
        if ui.selectable_label(ui_state.selected_tool == ToolType::LineSource, "  ━ Line Source").clicked() {
            ui_state.selected_tool = ToolType::LineSource;
        }
        if ui.selectable_label(ui_state.selected_tool == ToolType::PhasedArray, "  ⋯ Phased Array").clicked() {
            ui_state.selected_tool = ToolType::PhasedArray;
        }
        if ui.selectable_label(ui_state.selected_tool == ToolType::MovingSource, "  🚤 Moving Source").clicked() {
            ui_state.selected_tool = ToolType::MovingSource;
        }

        ui.separator();
        ui.label("🧱 Obstacles");

        if ui.selectable_label(ui_state.selected_tool == ToolType::Reflector, "  ▬ Reflector").clicked() {
            ui_state.selected_tool = ToolType::Reflector;
        }
        if ui.selectable_label(ui_state.selected_tool == ToolType::SingleSlit, "  ╠ Single Slit").clicked() {
            ui_state.selected_tool = ToolType::SingleSlit;
        }
        if ui.selectable_label(ui_state.selected_tool == ToolType::DoubleSlit, "  ╬ Double Slit").clicked() {
            ui_state.selected_tool = ToolType::DoubleSlit;
        }
        if ui.selectable_label(ui_state.selected_tool == ToolType::RefractionBlock, "  ▢ Refraction Block").clicked() {
            ui_state.selected_tool = ToolType::RefractionBlock;
        }

        ui.separator();
        ui.label("📏 Measurement");

        if ui.selectable_label(ui_state.selected_tool == ToolType::Probe, "  ◉ Scope Probe").clicked() {
            ui_state.selected_tool = ToolType::Probe;
        }
        if ui.selectable_label(ui_state.selected_tool == ToolType::Ruler, "  📐 Ruler").clicked() {
            ui_state.selected_tool = ToolType::Ruler;
        }

        ui.separator();
        ui.small("Click viewport to place");
    });
}

// ══════════════════════════════════════════════════════════════════════════════
// Inspector (Right Panel)
// ══════════════════════════════════════════════════════════════════════════════

pub fn render_inspector_ui(
    mut contexts: EguiContexts,
    ui_state: Res<UIState>,
    mut config: ResMut<RippleTankConfig>,
    mut sources: Query<(&SceneObject, &mut WaveSource)>,
    mut obstacles: Query<(&SceneObject, &mut Obstacle), Without<WaveSource>>,
    mut moving: Query<(&SceneObject, &mut MovingSource)>,
    mut commands: Commands,
) {
    egui::SidePanel::right("inspector").default_width(super::INSPECTOR_PANEL_WIDTH).show(contexts.ctx_mut(), |ui| {
        ui.heading("🔧 Inspector");
        ui.separator();

        if let Some(entity) = ui_state.selected_entity {
            if let Ok((obj, mut source)) = sources.get_mut(entity) {
                ui.label(format!("Wave Source #{}", obj.id));
                ui.separator();

                ui.checkbox(&mut source.enabled, "Enabled");
                ui.add(egui::Slider::new(&mut source.frequency, 0.5..=10.0).text("Frequency (Hz)"));
                ui.add(egui::Slider::new(&mut source.amplitude, 0.1..=2.0).text("Amplitude"));
                ui.add(egui::Slider::new(&mut source.phase, 0.0..=2.0 * PI).text("Phase (rad)"));

                ui.horizontal(|ui| {
                    ui.label("Waveform:");
                    egui::ComboBox::from_id_source("waveform")
                        .selected_text(format!("{:?}", source.waveform))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut source.waveform, Waveform::Sine, "Sine");
                            ui.selectable_value(&mut source.waveform, Waveform::Square, "Square");
                            ui.selectable_value(&mut source.waveform, Waveform::Pulse, "Pulse");
                        });
                });

                if let Ok((_, mut mov)) = moving.get_mut(entity) {
                    ui.separator();
                    ui.label("Movement");
                    ui.add(egui::Slider::new(&mut mov.velocity.x, -100.0..=100.0).text("Vx"));
                    ui.add(egui::Slider::new(&mut mov.velocity.y, -100.0..=100.0).text("Vy"));
                }

                ui.separator();
                if ui.button("🗑 Delete").clicked() {
                    commands.entity(entity).despawn();
                }
            } else if let Ok((obj, mut obstacle)) = obstacles.get_mut(entity) {
                ui.label(format!("Obstacle #{}", obj.id));
                ui.separator();

                match obstacle.obstacle_type {
                    ObstacleType::Reflector => {
                        ui.label("Type: Reflector");
                        ui.add(egui::Slider::new(&mut obstacle.width, 10.0..=200.0).text("Width"));
                    }
                    ObstacleType::SingleSlit => {
                        ui.label("Type: Single Slit");
                        ui.add(egui::Slider::new(&mut obstacle.width, 50.0..=200.0).text("Width"));
                        ui.add(egui::Slider::new(&mut obstacle.slit_width, 5.0..=50.0).text("Slit Width"));
                    }
                    ObstacleType::DoubleSlit => {
                        ui.label("Type: Double Slit");
                        ui.add(egui::Slider::new(&mut obstacle.width, 50.0..=200.0).text("Width"));
                        ui.add(egui::Slider::new(&mut obstacle.slit_width, 5.0..=30.0).text("Slit Width"));
                        ui.add(egui::Slider::new(&mut obstacle.slit_separation, 10.0..=80.0).text("Separation"));
                    }
                    ObstacleType::RefractionBlock => {
                        ui.label("Type: Refraction Block");
                        ui.add(egui::Slider::new(&mut obstacle.width, 20.0..=150.0).text("Width"));
                        ui.add(egui::Slider::new(&mut obstacle.height, 20.0..=150.0).text("Height"));
                        ui.add(egui::Slider::new(&mut obstacle.refractive_index, 1.0..=3.0).text("Refractive Index"));
                    }
                }

                ui.separator();
                if ui.button("🗑 Delete").clicked() {
                    commands.entity(entity).despawn();
                }
            }
        } else {
            ui.label("Global Settings");
            ui.separator();

            ui.add(egui::Slider::new(&mut config.wave_speed, 0.1..=5.0).text("Wave Speed"));
            ui.add(egui::Slider::new(&mut config.damping, 0.9..=1.0).text("Damping"));

            ui.separator();
            ui.label("Color Scheme:");
            egui::ComboBox::from_id_source("color_scheme")
                .selected_text(format!("{:?}", config.color_scheme))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut config.color_scheme, ColorScheme::DeepOcean, "Deep Ocean");
                    ui.selectable_value(&mut config.color_scheme, ColorScheme::Scientific, "Scientific");
                    ui.selectable_value(&mut config.color_scheme, ColorScheme::PhaseColor, "Phase Color");
                    ui.selectable_value(&mut config.color_scheme, ColorScheme::Grayscale, "Grayscale");
                });

            ui.separator();
            ui.small("Select an object to edit");
        }
    });
}

// ══════════════════════════════════════════════════════════════════════════════
// Data Panel (Bottom)
// ══════════════════════════════════════════════════════════════════════════════

pub fn render_data_panel_ui(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UIState>,
    stats: Res<SimulationStats>,
    probes: Query<&Probe>,
    rulers: Query<(&Transform, &Ruler)>,
) {
    egui::TopBottomPanel::bottom("data_panel")
        .default_height(super::DATA_PANEL_HEIGHT)
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.heading("📊 Data Lab");
                ui.checkbox(&mut ui_state.show_data_panel, "Expand");
            });

            if !ui_state.show_data_panel {
                return;
            }

            ui.separator();

            ui.columns(2, |columns| {
                // Oscilloscope view
                columns[0].heading("Oscilloscope");

                let probe_vec: Vec<&Probe> = probes.iter().collect();
                if probe_vec.is_empty() {
                    columns[0].label("No probes placed. Add probes from toolbox.");
                } else {
                    for probe in &probe_vec {
                        let current = probe.history.last().copied().unwrap_or(0.0);
                        let min = probe.history.iter().copied().fold(f32::INFINITY, f32::min);
                        let max = probe.history.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                        
                        let c = probe.color.to_srgba();
                        let color = egui::Color32::from_rgb(
                            (c.red * 255.0) as u8,
                            (c.green * 255.0) as u8,
                            (c.blue * 255.0) as u8,
                        );
                        
                        columns[0].horizontal(|ui| {
                            ui.colored_label(color, format!("{}:", probe.label));
                            ui.label(format!("{:+.3}", current));
                            ui.small(format!("[{:.2}, {:.2}]", min, max));
                        });

                        // Simple ASCII waveform display
                        let width = 40;
                        let start = probe.history.len().saturating_sub(width);
                        let wave: String = probe.history[start..].iter()
                            .map(|&v| {
                                let normalized = ((v + 2.0) / 4.0).clamp(0.0, 1.0);
                                let idx = (normalized * 4.0) as usize;
                                ['▁', '▂', '▄', '▆', '█'][idx.min(4)]
                            })
                            .collect();
                        columns[0].monospace(wave);
                    }

                    if let Some(phase_diff) = stats.probe_phase_diff {
                        columns[0].label(format!("Phase Difference: {:.2} rad", phase_diff));
                    }
                }

                // Measurements view
                columns[1].heading("Measurements");

                columns[1].label(format!("Wave Energy: {:.2}", stats.wave_energy));

                for (transform, ruler) in rulers.iter() {
                    let length = (ruler.end - ruler.start).length();
                    let pos = transform.translation.truncate();
                    columns[1].label(format!(
                        "Ruler at ({:.0}, {:.0}): {:.1} units",
                        pos.x, pos.y, length
                    ));
                }

                if rulers.is_empty() {
                    columns[1].small("Add rulers to measure wavelength");
                }
            });
        });
}
