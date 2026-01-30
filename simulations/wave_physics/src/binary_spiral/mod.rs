//! Binary Spiral Simulation - Double Star Particle Emission
//!
//! A 3D particle density field visualization featuring:
//! - Two orbiting point sources (binary stars)
//! - Spherical particle emission with velocity-based coloring
//! - Additive blending for density visualization
//! - Interactive dragging to adjust orbit radius

mod components;
mod resources;
mod systems;
mod ui;

pub use components::*;
pub use resources::*;
pub use systems::*;
pub use ui::*;

use bevy::prelude::*;
use ez_core::prelude::*;

// ══════════════════════════════════════════════════════════════════════════════
// Constants
// ══════════════════════════════════════════════════════════════════════════════

pub const MAX_PARTICLES: usize = 200_000;
pub const DEFAULT_ORBIT_RADIUS: f32 = 20.0;
pub const DEFAULT_EMISSION_RATE: usize = 1000;
pub const DEFAULT_PARTICLE_LIFE: u32 = 300;

// ══════════════════════════════════════════════════════════════════════════════
// Simulation Definition
// ══════════════════════════════════════════════════════════════════════════════

pub struct BinarySpiralSimulation;

impl Simulation for BinarySpiralSimulation {
    fn id(&self) -> &'static str {
        "binary_spiral"
    }

    fn name(&self) -> &'static str {
        "Binary Spiral"
    }

    fn category(&self) -> SimulationCategory {
        SimulationCategory::RelativisticPhysics(RelativisticSubdomain::BlackHoles)
    }

    fn description(&self) -> &'static str {
        "Binary star system with particle emission. Two orbiting sources emit particles \
         in all directions, creating spiral density patterns. Brightness represents \
         particle density through additive blending. Drag the stars to change orbit radius."
    }

    fn parameters(&self) -> Vec<ParameterDef> {
        vec![
            ParameterDef::Float {
                id: "orbit_speed",
                name: "Orbit Speed",
                description: "Angular velocity of the orbiting sources",
                min: 0.1,
                max: 4.0,
                default: 1.5,
                step: Some(0.1),
                unit: Some("rad/s"),
            },
            ParameterDef::Float {
                id: "emission_rate",
                name: "Emission Rate",
                description: "Particles emitted per source per frame",
                min: 200.0,
                max: 2000.0,
                default: 1000.0,
                step: Some(100.0),
                unit: None,
            },
            ParameterDef::Float {
                id: "particle_speed",
                name: "Particle Speed",
                description: "Outward flow speed of particles",
                min: 1.0,
                max: 5.0,
                default: 2.0,
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
            app.add_plugins(BinarySpiralPlugin);
        })
    }

    fn difficulty(&self) -> u8 {
        2
    }

    fn tags(&self) -> &'static [&'static str] {
        &["particles", "binary", "spiral", "density", "stars", "orbital"]
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Plugin
// ══════════════════════════════════════════════════════════════════════════════

pub struct BinarySpiralPlugin;

impl Plugin for BinarySpiralPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BinarySpiralConfig>()
            .init_resource::<ParticlePool>()
            .init_resource::<DragState>()
            .init_resource::<RandomDirections>()
            .register_type::<OrbitalSource>()
            .add_systems(Startup, setup_scene)
            .add_systems(
                Update,
                (
                    handle_mouse_input,
                    update_orbital_sources,
                    emit_particles,
                    update_particles,
                    sync_particle_mesh,
                    render_ui,
                ),
            );
    }
}
