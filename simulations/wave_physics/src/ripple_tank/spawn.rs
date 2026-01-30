//! Spawn helper functions for scene objects

use bevy::prelude::*;

use super::{components::*, resources::*};

pub fn spawn_point_source(commands: &mut Commands, object_id: &mut ObjectIdCounter, pos: Vec2) {
    object_id.0 += 1;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.3, 0.3),
                custom_size: Some(Vec2::splat(12.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 1.0),
            ..default()
        },
        WaveSource { source_type: WaveSourceType::Point, ..default() },
        SceneObject { id: object_id.0, selected: false, locked: false },
    ));
}

pub fn spawn_line_source(commands: &mut Commands, object_id: &mut ObjectIdCounter, pos: Vec2) {
    object_id.0 += 1;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.5, 0.2),
                custom_size: Some(Vec2::new(80.0, 8.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 1.0),
            ..default()
        },
        WaveSource { source_type: WaveSourceType::Line, ..default() },
        SceneObject { id: object_id.0, selected: false, locked: false },
    ));
}

pub fn spawn_phased_array(commands: &mut Commands, object_id: &mut ObjectIdCounter, pos: Vec2) {
    object_id.0 += 1;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.8, 0.2, 0.8),
                custom_size: Some(Vec2::new(60.0, 12.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 1.0),
            ..default()
        },
        WaveSource { source_type: WaveSourceType::PhasedArray { count: 5 }, ..default() },
        SceneObject { id: object_id.0, selected: false, locked: false },
    ));
}

pub fn spawn_moving_source(commands: &mut Commands, object_id: &mut ObjectIdCounter, pos: Vec2) {
    object_id.0 += 1;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.8, 0.4),
                custom_size: Some(Vec2::new(16.0, 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 1.0),
            ..default()
        },
        WaveSource { source_type: WaveSourceType::Moving, ..default() },
        MovingSource { velocity: Vec2::new(50.0, 0.0), path: MovementPath::Linear },
        SceneObject { id: object_id.0, selected: false, locked: false },
    ));
}

pub fn spawn_reflector(commands: &mut Commands, object_id: &mut ObjectIdCounter, pos: Vec2) {
    object_id.0 += 1;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.4, 0.4, 0.5),
                custom_size: Some(Vec2::new(80.0, 8.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 1.0),
            ..default()
        },
        Obstacle { obstacle_type: ObstacleType::Reflector, width: 80.0, height: 8.0, ..default() },
        SceneObject { id: object_id.0, selected: false, locked: false },
    ));
}

pub fn spawn_single_slit(commands: &mut Commands, object_id: &mut ObjectIdCounter, pos: Vec2) {
    object_id.0 += 1;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.3, 0.3, 0.6),
                custom_size: Some(Vec2::new(120.0, 8.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 1.0),
            ..default()
        },
        Obstacle {
            obstacle_type: ObstacleType::SingleSlit,
            width: 120.0,
            height: 8.0,
            slit_width: 15.0,
            ..default()
        },
        SceneObject { id: object_id.0, selected: false, locked: false },
    ));
}

pub fn spawn_double_slit(commands: &mut Commands, object_id: &mut ObjectIdCounter, pos: Vec2) {
    object_id.0 += 1;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.3, 0.5, 0.6),
                custom_size: Some(Vec2::new(120.0, 8.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 1.0),
            ..default()
        },
        Obstacle {
            obstacle_type: ObstacleType::DoubleSlit,
            width: 120.0,
            height: 8.0,
            slit_width: 10.0,
            slit_separation: 30.0,
            ..default()
        },
        SceneObject { id: object_id.0, selected: false, locked: false },
    ));
}

pub fn spawn_refraction_block(commands: &mut Commands, object_id: &mut ObjectIdCounter, pos: Vec2) {
    object_id.0 += 1;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.3, 0.6, 0.8, 0.5),
                custom_size: Some(Vec2::new(60.0, 60.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 0.5),
            ..default()
        },
        Obstacle {
            obstacle_type: ObstacleType::RefractionBlock,
            width: 60.0,
            height: 60.0,
            refractive_index: 1.5,
            ..default()
        },
        SceneObject { id: object_id.0, selected: false, locked: false },
    ));
}

pub fn spawn_probe(commands: &mut Commands, object_id: &mut ObjectIdCounter, pos: Vec2) {
    object_id.0 += 1;
    let color = if object_id.0 % 2 == 0 {
        Color::srgb(0.2, 0.6, 1.0)
    } else {
        Color::srgb(1.0, 0.4, 0.4)
    };
    let label = format!("Probe {}", (object_id.0 as u8 + b'A' - 1) as char);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::splat(10.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 2.0),
            ..default()
        },
        Probe::new(&label, color),
        SceneObject { id: object_id.0, selected: false, locked: false },
    ));
}

pub fn spawn_ruler(commands: &mut Commands, object_id: &mut ObjectIdCounter, pos: Vec2) {
    object_id.0 += 1;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 0.3, 0.8),
                custom_size: Some(Vec2::new(100.0, 5.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 2.0),
            ..default()
        },
        Ruler { start: Vec2::new(-50.0, 0.0), end: Vec2::new(50.0, 0.0) },
        SceneObject { id: object_id.0, selected: false, locked: false },
    ));
}
