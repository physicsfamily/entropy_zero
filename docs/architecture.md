# Entropy Zero Architecture

## Overview

Entropy Zero is a modular scientific simulation platform built with Rust and Bevy. The architecture is designed to scale to dozens of simulation domains while maintaining clean separation of concerns.

## Directory Structure

```
entropy_zero/
├── Cargo.toml                    # Workspace root
├── crates/                       # Core infrastructure
│   ├── ez_core/                  # Traits, taxonomy, parameters, math
│   ├── ez_renderer/              # Camera, grid, materials
│   ├── ez_ui/                    # Control panels, widgets
│   └── ez_physics/               # Forces, integrators
├── simulations/                  # Domain-specific simulations
│   └── classical_mechanics/      # Particle systems, pendulums, etc.
└── apps/                         # Runnable applications
    └── web/                      # WASM web application
```

## Dependency Layers

```
┌─────────────────────────────────────────┐
│                 apps/web                │  Application Layer
├─────────────────────────────────────────┤
│          classical_mechanics            │  Simulation Layer
├─────────────────────────────────────────┤
│  ez_renderer  │  ez_ui  │  ez_physics   │  Infrastructure Layer
├─────────────────────────────────────────┤
│               ez_core                   │  Core Layer
└─────────────────────────────────────────┘
```

## Core Concepts

### Simulation Trait

Every simulation implements the `Simulation` trait from `ez_core`:

```rust
pub trait Simulation: Send + Sync + 'static {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn category(&self) -> SimulationCategory;
    fn description(&self) -> &'static str;
    fn parameters(&self) -> Vec<ParameterDef>;
    fn build_plugin(&self) -> Box<dyn Fn(&mut App) + Send + Sync>;
}
```

### Taxonomy

Simulations are classified using `SimulationCategory`:

- **Physical Sciences**: Classical Mechanics, Electromagnetism, Wave Physics, Optics, Thermodynamics, Relativistic Physics, Quantum Mechanics
- **Life Sciences**: Epidemiology, Ecology, Neuroscience
- **Social Sciences**: Economics, Game Theory, Social Networks
- **Formal Sciences**: Cellular Automata, Chaos Theory, Fractals

### Parameters

Simulations define their parameters using `ParameterDef`, enabling automatic UI generation:

```rust
ParameterDef::Float {
    id: "gravity",
    name: "Gravity",
    min: 0.0,
    max: 30.0,
    default: 9.8,
    unit: Some("m/s²"),
    ...
}
```

## Adding a New Simulation

1. Create a new module in the appropriate domain crate (e.g., `simulations/classical_mechanics/src/my_sim.rs`)
2. Define components, resources, and systems following ECS patterns
3. Implement the `Simulation` trait
4. Export from the crate's `lib.rs`
5. Register the plugin in the application

See `simulations/classical_mechanics/src/particle_system.rs` as a reference implementation.

## Running

```bash
# Desktop (native)
cargo run -p entropy_zero_web

# Web (requires trunk)
cd apps/web && trunk serve
```
