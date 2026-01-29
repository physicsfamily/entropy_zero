# Simulation Development Guide

This guide explains how to create new simulations in the Entropy Zero engine following the ECS (Entity-Component-System) architecture.

## Architecture Overview

```
src/
├── main.rs            # Entry point: App builder & Plugin registration ONLY
├── components/        # Pure data structs (NO logic)
├── systems/           # Stateless logic functions
├── resources/         # Global singleton configs
└── plugins/           # Domain organizers that wire everything together
```

## Step-by-Step: Creating a New Simulation

### Step 1: Define Your Component (Data)

Components are **pure data containers**. No methods, no logic.

Create a new file: `src/components/my_entity.rs`

```rust
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct MyEntity {
    pub velocity: Vec3,
    pub mass: f32,
    pub lifetime: f32,
}
```

Export it in `src/components/mod.rs`:
```rust
pub mod my_entity;
```

### Step 2: Define Your Resource (Global State)

Resources hold global configuration and state.

Create: `src/resources/my_config.rs`

```rust
use bevy::prelude::*;

#[derive(Resource)]
pub struct MySimulationConfig {
    pub gravity: Vec3,
    pub time_scale: f32,
    pub paused: bool,
}

impl Default for MySimulationConfig {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.8, 0.0),
            time_scale: 1.0,
            paused: false,
        }
    }
}
```

Export it in `src/resources/mod.rs`:
```rust
pub mod my_config;
```

### Step 3: Define Your Systems (Logic)

Systems are **stateless functions** that operate on Components and Resources.

Create: `src/systems/my_systems.rs`

```rust
use bevy::prelude::*;
use crate::components::my_entity::MyEntity;
use crate::resources::my_config::MySimulationConfig;

pub fn update_entities(
    mut query: Query<(&mut Transform, &mut MyEntity)>,
    config: Res<MySimulationConfig>,
    time: Res<Time>,
) {
    if config.paused {
        return;
    }

    let dt = time.delta_seconds() * config.time_scale;

    // Use parallel iteration for large entity counts
    query.par_iter_mut().for_each(|(mut transform, mut entity)| {
        entity.velocity += config.gravity * dt;
        transform.translation += entity.velocity * dt;
        entity.lifetime -= dt;
    });
}

pub fn despawn_expired(
    mut commands: Commands,
    query: Query<(Entity, &MyEntity)>,
) {
    for (entity, my_entity) in query.iter() {
        if my_entity.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
```

Export it in `src/systems/mod.rs`:
```rust
pub mod my_systems;
```

### Step 4: Create Your Plugin (Organizer)

Plugins wire Components, Resources, and Systems together.

Create: `src/plugins/my_plugin.rs`

```rust
use bevy::prelude::*;
use crate::components::my_entity::MyEntity;
use crate::resources::my_config::MySimulationConfig;
use crate::systems::my_systems::{update_entities, despawn_expired};

pub struct MySimulationPlugin;

impl Plugin for MySimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register resources
            .init_resource::<MySimulationConfig>()
            
            // Register startup systems (run once)
            .add_systems(Startup, setup_scene)
            
            // Register update systems (run every frame)
            .add_systems(Update, (update_entities, despawn_expired));
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn your entities here
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.0).mesh().ico(3).unwrap()),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.2, 0.2),
                ..default()
            }),
            ..default()
        },
        MyEntity {
            velocity: Vec3::ZERO,
            mass: 1.0,
            lifetime: 10.0,
        },
    ));

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
```

Export it in `src/plugins/mod.rs`:
```rust
pub mod my_plugin;
```

### Step 5: Register Your Plugin

In `src/main.rs`:

```rust
use plugins::my_plugin::MySimulationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(MySimulationPlugin)  // Add your plugin here
        .run();
}
```

## Adding UI with bevy_egui

Create a UI system in `src/systems/my_ui.rs`:

```rust
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::resources::my_config::MySimulationConfig;

pub fn render_ui(
    mut contexts: EguiContexts,
    mut config: ResMut<MySimulationConfig>,
) {
    egui::Window::new("Controls").show(contexts.ctx_mut(), |ui| {
        if ui.button(if config.paused { "Resume" } else { "Pause" }).clicked() {
            config.paused = !config.paused;
        }
        ui.add(egui::Slider::new(&mut config.time_scale, 0.1..=5.0).text("Time Scale"));
    });
}
```

Add it to your plugin's Update systems.

## Performance Tips for Large Simulations

1. **Use `par_iter_mut()`** for queries with many entities (enables parallel processing)
2. **Share meshes and materials** - clone handles instead of creating new assets per entity
3. **Disable shadows** for large particle counts
4. **Use simple meshes** - `Sphere::new(r).mesh().ico(1)` instead of higher subdivisions
5. **Batch spawning** - spawn entities in chunks during setup

## Rules to Follow

1. **NO OOP inheritance** - use composition with Components
2. **Components = Data Only** - no methods or logic inside components
3. **Systems = Logic Only** - stateless functions that query components
4. **NO `unwrap()`** - use `expect()` with meaningful messages or handle errors
5. **NO HTML/CSS** - use `bevy_egui` for all UI

## Running Your Simulation

```bash
# Native (desktop)
cargo run

# WASM (browser)
trunk serve
```
