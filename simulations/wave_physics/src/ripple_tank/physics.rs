//! Physics systems for wave propagation using FDTD method

use bevy::prelude::*;
use std::f32::consts::PI;

use super::{
    components::*, resources::*, GRID_SCALE, GRID_SIZE, MAX_PROBE_HISTORY,
};

// ══════════════════════════════════════════════════════════════════════════════
// Setup
// ══════════════════════════════════════════════════════════════════════════════

pub fn setup_scene(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 100.0),
        projection: OrthographicProjection {
            scale: 1.5,
            ..default()
        },
        ..default()
    });

    // Create wave field visualization texture
    let size = bevy::render::render_resource::Extent3d {
        width: GRID_SIZE as u32,
        height: GRID_SIZE as u32,
        depth_or_array_layers: 1,
    };

    let mut image = Image::new_fill(
        size,
        bevy::render::render_resource::TextureDimension::D2,
        &[0, 50, 100, 255],
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD
            | bevy::render::render_asset::RenderAssetUsages::MAIN_WORLD,
    );
    image.sampler = bevy::render::texture::ImageSampler::nearest();

    let image_handle = images.add(image);

    // Spawn wave field sprite
    commands.spawn((
        SpriteBundle {
            texture: image_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(GRID_SCALE)),
            ..default()
        },
        WaveFieldVisual { texture: image_handle },
    ));

    // Spawn default point source
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.3, 0.3),
                custom_size: Some(Vec2::splat(12.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        WaveSource::default(),
        SceneObject { id: 0, selected: false, locked: false },
    ));
}

// ══════════════════════════════════════════════════════════════════════════════
// Input Handling
// ══════════════════════════════════════════════════════════════════════════════

pub fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<RippleTankConfig>,
    mut wave_field: ResMut<WaveField>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        config.paused = !config.paused;
    }
    if keyboard.just_pressed(KeyCode::KeyC) {
        wave_field.clear();
    }
    if keyboard.just_pressed(KeyCode::KeyG) {
        config.show_grid = !config.show_grid;
    }
}

