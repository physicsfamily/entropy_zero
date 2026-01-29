# Entropy Zero Developer Guide

> A comprehensive guide to creating new simulations for the Entropy Zero scientific simulation platform.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Architecture Overview](#architecture-overview)
3. [Creating a New Simulation](#creating-a-new-simulation)
   - [Step 1: Choose the Domain](#step-1-choose-the-domain)
   - [Step 2: Create the Simulation File](#step-2-create-the-simulation-file)
   - [Step 3: Implement the Simulation Trait](#step-3-implement-the-simulation-trait)
   - [Step 4: Define Components](#step-4-define-components)
   - [Step 5: Define Resources](#step-5-define-resources)
   - [Step 6: Create the Plugin](#step-6-create-the-plugin)
   - [Step 7: Implement Systems](#step-7-implement-systems)
   - [Step 8: Add UI Controls](#step-8-add-ui-controls)
   - [Step 9: Export and Register](#step-9-export-and-register)
4. [Complete Example: Pendulum Simulation](#complete-example-pendulum-simulation)
5. [Testing Your Simulation](#testing-your-simulation)
6. [Best Practices](#best-practices)
7. [Troubleshooting](#troubleshooting)

---

## Prerequisites

Before creating a simulation, ensure you have:

- **Rust 1.75+** installed (`rustup update stable`)
- **Cargo** package manager
- Basic understanding of **Bevy ECS** (Entity-Component-System)
- Familiarity with Rust traits and derive macros

### Recommended Reading

- [Bevy Book](https://bevyengine.org/learn/book/introduction/)
- [Bevy ECS Guide](https://bevy-cheatbook.github.io/programming/ecs-intro.html)
- [Egui Documentation](https://docs.rs/egui/latest/egui/)

---

## Architecture Overview

Entropy Zero uses a **layered monorepo architecture**:

```
┌─────────────────────────────────────────┐
│              apps/web                   │  Application Layer
├─────────────────────────────────────────┤
│    classical_mechanics, optics, ...     │  Simulation Layer
├─────────────────────────────────────────┤
│  ez_renderer | ez_ui | ez_physics       │  Infrastructure Layer
├─────────────────────────────────────────┤
│              ez_core                    │  Core Layer
└─────────────────────────────────────────┘
```

### Key Concepts

| Concept | Description |
|---------|-------------|
| **Simulation** | A trait that defines metadata and builds a Bevy plugin |
| **Component** | Pure data attached to entities (no logic) |
| **Resource** | Global singleton data (configuration, statistics) |
| **System** | Stateless functions that query and mutate components |
| **Plugin** | Groups components, resources, and systems together |

---

## Creating a New Simulation

### Step 1: Choose the Domain

Simulations are organized by scientific domain. Find the appropriate crate in `simulations/`:

| Domain | Crate | Examples |
|--------|-------|----------|
| Classical Mechanics | `simulations/classical_mechanics` | Particles, pendulums, springs |
| Electromagnetism | `simulations/electromagnetism` | Electric fields, circuits |
| Wave Physics | `simulations/wave_physics` | Ripple tank, interference |
| Optics | `simulations/optics` | Ray tracing, lenses |
| Thermodynamics | `simulations/thermodynamics` | Heat diffusion, ideal gas |
| Economics | `simulations/economics` | Supply-demand, markets |

If your domain doesn't exist yet, you'll need to [create a new domain crate](#creating-a-new-domain-crate).

---

### Step 2: Create the Simulation File

Create a new Rust file in the domain's `src/` directory:

```bash
touch simulations/classical_mechanics/src/pendulum.rs
```

Add the module declaration to `lib.rs`:

```rust
// simulations/classical_mechanics/src/lib.rs
pub mod particle_system;
pub mod pendulum;  // ← Add this line
```

---

### Step 3: Implement the Simulation Trait

Every simulation must implement the `Simulation` trait from `ez_core`. This provides metadata for the simulation browser and enables dynamic loading.

```rust
// simulations/classical_mechanics/src/pendulum.rs

use bevy::prelude::*;
use ez_core::prelude::*;

/// Metadata definition for the pendulum simulation.
pub struct PendulumSimulation;

impl Simulation for PendulumSimulation {
    /// Unique identifier (used for routing, storage)
    fn id(&self) -> &'static str {
        "simple_pendulum"
    }

    /// Human-readable name (displayed in UI)
    fn name(&self) -> &'static str {
        "Simple Pendulum"
    }

    /// Scientific classification
    fn category(&self) -> SimulationCategory {
        SimulationCategory::ClassicalMechanics(
            ClassicalMechanicsSubdomain::Dynamics
        )
    }

    /// Description for the simulation browser
    fn description(&self) -> &'static str {
        "A simple pendulum demonstrating harmonic motion. \
         Drag the bob to set initial conditions."
    }

    /// Parameters exposed to the UI (auto-generates controls)
    fn parameters(&self) -> Vec<ParameterDef> {
        vec![
            ParameterDef::Float {
                id: "length",
                name: "Pendulum Length",
                description: "Length of the pendulum arm",
                min: 0.5,
                max: 10.0,
                default: 2.0,
                step: Some(0.1),
                unit: Some("m"),
            },
            ParameterDef::Float {
                id: "gravity",
                name: "Gravity",
                description: "Gravitational acceleration",
                min: 0.0,
                max: 20.0,
                default: 9.8,
                step: Some(0.1),
                unit: Some("m/s²"),
            },
            ParameterDef::Float {
                id: "damping",
                name: "Damping",
                description: "Air resistance factor",
                min: 0.0,
                max: 1.0,
                default: 0.01,
                step: Some(0.01),
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

    /// Factory function that registers the simulation's plugin
    fn build_plugin(&self) -> Box<dyn Fn(&mut App) + Send + Sync> {
        Box::new(|app| {
            app.add_plugins(PendulumPlugin);
        })
    }

    /// Difficulty rating (1-5, for sorting in browser)
    fn difficulty(&self) -> u8 {
        2
    }

    /// Tags for search/filtering
    fn tags(&self) -> &'static [&'static str] {
        &["pendulum", "harmonic", "oscillation", "dynamics"]
    }
}
```

### Parameter Types Reference

The `ParameterDef` enum supports these types:

```rust
// Float slider with optional unit display
ParameterDef::Float {
    id: "velocity",
    name: "Initial Velocity",
    description: "Starting velocity",
    min: 0.0,
    max: 100.0,
    default: 10.0,
    step: Some(0.5),      // Optional: slider step size
    unit: Some("m/s"),    // Optional: unit label
}

// Integer slider
ParameterDef::Int {
    id: "count",
    name: "Object Count",
    description: "Number of objects",
    min: 1,
    max: 1000,
    default: 100,
}

// Boolean checkbox
ParameterDef::Bool {
    id: "show_trails",
    name: "Show Trails",
    description: "Display motion trails",
    default: true,
}

// 3D vector input
ParameterDef::Vec3 {
    id: "initial_position",
    name: "Initial Position",
    description: "Starting position",
    default: [0.0, 5.0, 0.0],
}

// Color picker
ParameterDef::Color {
    id: "object_color",
    name: "Object Color",
    description: "Color of objects",
    default: [0.2, 0.7, 1.0, 1.0],
}

// Dropdown selection
ParameterDef::Enum {
    id: "mode",
    name: "Simulation Mode",
    description: "Choose simulation behavior",
    variants: &["Normal", "Fast", "Slow Motion"],
    default: 0,
}
```

---

### Step 4: Define Components

Components are pure data containers with no logic. They are attached to entities.

```rust
// ═══════════════════════════════════════════════════════════════════
// Components
// ═══════════════════════════════════════════════════════════════════

/// State of the pendulum bob.
#[derive(Component, Reflect)]
pub struct Pendulum {
    /// Current angle from vertical (radians)
    pub angle: f32,
    /// Angular velocity (rad/s)
    pub angular_velocity: f32,
}

impl Default for Pendulum {
    fn default() -> Self {
        Self {
            angle: std::f32::consts::PI / 4.0,  // Start at 45°
            angular_velocity: 0.0,
        }
    }
}

/// Marker component for the pendulum arm visual.
#[derive(Component)]
pub struct PendulumArm;

/// Marker component for the pivot point.
#[derive(Component)]
pub struct PivotPoint;
```

### Component Best Practices

| ✅ Do | ❌ Don't |
|------|---------|
| Store only data | Add methods with logic |
| Use `#[derive(Component)]` | Forget the derive macro |
| Implement `Default` when sensible | Use complex nested types |
| Add `#[derive(Reflect)]` for debugging | Store references (`&'a`) |

---

### Step 5: Define Resources

Resources are global singletons for configuration and runtime statistics.

```rust
// ═══════════════════════════════════════════════════════════════════
// Resources
// ═══════════════════════════════════════════════════════════════════

/// Configuration parameters (controllable via UI).
#[derive(Resource)]
pub struct PendulumConfig {
    pub length: f32,
    pub gravity: f32,
    pub damping: f32,
    pub paused: bool,
}

impl Default for PendulumConfig {
    fn default() -> Self {
        Self {
            length: 2.0,
            gravity: 9.8,
            damping: 0.01,
            paused: false,
        }
    }
}

/// Runtime statistics (displayed in UI).
#[derive(Resource, Default)]
pub struct PendulumStats {
    pub current_angle: f32,
    pub max_angle: f32,
    pub period: f32,
    pub energy: f32,
}
```

### Resource Guidelines

- **Config resources**: Store user-controllable parameters
- **Stats resources**: Store computed values for display
- **State resources**: Store simulation state (e.g., time accumulator)

---

### Step 6: Create the Plugin

The plugin wires everything together.

```rust
// ═══════════════════════════════════════════════════════════════════
// Plugin
// ═══════════════════════════════════════════════════════════════════

pub struct PendulumPlugin;

impl Plugin for PendulumPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register resources
            .init_resource::<PendulumConfig>()
            .init_resource::<PendulumStats>()
            
            // Register component types for reflection/debugging
            .register_type::<Pendulum>()
            
            // Startup systems (run once)
            .add_systems(Startup, setup_pendulum_scene)
            
            // Update systems (run every frame)
            .add_systems(Update, (
                update_pendulum_physics,
                update_pendulum_visuals,
                update_pendulum_stats,
                render_pendulum_ui,
            ));
    }
}
```

### System Ordering

If systems have dependencies, use `.chain()` or explicit ordering:

```rust
.add_systems(Update, (
    read_input,
    update_physics,      // Must run after input
    update_visuals,      // Must run after physics
).chain())
```

Or with explicit ordering:

```rust
.add_systems(Update, update_physics.after(read_input))
.add_systems(Update, update_visuals.after(update_physics))
```

---

### Step 7: Implement Systems

Systems are stateless functions that query and mutate the world.

```rust
// ═══════════════════════════════════════════════════════════════════
// Systems
// ═══════════════════════════════════════════════════════════════════

/// Setup the scene (runs once at startup).
fn setup_pendulum_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<PendulumConfig>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
    });

    // Pivot point (fixed)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.1)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.5, 0.5, 0.5),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        PivotPoint,
    ));

    // Pendulum bob
    let initial_angle = std::f32::consts::PI / 4.0;
    let bob_position = calculate_bob_position(initial_angle, config.length);

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.3)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.5, 0.9),
                ..default()
            }),
            transform: Transform::from_translation(bob_position),
            ..default()
        },
        Pendulum::default(),
    ));
}

/// Update pendulum physics.
fn update_pendulum_physics(
    mut query: Query<&mut Pendulum>,
    config: Res<PendulumConfig>,
    time: Res<Time>,
) {
    if config.paused {
        return;
    }

    let dt = time.delta_seconds();

    for mut pendulum in query.iter_mut() {
        // Simple pendulum equation: θ'' = -(g/L) * sin(θ) - c * θ'
        let angular_acceleration = 
            -(config.gravity / config.length) * pendulum.angle.sin()
            - config.damping * pendulum.angular_velocity;

        // Semi-implicit Euler integration
        pendulum.angular_velocity += angular_acceleration * dt;
        pendulum.angle += pendulum.angular_velocity * dt;
    }
}

/// Update visual positions to match physics state.
fn update_pendulum_visuals(
    mut query: Query<(&Pendulum, &mut Transform)>,
    config: Res<PendulumConfig>,
) {
    for (pendulum, mut transform) in query.iter_mut() {
        let bob_pos = calculate_bob_position(pendulum.angle, config.length);
        transform.translation = bob_pos;
    }
}

/// Calculate bob position from angle and length.
fn calculate_bob_position(angle: f32, length: f32) -> Vec3 {
    let pivot = Vec3::new(0.0, 2.0, 0.0);
    Vec3::new(
        pivot.x + length * angle.sin(),
        pivot.y - length * angle.cos(),
        0.0,
    )
}

/// Update statistics for display.
fn update_pendulum_stats(
    query: Query<&Pendulum>,
    config: Res<PendulumConfig>,
    mut stats: ResMut<PendulumStats>,
) {
    for pendulum in query.iter() {
        stats.current_angle = pendulum.angle.to_degrees();
        stats.max_angle = stats.max_angle.max(pendulum.angle.abs().to_degrees());
        
        // Theoretical period: T = 2π√(L/g)
        stats.period = 2.0 * std::f32::consts::PI 
            * (config.length / config.gravity).sqrt();
        
        // Total mechanical energy
        let kinetic = 0.5 * config.length.powi(2) 
            * pendulum.angular_velocity.powi(2);
        let potential = config.gravity * config.length 
            * (1.0 - pendulum.angle.cos());
        stats.energy = kinetic + potential;
    }
}
```

### System Parameters Reference

Common system parameters:

```rust
fn example_system(
    // Commands: spawn/despawn entities
    mut commands: Commands,
    
    // Query: access entities with specific components
    query: Query<(&Transform, &Velocity), With<Player>>,
    mut query_mut: Query<&mut Transform>,
    
    // Resources: global singletons
    config: Res<MyConfig>,
    mut config_mut: ResMut<MyConfig>,
    
    // Time: delta time, elapsed time
    time: Res<Time>,
    
    // Assets: meshes, materials, sounds
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    
    // Input: keyboard, mouse
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    
    // Events: send/receive events
    mut events: EventWriter<MyEvent>,
    events: EventReader<MyEvent>,
)
```

---

### Step 8: Add UI Controls

Use `bevy_egui` for immediate-mode UI:

```rust
use bevy_egui::{egui, EguiContexts};

/// Render the control panel.
fn render_pendulum_ui(
    mut contexts: EguiContexts,
    mut config: ResMut<PendulumConfig>,
    stats: Res<PendulumStats>,
    mut query: Query<&mut Pendulum>,
) {
    egui::Window::new("Pendulum Controls")
        .default_pos([10.0, 10.0])
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            // Statistics section
            ui.heading("📊 Statistics");
            ui.label(format!("Angle: {:.1}°", stats.current_angle));
            ui.label(format!("Max Angle: {:.1}°", stats.max_angle));
            ui.label(format!("Period: {:.2} s", stats.period));
            ui.label(format!("Energy: {:.3} J", stats.energy));
            
            ui.separator();
            
            // Controls section
            ui.heading("⚙️ Parameters");
            
            // Play/Pause button
            let pause_text = if config.paused { "▶ Play" } else { "⏸ Pause" };
            if ui.button(pause_text).clicked() {
                config.paused = !config.paused;
            }
            
            // Reset button
            if ui.button("🔄 Reset").clicked() {
                for mut pendulum in query.iter_mut() {
                    pendulum.angle = std::f32::consts::PI / 4.0;
                    pendulum.angular_velocity = 0.0;
                }
            }
            
            ui.add_space(10.0);
            
            // Parameter sliders
            ui.add(
                egui::Slider::new(&mut config.length, 0.5..=10.0)
                    .text("Length (m)")
            );
            
            ui.add(
                egui::Slider::new(&mut config.gravity, 0.0..=20.0)
                    .text("Gravity (m/s²)")
            );
            
            ui.add(
                egui::Slider::new(&mut config.damping, 0.0..=1.0)
                    .text("Damping")
            );
        });
}
```

### UI Widgets Reference

```rust
// Labels
ui.label("Hello World");
ui.heading("Section Title");

// Buttons
if ui.button("Click Me").clicked() { /* action */ }

// Sliders
ui.add(egui::Slider::new(&mut value, 0.0..=100.0).text("Value"));

// Checkboxes
ui.checkbox(&mut enabled, "Enable feature");

// Color picker
ui.color_edit_button_rgb(&mut color);

// Combo box (dropdown)
egui::ComboBox::from_label("Mode")
    .selected_text(modes[selected])
    .show_ui(ui, |ui| {
        for (i, mode) in modes.iter().enumerate() {
            ui.selectable_value(&mut selected, i, *mode);
        }
    });

// Horizontal layout
ui.horizontal(|ui| {
    ui.label("X:");
    ui.add(egui::DragValue::new(&mut x));
    ui.label("Y:");
    ui.add(egui::DragValue::new(&mut y));
});

// Collapsing section
ui.collapsible("Advanced", |ui| {
    // advanced controls here
});
```

---

### Step 9: Export and Register

#### 9.1 Export from the domain crate

```rust
// simulations/classical_mechanics/src/lib.rs

pub mod particle_system;
pub mod pendulum;

use bevy::prelude::*;
use ez_core::prelude::*;

/// Returns all simulations in this domain.
pub fn all_simulations() -> Vec<Box<dyn Simulation>> {
    vec![
        Box::new(particle_system::ParticleSystemSimulation),
        Box::new(pendulum::PendulumSimulation),
    ]
}

/// Plugin that registers all classical mechanics simulations.
pub struct ClassicalMechanicsPlugin;

impl Plugin for ClassicalMechanicsPlugin {
    fn build(&self, app: &mut App) {
        // Only add the simulation you want to run for now
        // In the future, this will be dynamic
        app.add_plugins(particle_system::ParticleSystemPlugin);
        // app.add_plugins(pendulum::PendulumPlugin);
    }
}
```

#### 9.2 Register in the application

For development, modify the app entry point to load your simulation:

```rust
// apps/web/src/main.rs

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

// Import your simulation's plugin directly for testing
use classical_mechanics::pendulum::PendulumPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pendulum Simulation".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(PendulumPlugin)  // ← Your simulation
        .run();
}
```

---

## Complete Example: Pendulum Simulation

Here's the complete pendulum simulation file:

```rust
// simulations/classical_mechanics/src/pendulum.rs

//! Simple Pendulum Simulation
//!
//! Demonstrates harmonic motion with configurable length,
//! gravity, and damping.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use ez_core::prelude::*;

// ═══════════════════════════════════════════════════════════════════
// Simulation Definition
// ═══════════════════════════════════════════════════════════════════

pub struct PendulumSimulation;

impl Simulation for PendulumSimulation {
    fn id(&self) -> &'static str { "simple_pendulum" }
    fn name(&self) -> &'static str { "Simple Pendulum" }
    
    fn category(&self) -> SimulationCategory {
        SimulationCategory::ClassicalMechanics(ClassicalMechanicsSubdomain::Dynamics)
    }
    
    fn description(&self) -> &'static str {
        "A simple pendulum demonstrating harmonic motion."
    }
    
    fn parameters(&self) -> Vec<ParameterDef> {
        vec![
            ParameterDef::Float {
                id: "length", name: "Length", description: "Arm length",
                min: 0.5, max: 10.0, default: 2.0, step: Some(0.1), unit: Some("m"),
            },
            ParameterDef::Float {
                id: "gravity", name: "Gravity", description: "g",
                min: 0.0, max: 20.0, default: 9.8, step: Some(0.1), unit: Some("m/s²"),
            },
            ParameterDef::Bool {
                id: "paused", name: "Paused", description: "Pause", default: false,
            },
        ]
    }
    
    fn build_plugin(&self) -> Box<dyn Fn(&mut App) + Send + Sync> {
        Box::new(|app| { app.add_plugins(PendulumPlugin); })
    }
}

// ═══════════════════════════════════════════════════════════════════
// Components
// ═══════════════════════════════════════════════════════════════════

#[derive(Component, Reflect)]
pub struct Pendulum {
    pub angle: f32,
    pub angular_velocity: f32,
}

impl Default for Pendulum {
    fn default() -> Self {
        Self { angle: std::f32::consts::PI / 4.0, angular_velocity: 0.0 }
    }
}

// ═══════════════════════════════════════════════════════════════════
// Resources
// ═══════════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct PendulumConfig {
    pub length: f32,
    pub gravity: f32,
    pub damping: f32,
    pub paused: bool,
}

impl Default for PendulumConfig {
    fn default() -> Self {
        Self { length: 2.0, gravity: 9.8, damping: 0.01, paused: false }
    }
}

// ═══════════════════════════════════════════════════════════════════
// Plugin
// ═══════════════════════════════════════════════════════════════════

pub struct PendulumPlugin;

impl Plugin for PendulumPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PendulumConfig>()
            .register_type::<Pendulum>()
            .add_systems(Startup, setup)
            .add_systems(Update, (physics, visuals, ui));
    }
}

// ═══════════════════════════════════════════════════════════════════
// Systems
// ═══════════════════════════════════════════════════════════════════

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
    });

    // Bob
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.3)),
            material: materials.add(Color::srgb(0.2, 0.5, 0.9)),
            ..default()
        },
        Pendulum::default(),
    ));
}

fn physics(mut query: Query<&mut Pendulum>, config: Res<PendulumConfig>, time: Res<Time>) {
    if config.paused { return; }
    let dt = time.delta_seconds();
    
    for mut p in query.iter_mut() {
        let accel = -(config.gravity / config.length) * p.angle.sin() 
                    - config.damping * p.angular_velocity;
        p.angular_velocity += accel * dt;
        p.angle += p.angular_velocity * dt;
    }
}

fn visuals(mut query: Query<(&Pendulum, &mut Transform)>, config: Res<PendulumConfig>) {
    for (p, mut t) in query.iter_mut() {
        t.translation = Vec3::new(
            config.length * p.angle.sin(),
            2.0 - config.length * p.angle.cos(),
            0.0,
        );
    }
}

fn ui(mut ctx: EguiContexts, mut config: ResMut<PendulumConfig>, mut query: Query<&mut Pendulum>) {
    egui::Window::new("Pendulum").show(ctx.ctx_mut(), |ui| {
        if ui.button(if config.paused { "▶ Play" } else { "⏸ Pause" }).clicked() {
            config.paused = !config.paused;
        }
        if ui.button("🔄 Reset").clicked() {
            for mut p in query.iter_mut() {
                p.angle = std::f32::consts::PI / 4.0;
                p.angular_velocity = 0.0;
            }
        }
        ui.add(egui::Slider::new(&mut config.length, 0.5..=10.0).text("Length"));
        ui.add(egui::Slider::new(&mut config.gravity, 0.0..=20.0).text("Gravity"));
    });
}
```

---

## Testing Your Simulation

### Run locally

```bash
# From the workspace root
cargo run -p entropy_zero_web
```

### Run tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p classical_mechanics
```

### Add unit tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pendulum_physics_conserves_energy() {
        let mut pendulum = Pendulum {
            angle: std::f32::consts::PI / 4.0,
            angular_velocity: 0.0,
        };
        let config = PendulumConfig::default();
        
        // Calculate initial energy
        let initial_energy = calculate_energy(&pendulum, &config);
        
        // Simulate several steps
        for _ in 0..1000 {
            let accel = -(config.gravity / config.length) * pendulum.angle.sin();
            pendulum.angular_velocity += accel * 0.01;
            pendulum.angle += pendulum.angular_velocity * 0.01;
        }
        
        let final_energy = calculate_energy(&pendulum, &config);
        
        // Energy should be approximately conserved (within 5%)
        assert!((final_energy - initial_energy).abs() / initial_energy < 0.05);
    }
    
    fn calculate_energy(p: &Pendulum, c: &PendulumConfig) -> f32 {
        let kinetic = 0.5 * c.length.powi(2) * p.angular_velocity.powi(2);
        let potential = c.gravity * c.length * (1.0 - p.angle.cos());
        kinetic + potential
    }
}
```

---

## Best Practices

### Code Organization

1. **One simulation per file**: Keep simulations modular
2. **Group by section**: Use comment headers (Components, Resources, Systems, etc.)
3. **Document public APIs**: Add doc comments to all public items

### Performance

1. **Use parallel iteration** for large entity counts:
   ```rust
   query.par_iter_mut().for_each(|(mut transform, velocity)| {
       // ...
   });
   ```

2. **Avoid allocations in hot loops**: Pre-allocate vectors, use `SmallVec`

3. **Use `Changed<T>` and `Added<T>` filters**:
   ```rust
   fn update_on_change(query: Query<&Config, Changed<Config>>) {
       // Only runs when Config changes
   }
   ```

### Physics

1. **Use semi-implicit Euler** for stability:
   ```rust
   velocity += acceleration * dt;  // Update velocity first
   position += velocity * dt;      // Then position
   ```

2. **Consider sub-stepping** for fast-moving objects:
   ```rust
   let sub_steps = 4;
   let sub_dt = dt / sub_steps as f32;
   for _ in 0..sub_steps {
       // physics update
   }
   ```

3. **Use `ez_physics`** integrators for complex simulations

### UI

1. **Keep UI systems separate** from physics systems
2. **Use `ui.separator()`** to group related controls
3. **Add tooltips** for complex parameters:
   ```rust
   ui.add(egui::Slider::new(&mut value, 0.0..=1.0))
       .on_hover_text("This controls the simulation speed");
   ```

---

## Troubleshooting

### Common Issues

| Problem | Solution |
|---------|----------|
| "Component not found" | Add `#[derive(Component)]` to struct |
| "Resource not found" | Call `.init_resource::<T>()` in plugin |
| Entities not visible | Check camera position, add lights |
| UI not showing | Ensure `EguiPlugin` is added before your plugin |
| Slow performance | Use `.par_iter_mut()` for large queries |

### Debugging Tips

1. **Enable Bevy's debug overlay**:
   ```rust
   app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
   ```

2. **Log values**:
   ```rust
   use bevy::log::info;
   info!("Velocity: {:?}", velocity);
   ```

3. **Use the inspector egui plugin** (optional dependency):
   ```rust
   app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
   ```

---

## Creating a New Domain Crate

If your simulation doesn't fit in an existing domain:

1. Create the crate structure:
   ```bash
   mkdir -p simulations/my_domain/src
   ```

2. Create `Cargo.toml`:
   ```toml
   [package]
   name = "my_domain"
   version.workspace = true
   edition.workspace = true
   
   [dependencies]
   bevy.workspace = true
   bevy_egui.workspace = true
   ez_core.workspace = true
   ez_physics.workspace = true
   ```

3. Add to workspace in root `Cargo.toml`:
   ```toml
   [workspace]
   members = [
       # ...
       "simulations/my_domain",
   ]
   
   [workspace.dependencies]
   my_domain = { path = "simulations/my_domain" }
   ```

4. Create `src/lib.rs` following the pattern above.

---

## Resources

- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Egui Demo](https://www.egui.rs/)
- [Entropy Zero Architecture](./architecture.md)
- [Physics Constants](../crates/ez_core/src/math.rs)

---

*Last updated: 2026-01-29*
