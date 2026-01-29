//! Particle System Simulation
//!
//! A high-performance particle simulation demonstrating gravity,
//! boundary collisions, and parallel processing.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use ez_core::prelude::*;
use rand::Rng;

// ══════════════════════════════════════════════════════════════════════════════
// Simulation Definition
// ══════════════════════════════════════════════════════════════════════════════

/// The particle system simulation metadata.
pub struct ParticleSystemSimulation;

impl Simulation for ParticleSystemSimulation {
    fn id(&self) -> &'static str {
        "particle_system"
    }

    fn name(&self) -> &'static str {
        "Particle System"
    }

    fn category(&self) -> SimulationCategory {
        SimulationCategory::ClassicalMechanics(ClassicalMechanicsSubdomain::Dynamics)
    }

    fn description(&self) -> &'static str {
        "High-performance particle simulation with gravity and boundary collisions. \
         Demonstrates parallel processing for large entity counts."
    }

    fn parameters(&self) -> Vec<ParameterDef> {
        vec![
            ParameterDef::Int {
                id: "particle_count",
                name: "Particle Count",
                description: "Number of particles to simulate",
                min: 100,
                max: 1_000_000,
                default: 100_000,
            },
            ParameterDef::Float {
                id: "gravity",
                name: "Gravity",
                description: "Gravitational acceleration",
                min: 0.0,
                max: 30.0,
                default: 9.8,
                step: Some(0.1),
                unit: Some("m/s²"),
            },
            ParameterDef::Float {
                id: "bounds",
                name: "Bounds",
                description: "Size of the simulation boundary",
                min: 10.0,
                max: 200.0,
                default: 50.0,
                step: Some(1.0),
                unit: Some("m"),
            },
            ParameterDef::Float {
                id: "speed",
                name: "Speed Multiplier",
                description: "Time scale for simulation",
                min: 0.1,
                max: 5.0,
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
            app.add_plugins(ParticleSystemPlugin);
        })
    }

    fn difficulty(&self) -> u8 {
        2
    }

    fn tags(&self) -> &'static [&'static str] {
        &["particles", "gravity", "collision", "performance"]
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Components
// ══════════════════════════════════════════════════════════════════════════════

/// Velocity component for particles.
#[derive(Component, Default, Reflect)]
pub struct Particle {
    pub velocity: Vec3,
}

/// Tag component for particle entities.
#[derive(Component)]
pub struct ParticleTag;

// ══════════════════════════════════════════════════════════════════════════════
// Resources
// ══════════════════════════════════════════════════════════════════════════════

/// Configuration for the particle simulation.
#[derive(Resource)]
pub struct ParticleConfig {
    pub particle_count: usize,
    pub gravity: Vec3,
    pub bounds: f32,
    pub speed_multiplier: f32,
    pub paused: bool,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            particle_count: 100_000,
            gravity: Vec3::new(0.0, -9.8, 0.0),
            bounds: 50.0,
            speed_multiplier: 1.0,
            paused: false,
        }
    }
}

/// Runtime statistics.
#[derive(Resource, Default)]
pub struct ParticleStats {
    pub fps: f32,
    pub particle_count: usize,
}

// ══════════════════════════════════════════════════════════════════════════════
// Plugin
// ══════════════════════════════════════════════════════════════════════════════

/// Bevy plugin for the particle system simulation.
pub struct ParticleSystemPlugin;

impl Plugin for ParticleSystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ParticleConfig>()
            .init_resource::<ParticleStats>()
            .add_systems(Startup, setup_particle_scene)
            .add_systems(Update, (update_particles, update_stats, render_ui));
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Systems
// ══════════════════════════════════════════════════════════════════════════════

fn setup_particle_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<ParticleConfig>,
) {
    let mut rng = rand::thread_rng();
    let bounds = config.bounds;

    // Create shared mesh and material for instancing
    let mesh = meshes.add(
        Sphere::new(0.05)
            .mesh()
            .ico(1)
            .expect("Failed to create sphere mesh"),
    );
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.7, 1.0),
        emissive: Color::srgb(0.1, 0.3, 0.5).into(),
        ..default()
    });

    // Spawn particles
    for _ in 0..config.particle_count {
        let position = Vec3::new(
            rng.gen_range(-bounds..bounds),
            rng.gen_range(0.0..bounds * 2.0),
            rng.gen_range(-bounds..bounds),
        );

        let velocity = Vec3::new(
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-5.0..15.0),
            rng.gen_range(-10.0..10.0),
        );

        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
            Particle { velocity },
            ParticleTag,
        ));
    }

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 50.0, 150.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
    });

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_particles(
    mut query: Query<(&mut Transform, &mut Particle)>,
    config: Res<ParticleConfig>,
    time: Res<Time>,
) {
    if config.paused {
        return;
    }

    let dt = time.delta_seconds() * config.speed_multiplier;
    let bounds = config.bounds;
    let gravity = config.gravity;

    query
        .par_iter_mut()
        .for_each(|(mut transform, mut particle)| {
            particle.velocity += gravity * dt;
            transform.translation += particle.velocity * dt;

            // Bounce off bounds
            if transform.translation.x.abs() > bounds {
                transform.translation.x = transform.translation.x.signum() * bounds;
                particle.velocity.x *= -0.8;
            }
            if transform.translation.y < -bounds {
                transform.translation.y = -bounds;
                particle.velocity.y *= -0.8;
            }
            if transform.translation.y > bounds {
                transform.translation.y = bounds;
                particle.velocity.y *= -0.8;
            }
            if transform.translation.z.abs() > bounds {
                transform.translation.z = transform.translation.z.signum() * bounds;
                particle.velocity.z *= -0.8;
            }
        });
}

fn update_stats(query: Query<&Particle>, mut stats: ResMut<ParticleStats>, time: Res<Time>) {
    stats.fps = 1.0 / time.delta_seconds();
    stats.particle_count = query.iter().count();
}

fn render_ui(
    mut contexts: EguiContexts,
    mut config: ResMut<ParticleConfig>,
    stats: Res<ParticleStats>,
) {
    egui::Window::new("Particle System Controls").show(contexts.ctx_mut(), |ui| {
        ui.heading("Statistics");
        ui.label(format!("FPS: {:.0}", stats.fps));
        ui.label(format!("Particles: {}", stats.particle_count));

        ui.separator();
        ui.heading("Controls");

        if ui
            .button(if config.paused { "▶ Play" } else { "⏸ Pause" })
            .clicked()
        {
            config.paused = !config.paused;
        }

        ui.add(egui::Slider::new(&mut config.speed_multiplier, 0.1..=5.0).text("Speed"));

        let mut gravity_y = -config.gravity.y;
        ui.add(egui::Slider::new(&mut gravity_y, 0.0..=30.0).text("Gravity (m/s²)"));
        config.gravity.y = -gravity_y;

        ui.add(egui::Slider::new(&mut config.bounds, 10.0..=200.0).text("Bounds (m)"));
    });
}
