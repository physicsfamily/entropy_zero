//! Ripple Tank Simulation - Professional Water Wave Physics Lab
//!
//! A comprehensive 2D water wave simulation featuring:
//! - FDTD wave equation solver
//! - Draggable wave sources (point, line, phased array, moving)
//! - Obstacles (reflectors, slits, refraction media)
//! - Measurement tools (oscilloscope probes, rulers)
//! - Real-time data visualization

mod components;
mod physics;
mod resources;
mod spawn;
mod ui;

pub use components::*;
pub use physics::*;
pub use resources::*;
pub use spawn::*;
pub use ui::*;

use bevy::prelude::*;
use ez_core::prelude::*;

// ══════════════════════════════════════════════════════════════════════════════
// Constants
// ══════════════════════════════════════════════════════════════════════════════

pub const GRID_SIZE: usize = 256;
pub const GRID_SCALE: f32 = 2.0;
pub const MAX_PROBE_HISTORY: usize = 512;

// ══════════════════════════════════════════════════════════════════════════════
// Simulation Definition
// ══════════════════════════════════════════════════════════════════════════════

pub struct RippleTankSimulation;

impl Simulation for RippleTankSimulation {
    fn id(&self) -> &'static str {
        "ripple_tank"
    }

    fn name(&self) -> &'static str {
        "Ripple Tank"
    }

    fn category(&self) -> SimulationCategory {
        SimulationCategory::WavePhysics(WavePhysicsSubdomain::MechanicalWaves)
    }

    fn description(&self) -> &'static str {
        "Professional ripple tank simulation for wave physics experiments. \
         Features draggable wave sources, obstacles, slits for diffraction, \
         and oscilloscope probes for quantitative analysis."
    }

    fn parameters(&self) -> Vec<ParameterDef> {
        vec![
            ParameterDef::Float {
                id: "wave_speed",
                name: "Wave Speed",
                description: "Propagation speed of waves",
                min: 0.1,
                max: 5.0,
                default: 1.0,
                step: Some(0.1),
                unit: Some("m/s"),
            },
            ParameterDef::Float {
                id: "damping",
                name: "Damping",
                description: "Energy dissipation rate (1.0 = no damping)",
                min: 0.9,
                max: 1.0,
                default: 0.995,
                step: Some(0.001),
                unit: None,
            },
            ParameterDef::Float {
                id: "time_scale",
                name: "Time Scale",
                description: "Simulation speed multiplier",
                min: 0.1,
                max: 2.0,
                default: 1.0,
                step: Some(0.1),
                unit: None,
            },
            ParameterDef::Bool {
                id: "paused",
                name: "Paused",
                description: "Pause the simulation",
                default: false,
            },
        ]
    }

    fn build_plugin(&self) -> Box<dyn Fn(&mut App) + Send + Sync> {
        Box::new(|app| {
            app.add_plugins(RippleTankPlugin);
        })
    }

    fn difficulty(&self) -> u8 {
        3
    }

    fn tags(&self) -> &'static [&'static str] {
        &["waves", "interference", "diffraction", "ripple", "huygen", "doppler"]
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Plugin
// ══════════════════════════════════════════════════════════════════════════════

pub struct RippleTankPlugin;

impl Plugin for RippleTankPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WaveField>()
            .init_resource::<RippleTankConfig>()
            .init_resource::<UIState>()
            .init_resource::<SimulationStats>()
            .init_resource::<ObjectIdCounter>()
            .register_type::<WaveSource>()
            .register_type::<Obstacle>()
            .register_type::<Probe>()
            .add_systems(Startup, setup_scene)
            .add_systems(
                Update,
                (
                    handle_keyboard_input,
                    handle_mouse_input,
                    update_moving_sources,
                    rasterize_obstacles,
                    apply_wave_sources,
                    update_wave_field,
                    update_probes,
                    update_wave_visualization,
                    update_stats,
                    render_top_bar_ui,
                    render_toolbox_ui,
                    render_inspector_ui,
                    render_data_panel_ui,
                ),
            );
    }
}
