//! # Entropy Zero Web Application
//!
//! Main entry point for the web-based simulation platform.
//! 
//! ## Debugging in Browser
//! 
//! - Press F12 to open DevTools -> Console to see Rust logs
//! - All `info!()`, `warn!()`, `error!()` macros output here
//! - Rust panics are automatically printed to the console

use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy_egui::EguiPlugin;
use classical_mechanics::ClassicalMechanicsPlugin;

fn main() {
    // ═══════════════════════════════════════════════════════════════════
    // WASM Debugging Setup
    // ═══════════════════════════════════════════════════════════════════
    
    // Map Rust panics to browser console (instead of silent crashes)
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    // ═══════════════════════════════════════════════════════════════════
    // Application
    // ═══════════════════════════════════════════════════════════════════

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Entropy Zero - Scientific Simulation Platform".into(),
                        canvas: Some("#canvas".into()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    // Log levels: TRACE < DEBUG < INFO < WARN < ERROR
                    level: bevy::log::Level::DEBUG,
                    // Filter noisy modules, keep our code verbose
                    filter: "wgpu=error,wgpu_core=error,wgpu_hal=error,\
                             naga=warn,bevy_render=info,bevy_ecs=warn,\
                             entropy_zero=debug,classical_mechanics=debug,\
                             ez_core=debug,ez_physics=debug".to_string(),
                    ..default()
                }),
        )
        .add_plugins(EguiPlugin)
        .add_plugins(ClassicalMechanicsPlugin)
        // Add debug startup message
        .add_systems(Startup, log_startup_info)
        .run();
}

/// Log startup information to browser console.
fn log_startup_info() {
    info!("╔════════════════════════════════════════════════════════════╗");
    info!("║     Entropy Zero - Scientific Simulation Platform          ║");
    info!("║                                                            ║");
    info!("║  Press F12 to open DevTools and see this console           ║");
    info!("║  Use egui panels to adjust parameters in real-time         ║");
    info!("╚════════════════════════════════════════════════════════════╝");
    debug!("Debug logging is enabled. You'll see detailed physics info here.");
}
