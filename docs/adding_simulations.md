# Adding New Simulations

This guide explains how to add new simulations to the Entropy Zero platform.

## Prerequisites

- Rust 2021 edition
- Understanding of Bevy ECS patterns
- Familiarity with the `ez_core` traits

## Step 1: Choose the Right Crate

Simulations are organized by scientific domain:

| Domain | Crate Path |
|--------|------------|
| Classical Mechanics | `simulations/classical_mechanics/` |
| Electromagnetism | `simulations/electromagnetism/` |
| Wave Physics | `simulations/wave_physics/` |
| Optics | `simulations/optics/` |
| Economics | `simulations/economics/` |
| ... | ... |

If the domain doesn't exist, create a new crate following the pattern.

## Step 2: Create the Simulation Module

Create a new file, e.g., `src/my_simulation.rs`:

```rust
use bevy::prelude::*;
use ez_core::prelude::*;

// ═══════════════════════════════════════════════════════════════════
// Simulation Definition (implements the Simulation trait)
// ═══════════════════════════════════════════════════════════════════

pub struct MySimulation;

impl Simulation for MySimulation {
    fn id(&self) -> &'static str { "my_simulation" }
    fn name(&self) -> &'static str { "My Simulation" }
    
    fn category(&self) -> SimulationCategory {
        SimulationCategory::ClassicalMechanics(ClassicalMechanicsSubdomain::Dynamics)
    }
    
    fn description(&self) -> &'static str {
        "Description of what this simulation demonstrates."
    }
    
    fn parameters(&self) -> Vec<ParameterDef> {
        vec![
            ParameterDef::Float {
                id: "speed",
                name: "Speed",
                description: "Simulation speed",
                min: 0.0,
                max: 10.0,
                default: 1.0,
                step: Some(0.1),
                unit: None,
            },
        ]
    }
    
    fn build_plugin(&self) -> Box<dyn Fn(&mut App) + Send + Sync> {
        Box::new(|app| { app.add_plugins(MySimulationPlugin); })
    }
}

// ═══════════════════════════════════════════════════════════════════
// Components (pure data, no logic)
// ═══════════════════════════════════════════════════════════════════

#[derive(Component, Default, Reflect)]
pub struct MyComponent {
    pub velocity: Vec3,
}

// ═══════════════════════════════════════════════════════════════════
// Resources (global state)
// ═══════════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct MyConfig {
    pub speed: f32,
    pub paused: bool,
}

impl Default for MyConfig {
    fn default() -> Self {
        Self { speed: 1.0, paused: false }
    }
}

// ═══════════════════════════════════════════════════════════════════
// Plugin (wires everything together)
// ═══════════════════════════════════════════════════════════════════

pub struct MySimulationPlugin;

impl Plugin for MySimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyConfig>()
            .add_systems(Startup, setup)
            .add_systems(Update, (update_simulation, render_ui));
    }
}

// ═══════════════════════════════════════════════════════════════════
// Systems (stateless logic functions)
// ═══════════════════════════════════════════════════════════════════

fn setup(mut commands: Commands) {
    // Spawn entities, camera, lights
}

fn update_simulation(
    mut query: Query<&mut MyComponent>,
    config: Res<MyConfig>,
    time: Res<Time>,
) {
    if config.paused { return; }
    // Update logic
}

fn render_ui(
    mut contexts: bevy_egui::EguiContexts,
    mut config: ResMut<MyConfig>,
) {
    // egui UI code
}
```

## Step 3: Export from lib.rs

Add to the crate's `src/lib.rs`:

```rust
pub mod my_simulation;

pub fn all_simulations() -> Vec<Box<dyn Simulation>> {
    vec![
        Box::new(my_simulation::MySimulation),
        // ... other simulations
    ]
}
```

## Step 4: Register in Plugin

If using a domain plugin, register the simulation's plugin:

```rust
impl Plugin for ClassicalMechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(my_simulation::MySimulationPlugin);
    }
}
```

## Rules

1. **Components = Data Only**: No methods or logic inside components
2. **Systems = Logic Only**: Stateless functions that query components
3. **No `unwrap()`**: Use `expect()` with meaningful messages
4. **No HTML/CSS**: Use `bevy_egui` for UI
5. **Use parallel iteration**: `query.par_iter_mut()` for large entity counts
