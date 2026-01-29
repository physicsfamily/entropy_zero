//! Camera controllers for 3D navigation.

use bevy::prelude::*;

/// Plugin for camera control systems.
pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, orbit_camera_system);
    }
}

/// Component marking a camera with orbit controls.
#[derive(Component)]
pub struct OrbitCamera {
    pub focus: Vec3,
    pub distance: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            distance: 50.0,
            pitch: -0.5,
            yaw: 0.0,
        }
    }
}

/// Spawns a 3D camera with orbit controls.
pub fn spawn_orbit_camera(commands: &mut Commands, config: OrbitCamera) -> Entity {
    let position = calculate_camera_position(&config);

    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_translation(position).looking_at(config.focus, Vec3::Y),
                ..default()
            },
            config,
        ))
        .id()
}

fn calculate_camera_position(config: &OrbitCamera) -> Vec3 {
    let x = config.distance * config.pitch.cos() * config.yaw.sin();
    let y = config.distance * config.pitch.sin();
    let z = config.distance * config.pitch.cos() * config.yaw.cos();
    config.focus + Vec3::new(x, -y, z)
}

fn orbit_camera_system(
    mut query: Query<(&mut Transform, &OrbitCamera)>,
) {
    for (mut transform, orbit) in query.iter_mut() {
        let position = calculate_camera_position(orbit);
        transform.translation = position;
        transform.look_at(orbit.focus, Vec3::Y);
    }
}
