use bevy::prelude::*;
use rand::Rng;

use crate::components::particle::{Particle, ParticleTag};
use crate::resources::simulation::{SimulationConfig, SimulationStats};
use crate::systems::particle_systems::{update_particles, update_stats};
use crate::systems::ui_systems::render_ui;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationConfig>()
            .init_resource::<SimulationStats>()
            .add_systems(Startup, setup_particle_scene)
            .add_systems(Update, (update_particles, update_stats, render_ui));
    }
}

fn setup_particle_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<SimulationConfig>,
) {
    let mut rng = rand::thread_rng();
    let bounds = config.bounds;

    // Create shared mesh and material for instancing
    let mesh = meshes.add(Sphere::new(0.05).mesh().ico(1).expect("Failed to create sphere mesh"));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.7, 1.0),
        emissive: Color::srgb(0.1, 0.3, 0.5).into(),
        ..default()
    });

    // Spawn particles in batches for better performance
    let particle_count = config.particle_count;
    
    for _ in 0..particle_count {
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
