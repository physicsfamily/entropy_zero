//! Systems for the Binary Spiral simulation

use bevy::prelude::*;
use bevy::render::mesh::{PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;
use rand::Rng;

use super::components::*;
use super::resources::*;
use super::{DEFAULT_ORBIT_RADIUS, MAX_PARTICLES};

// ══════════════════════════════════════════════════════════════════════════════
// Colors
// ══════════════════════════════════════════════════════════════════════════════

const COLOR_SOURCE_A: Color = Color::srgb(0.67, 0.0, 1.0);    // Purple (#aa00ff)
const COLOR_SOURCE_B: Color = Color::srgb(1.0, 0.67, 0.0);    // Orange (#ffaa00)
const COLOR_FRONT: [f32; 3] = [0.0, 1.0, 1.0];                 // Cyan (#00ffff)
const COLOR_BACK: [f32; 3] = [1.0, 0.0, 0.33];                 // Red-pink (#ff0055)

// ══════════════════════════════════════════════════════════════════════════════
// Setup
// ══════════════════════════════════════════════════════════════════════════════

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera - positioned above looking down at an angle
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 80.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });

    // Grid floor
    let grid_size = 300.0;
    let grid_divisions = 60;
    let grid_mesh = create_grid_mesh(grid_size, grid_divisions);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(grid_mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::srgba(0.1, 0.1, 0.1, 0.5),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -0.1, 0.0),
            ..default()
        },
        GridFloor,
    ));

    // Orbit ring
    let ring_mesh = create_ring_mesh(DEFAULT_ORBIT_RADIUS, 64);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(ring_mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::srgba(0.2, 0.2, 0.2, 0.3),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            ..default()
        },
        OrbitRing,
    ));

    // Source A (Purple)
    spawn_orbital_source(
        &mut commands,
        &mut meshes,
        &mut materials,
        0,
        0.0,
        COLOR_SOURCE_A,
    );

    // Source B (Orange) - opposite side
    spawn_orbital_source(
        &mut commands,
        &mut meshes,
        &mut materials,
        1,
        std::f32::consts::PI,
        COLOR_SOURCE_B,
    );

    // Particle point cloud
    let particle_mesh = create_particle_mesh();
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(particle_mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                alpha_mode: AlphaMode::Add,
                ..default()
            }),
            ..default()
        },
        ParticleCloud,
    ));

    info!("Binary Spiral simulation initialized");
}

fn spawn_orbital_source(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    index: usize,
    angle: f32,
    color: Color,
) {
    let source = OrbitalSource::new(index, angle, color, DEFAULT_ORBIT_RADIUS);
    let pos = source.current_position();

    // Main sphere
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(2.5)),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    emissive: color.into(),
                    ..default()
                }),
                transform: Transform::from_translation(pos),
                ..default()
            },
            source,
        ))
        .with_children(|parent| {
            // Glow sphere (larger, transparent)
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Sphere::new(4.5)),
                    material: materials.add(StandardMaterial {
                        base_color: color.with_alpha(0.4),
                        emissive: color.with_alpha(0.5).into(),
                        alpha_mode: AlphaMode::Add,
                        cull_mode: None,
                        ..default()
                    }),
                    ..default()
                },
                SourceGlow,
            ));
        });
}