pub fn handle_mouse_input(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut ui_state: ResMut<UIState>,
    mut commands: Commands,
    mut object_id: ResMut<ObjectIdCounter>,
    mut scene_objects: Query<(Entity, &mut Transform, &SceneObject)>,
    mut contexts: bevy_egui::EguiContexts,
) {
    if contexts.ctx_mut().is_pointer_over_area() {
        return;
    }

    let Ok(window) = windows.get_single() else { return };
    let Ok((camera, camera_transform)) = camera_q.get_single() else { return };
    let Some(cursor_pos) = window.cursor_position() else { return };
    let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return };

    if mouse_button.just_pressed(MouseButton::Left) {
        match ui_state.selected_tool {
            ToolType::Select => {
                let mut found = None;
                for (entity, transform, _) in scene_objects.iter() {
                    if transform.translation.truncate().distance(world_pos) < 15.0 {
                        found = Some(entity);
                        break;
                    }
                }
                ui_state.selected_entity = found;
                if let Some(entity) = found {
                    ui_state.dragging = Some(entity);
                    if let Ok((_, transform, _)) = scene_objects.get(entity) {
                        ui_state.drag_offset = transform.translation.truncate() - world_pos;
                    }
                }
            }
            ToolType::PointSource => super::spawn::spawn_point_source(&mut commands, &mut object_id, world_pos),
            ToolType::LineSource => super::spawn::spawn_line_source(&mut commands, &mut object_id, world_pos),
            ToolType::PhasedArray => super::spawn::spawn_phased_array(&mut commands, &mut object_id, world_pos),
            ToolType::MovingSource => super::spawn::spawn_moving_source(&mut commands, &mut object_id, world_pos),
            ToolType::Reflector => super::spawn::spawn_reflector(&mut commands, &mut object_id, world_pos),
            ToolType::SingleSlit => super::spawn::spawn_single_slit(&mut commands, &mut object_id, world_pos),
            ToolType::DoubleSlit => super::spawn::spawn_double_slit(&mut commands, &mut object_id, world_pos),
            ToolType::RefractionBlock => super::spawn::spawn_refraction_block(&mut commands, &mut object_id, world_pos),
            ToolType::Probe => super::spawn::spawn_probe(&mut commands, &mut object_id, world_pos),
            ToolType::Ruler => super::spawn::spawn_ruler(&mut commands, &mut object_id, world_pos),
        }
    }

    if mouse_button.pressed(MouseButton::Left) {
        if let Some(entity) = ui_state.dragging {
            if let Ok((_, mut transform, obj)) = scene_objects.get_mut(entity) {
                if !obj.locked {
                    transform.translation.x = world_pos.x + ui_state.drag_offset.x;
                    transform.translation.y = world_pos.y + ui_state.drag_offset.y;
                }
            }
        }
    }

    if mouse_button.just_released(MouseButton::Left) {
        ui_state.dragging = None;
    }

    if mouse_button.just_pressed(MouseButton::Right) {
        ui_state.selected_entity = None;
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Physics Update
// ══════════════════════════════════════════════════════════════════════════════

pub fn update_moving_sources(
    mut sources: Query<(&mut Transform, &MovingSource)>,
    config: Res<RippleTankConfig>,
    time: Res<Time>,
) {
    if config.paused { return; }

    let dt = time.delta_seconds() * config.time_scale;
    let bounds = (GRID_SIZE as f32 / 2.0) * GRID_SCALE;

    for (mut transform, moving) in sources.iter_mut() {
        transform.translation.x += moving.velocity.x * dt;
        transform.translation.y += moving.velocity.y * dt;

        if transform.translation.x.abs() > bounds {
            transform.translation.x = -transform.translation.x.signum() * (bounds - 10.0);
        }
        if transform.translation.y.abs() > bounds {
            transform.translation.y = -transform.translation.y.signum() * (bounds - 10.0);
        }
    }
}

pub fn rasterize_obstacles(
    mut wave_field: ResMut<WaveField>,
    obstacles: Query<(&Transform, &Obstacle)>,
) {
    wave_field.clear_obstacles();

    let half_width = wave_field.width as f32 / 2.0;
    let half_height = wave_field.height as f32 / 2.0;

    for (transform, obstacle) in obstacles.iter() {
        let center_x = (transform.translation.x / GRID_SCALE + half_width) as i32;
        let center_y = (transform.translation.y / GRID_SCALE + half_height) as i32;
        let half_w = (obstacle.width / GRID_SCALE / 2.0) as i32;
        let half_h = (obstacle.height / GRID_SCALE / 2.0) as i32;

        let width = wave_field.width;
        let height = wave_field.height;

        match obstacle.obstacle_type {
            ObstacleType::Reflector => {
                for dy in -half_h..=half_h {
                    for dx in -half_w..=half_w {
                        let x = (center_x + dx) as usize;
                        let y = (center_y + dy) as usize;
                        if x < width && y < height {
                            let idx = y * width + x;
                            wave_field.obstacle_map[idx] = 0.0;
                        }
                    }
                }
            }
            ObstacleType::SingleSlit => {
                let slit_half = (obstacle.slit_width / GRID_SCALE / 2.0) as i32;
                for dy in -half_h..=half_h {
                    for dx in -half_w..=half_w {
                        if dx.abs() <= slit_half { continue; }
                        let x = (center_x + dx) as usize;
                        let y = (center_y + dy) as usize;
                        if x < width && y < height {
                            let idx = y * width + x;
                            wave_field.obstacle_map[idx] = 0.0;
                        }
                    }
                }
            }
            ObstacleType::DoubleSlit => {
                let slit_half = (obstacle.slit_width / GRID_SCALE / 2.0) as i32;
                let sep_half = (obstacle.slit_separation / GRID_SCALE / 2.0) as i32;
                for dy in -half_h..=half_h {
                    for dx in -half_w..=half_w {
                        let in_slit1 = (dx - sep_half).abs() <= slit_half;
                        let in_slit2 = (dx + sep_half).abs() <= slit_half;
                        if in_slit1 || in_slit2 { continue; }
                        let x = (center_x + dx) as usize;
                        let y = (center_y + dy) as usize;
                        if x < width && y < height {
                            let idx = y * width + x;
                            wave_field.obstacle_map[idx] = 0.0;
                        }
                    }
                }
            }
            ObstacleType::RefractionBlock => {
                let speed_factor = 1.0 / obstacle.refractive_index;
                for dy in -half_h..=half_h {
                    for dx in -half_w..=half_w {
                        let x = (center_x + dx) as usize;
                        let y = (center_y + dy) as usize;
                        if x < width && y < height {
                            let idx = y * width + x;
                            wave_field.obstacle_map[idx] = speed_factor;
                        }
                    }
                }
            }
        }
    }
}

pub fn apply_wave_sources(
    mut wave_field: ResMut<WaveField>,
    sources: Query<(&Transform, &WaveSource)>,
    config: Res<RippleTankConfig>,
) {
    if config.paused { return; }

    let half_width = wave_field.width as f32 / 2.0;
    let half_height = wave_field.height as f32 / 2.0;
    let t = config.accumulated_time;

    for (transform, source) in sources.iter() {
        if !source.enabled { continue; }

        let grid_x = (transform.translation.x / GRID_SCALE + half_width) as usize;
        let grid_y = (transform.translation.y / GRID_SCALE + half_height) as usize;

        let value = match source.waveform {
            Waveform::Sine => source.amplitude * (2.0 * PI * source.frequency * t + source.phase).sin(),
            Waveform::Square => {
                let phase = (2.0 * PI * source.frequency * t + source.phase).sin();
                source.amplitude * phase.signum()
            }
            Waveform::Pulse => {
                let phase = (source.frequency * t + source.phase / (2.0 * PI)) % 1.0;
                if phase < 0.1 { source.amplitude } else { 0.0 }
            }
        };

        let width = wave_field.width;
        let height = wave_field.height;

        match source.source_type {
            WaveSourceType::Point | WaveSourceType::Moving => {
                if grid_x < width && grid_y < height {
                    let idx = grid_y * width + grid_x;
                    wave_field.current[idx] = value;
                }
            }
            WaveSourceType::Line => {
                let half_len = 20usize;
                for dx in 0..half_len * 2 {
                    let x = grid_x.saturating_sub(half_len) + dx;
                    if x < width && grid_y < height {
                        let idx = grid_y * width + x;
                        wave_field.current[idx] = value;
                    }
                }
            }
            WaveSourceType::PhasedArray { count } => {
                let spacing = 8usize;
                let total_w = (count as usize - 1) * spacing;
                let start_x = grid_x.saturating_sub(total_w / 2);
                for i in 0..count as usize {
                    let x = start_x + i * spacing;
                    let phase_offset = i as f32 * 0.2;
                    let phased_value = source.amplitude * (2.0 * PI * source.frequency * t + source.phase + phase_offset).sin();
                    if x < width && grid_y < height {
                        let idx = grid_y * width + x;
                        wave_field.current[idx] = phased_value;
                    }
                }
            }
        }
    }
}

pub fn update_wave_field(
    mut wave_field: ResMut<WaveField>,
    mut config: ResMut<RippleTankConfig>,
    time: Res<Time>,
) {
    if config.paused { return; }

    let dt = time.delta_seconds() * config.time_scale;
    config.accumulated_time += dt;

    let c = config.wave_speed;
    let damping = config.damping;
    let width = wave_field.width;
    let height = wave_field.height;
    let c2 = (c * 0.4).powi(2);

    let mut next = vec![0.0f32; width * height];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let idx = y * width + x;
            let obstacle = wave_field.obstacle_map[idx];

            if obstacle == 0.0 {
                next[idx] = 0.0;
                continue;
            }

            let laplacian = wave_field.current[idx - 1]
                + wave_field.current[idx + 1]
                + wave_field.current[idx - width]
                + wave_field.current[idx + width]
                - 4.0 * wave_field.current[idx];

            let effective_c2 = c2 * obstacle * obstacle;
            next[idx] = damping * (2.0 * wave_field.current[idx] - wave_field.previous[idx] + effective_c2 * laplacian);
            next[idx] = next[idx].clamp(-5.0, 5.0);
        }
    }

    // Absorbing boundaries
    for x in 0..width {
        next[x] *= 0.5;
        next[(height - 1) * width + x] *= 0.5;
    }
    for y in 0..height {
        next[y * width] *= 0.5;
        next[y * width + width - 1] *= 0.5;
    }

    let old_current = std::mem::replace(&mut wave_field.current, next);
    wave_field.previous = old_current;
}

