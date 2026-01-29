//! Common UI widgets.

use bevy_egui::egui;

/// A styled play/pause button.
pub fn play_pause_button(ui: &mut egui::Ui, paused: &mut bool) -> bool {
    let label = if *paused { "▶ Play" } else { "⏸ Pause" };
    if ui.button(label).clicked() {
        *paused = !*paused;
        true
    } else {
        false
    }
}

/// A styled reset button.
pub fn reset_button(ui: &mut egui::Ui) -> bool {
    ui.button("↺ Reset").clicked()
}

/// Display FPS counter.
pub fn fps_display(ui: &mut egui::Ui, fps: f32) {
    ui.label(format!("FPS: {:.0}", fps));
}

/// Display entity/particle count.
pub fn entity_count_display(ui: &mut egui::Ui, count: usize, label: &str) {
    ui.label(format!("{}: {}", label, count));
}