fn create_grid_mesh(size: f32, divisions: u32) -> Mesh {
    let mut positions = Vec::new();
    let half = size / 2.0;
    let step = size / divisions as f32;

    for i in 0..=divisions {
        let offset = -half + step * i as f32;
        // Horizontal lines
        positions.push([offset, 0.0, -half]);
        positions.push([offset, 0.0, half]);
        // Vertical lines
        positions.push([-half, 0.0, offset]);
        positions.push([half, 0.0, offset]);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::LineList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh
}

fn create_ring_mesh(radius: f32, segments: u32) -> Mesh {
    let mut positions = Vec::new();

    for i in 0..segments {
        let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
        positions.push([angle1.cos() * radius, 0.0, angle1.sin() * radius]);
        positions.push([angle2.cos() * radius, 0.0, angle2.sin() * radius]);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::LineList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh
}

fn create_particle_mesh() -> Mesh {
    let positions: Vec<[f32; 3]> = vec![[99999.0, 99999.0, 99999.0]; MAX_PARTICLES];
    let colors: Vec<[f32; 4]> = vec![[1.0, 1.0, 1.0, 0.8]; MAX_PARTICLES];

    let mut mesh = Mesh::new(
        PrimitiveTopology::PointList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh
}

// ══════════════════════════════════════════════════════════════════════════════
// Mouse Input
// ══════════════════════════════════════════════════════════════════════════════

pub fn handle_mouse_input(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    sources: Query<(&OrbitalSource, &GlobalTransform)>,
    mut drag_state: ResMut<DragState>,
) {
    let Ok(window) = windows.get_single() else { return };
    let Ok((camera, camera_transform)) = cameras.get_single() else { return };
    let Some(cursor_pos) = window.cursor_position() else { return };

    // Ray from camera through cursor
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else { return };

    // Intersect with Y=0 plane for drag target
    let t = -ray.origin.y / ray.direction.y;
    if t > 0.0 {
        drag_state.drag_target = ray.origin + ray.direction * t;
    }

    if buttons.just_pressed(MouseButton::Left) {
        // Check if clicking on a source
        for (source, transform) in sources.iter() {
            let source_pos = transform.translation();
            let to_source = source_pos - ray.origin;
            let closest = ray.origin + *ray.direction * to_source.dot(*ray.direction);
            let distance = (closest - source_pos).length();

            if distance < 8.0 {
                drag_state.dragging_source = Some(source.index);
                break;
            }
        }
    }

    if buttons.just_released(MouseButton::Left) {
        drag_state.dragging_source = None;
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Orbital Source Update
// ══════════════════════════════════════════════════════════════════════════════

pub fn update_orbital_sources(
    time: Res<Time>,
    config: Res<BinarySpiralConfig>,
    drag_state: Res<DragState>,
    mut sources: Query<(&mut OrbitalSource, &mut Transform)>,
    mut ring_query: Query<&mut Transform, (With<OrbitRing>, Without<OrbitalSource>)>,
) {
    if config.paused {
        return;
    }

    let dt = time.delta_seconds();
    let mut max_radius = DEFAULT_ORBIT_RADIUS;

    for (mut source, mut transform) in sources.iter_mut() {
        // Handle dragging
        if drag_state.dragging_source == Some(source.index) {
            let dist = (drag_state.drag_target.x.powi(2) + drag_state.drag_target.z.powi(2)).sqrt();
            source.radius += (dist - source.radius) * 0.2;
            source.radius = source.radius.max(5.0);
        }

        // Update angle
        source.angle += config.orbit_speed * dt;

        // Calculate new position
        let new_pos = source.current_position();

        // Calculate velocity from position change
        source.velocity = (new_pos - source.last_position) * 5.0;
        source.last_position = new_pos;

        // Update transform
        transform.translation = new_pos;

        max_radius = max_radius.max(source.radius);
    }

    // Update orbit ring scale
    if let Ok(mut ring_transform) = ring_query.get_single_mut() {
        let scale = max_radius / DEFAULT_ORBIT_RADIUS;
        ring_transform.scale = Vec3::splat(scale);
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Particle Emission
// ══════════════════════════════════════════════════════════════════════════════

pub fn emit_particles(
    config: Res<BinarySpiralConfig>,
    sources: Query<&OrbitalSource>,
    random_dirs: Res<RandomDirections>,
    mut pool: ResMut<ParticlePool>,
) {
    if config.paused {
        return;
    }

    let mut rng = rand::thread_rng();

    for source in sources.iter() {
        let vel_dir = source.velocity.normalize_or_zero();
        let speed = source.velocity.length();
        let intensity = (speed / 3.0).min(1.0);

        let base_r = source.base_color.to_srgba().red;
        let base_g = source.base_color.to_srgba().green;
        let base_b = source.base_color.to_srgba().blue;

        for _ in 0..config.emission_rate {
            // Spherical jitter for initial position
            let jitter_dir = random_dirs.get(&mut rng);
            let jitter_r: f32 = rng.gen_range(0.0..1.5);
            let jitter = jitter_dir * jitter_r;

            let pos = source.current_position() + jitter;

            // Random direction for velocity
            let dir = random_dirs.get(&mut rng);
            let vel = dir * config.particle_speed;

            // Color based on alignment with source velocity
            let alignment = dir.dot(vel_dir);
            let mut color = [base_r, base_g, base_b];

            if alignment > 0.0 {
                // Lerp towards cyan (front)
                let t = alignment * intensity;
                color[0] = lerp(color[0], COLOR_FRONT[0], t);
                color[1] = lerp(color[1], COLOR_FRONT[1], t);
                color[2] = lerp(color[2], COLOR_FRONT[2], t);
            } else {
                // Lerp towards red-pink (back)
                let t = -alignment * intensity;
                color[0] = lerp(color[0], COLOR_BACK[0], t);
                color[1] = lerp(color[1], COLOR_BACK[1], t);
                color[2] = lerp(color[2], COLOR_BACK[2], t);
            }

            pool.emit(pos, vel, color, config.particle_life);
        }
    }
}

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

// ══════════════════════════════════════════════════════════════════════════════
// Particle Physics Update
// ══════════════════════════════════════════════════════════════════════════════

pub fn update_particles(config: Res<BinarySpiralConfig>, mut pool: ResMut<ParticlePool>) {
    if config.paused {
        return;
    }

    for p in pool.particles.iter_mut() {
        if p.active {
            p.position += p.velocity;
            p.life = p.life.saturating_sub(1);
            if p.life == 0 {
                p.active = false;
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Sync Particle Mesh
// ══════════════════════════════════════════════════════════════════════════════

pub fn sync_particle_mesh(
    pool: Res<ParticlePool>,
    mut meshes: ResMut<Assets<Mesh>>,
    cloud_query: Query<&Handle<Mesh>, With<ParticleCloud>>,
) {
    let Ok(mesh_handle) = cloud_query.get_single() else { return };
    let Some(mesh) = meshes.get_mut(mesh_handle) else { return };

    // Update positions
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for (i, p) in pool.particles.iter().enumerate() {
            if p.active {
                positions[i] = [p.position.x, p.position.y, p.position.z];
            } else {
                positions[i] = [99999.0, 99999.0, 99999.0];
            }
        }
    }

    // Update colors
    if let Some(VertexAttributeValues::Float32x4(colors)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR)
    {
        for (i, p) in pool.particles.iter().enumerate() {
            if p.active {
                colors[i] = [p.color[0], p.color[1], p.color[2], 0.8];
            }
        }
    }
}