pub fn update_probes(mut probes: Query<(&Transform, &mut Probe)>, wave_field: Res<WaveField>) {
    for (transform, mut probe) in probes.iter_mut() {
        let value = wave_field.sample(transform.translation.truncate());
        probe.history.push(value);
        if probe.history.len() > MAX_PROBE_HISTORY {
            probe.history.remove(0);
        }
    }
}

pub fn update_wave_visualization(
    wave_field: Res<WaveField>,
    config: Res<RippleTankConfig>,
    mut images: ResMut<Assets<Image>>,
    visual_query: Query<&WaveFieldVisual>,
) {
    let Ok(visual) = visual_query.get_single() else { return };
    let Some(image) = images.get_mut(&visual.texture) else { return };

    for y in 0..wave_field.height {
        for x in 0..wave_field.width {
            let idx = wave_field.idx(x, y);
            let value = wave_field.current[idx];
            let obstacle = wave_field.obstacle_map[idx];

            let (r, g, b) = if obstacle == 0.0 {
                (60, 60, 70)
            } else {
                match config.color_scheme {
                    ColorScheme::DeepOcean => {
                        let v = ((value + 1.0) * 0.5).clamp(0.0, 1.0);
                        ((20.0 + v * 40.0) as u8, (40.0 + v * 80.0) as u8, (80.0 + v * 175.0) as u8)
                    }
                    ColorScheme::Scientific => {
                        let v = ((value + 1.0) * 0.5).clamp(0.0, 1.0);
                        if v < 0.5 {
                            let t = v * 2.0;
                            ((255.0 * (1.0 - t)) as u8, (255.0 * t) as u8, 0)
                        } else {
                            let t = (v - 0.5) * 2.0;
                            (0, (255.0 * (1.0 - t)) as u8, (255.0 * t) as u8)
                        }
                    }
                    ColorScheme::PhaseColor => {
                        let hue = ((value.atan2(0.5) + PI) / (2.0 * PI) * 360.0) as u16;
                        hsl_to_rgb(hue, 80, 50)
                    }
                    ColorScheme::Grayscale => {
                        let v = ((value + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;
                        (v, v, v)
                    }
                }
            };

            let pixel_idx = idx * 4;
            image.data[pixel_idx] = r;
            image.data[pixel_idx + 1] = g;
            image.data[pixel_idx + 2] = b;
            image.data[pixel_idx + 3] = 255;
        }
    }
}

fn hsl_to_rgb(h: u16, s: u8, l: u8) -> (u8, u8, u8) {
    let h = h as f32 / 360.0;
    let s = s as f32 / 100.0;
    let l = l as f32 / 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match (h * 6.0) as u8 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (((r + m) * 255.0) as u8, ((g + m) * 255.0) as u8, ((b + m) * 255.0) as u8)
}

pub fn update_stats(
    mut stats: ResMut<SimulationStats>,
    time: Res<Time>,
    config: Res<RippleTankConfig>,
    wave_field: Res<WaveField>,
    probes: Query<&Probe>,
) {
    stats.fps = 1.0 / time.delta_seconds();
    stats.simulation_time = config.accumulated_time;

    let energy: f32 = wave_field.current.iter()
        .zip(wave_field.previous.iter())
        .map(|(c, p)| (c - p).powi(2) + c.powi(2))
        .sum();
    stats.wave_energy = energy;

    let probe_vec: Vec<&Probe> = probes.iter().collect();
    if probe_vec.len() >= 2 {
        let p1 = &probe_vec[0].history;
        let p2 = &probe_vec[1].history;
        if p1.len() > 10 && p2.len() > 10 {
            let len = p1.len().min(p2.len());
            let correlation: f32 = p1[len - 10..len].iter()
                .zip(p2[len - 10..len].iter())
                .map(|(a, b)| a * b)
                .sum();
            stats.probe_phase_diff = Some(correlation.atan2(1.0));
        }
    } else {
        stats.probe_phase_diff = None;
    }
}
