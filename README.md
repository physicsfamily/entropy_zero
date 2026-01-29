# Entropy Zero

> A world-class scientific simulation platform built with Rust and Bevy.

## Overview

Entropy Zero is a modular, high-performance simulation platform designed to simulate phenomena across all scientific domains—from classical mechanics to quantum physics, economics to epidemiology.

## Features

- 🚀 **High Performance**: Built with Rust and Bevy ECS for 60+ FPS simulations
- 🧩 **Modular Architecture**: Clean separation of core, infrastructure, and simulation layers
- 📊 **Scientific Taxonomy**: Rigorous classification of simulation domains
- 🎮 **WebGPU Rendering**: Hardware-accelerated graphics via wgpu
- 🖼️ **Immediate Mode UI**: In-simulation controls with bevy_egui

## Architecture

```
entropy_zero/
├── crates/                       # Core infrastructure
│   ├── ez_core/                  # Traits, taxonomy, math
│   ├── ez_renderer/              # Camera, grids, materials
│   ├── ez_ui/                    # Control panels, widgets
│   └── ez_physics/               # Forces, integrators
├── simulations/                  # Domain simulations
│   └── classical_mechanics/      # Particle systems, pendulums
└── apps/web/                     # WASM web application
```

## Quick Start

### Desktop

```bash
cargo run -p entropy_zero_web
```

### Web (requires Trunk)

```bash
cd apps/web
trunk serve
```

Then open [http://localhost:8080](http://localhost:8080).

## Simulation Domains

| Domain | Status | Examples |
|--------|--------|----------|
| Classical Mechanics | ✅ | Particle systems, N-body gravity |
| Electromagnetism | 🔜 | Electric fields, magnetic fields |
| Wave Physics | 🔜 | Ripple tank, interference |
| Optics | 🔜 | Ray tracing, lens systems |
| Thermodynamics | 🔜 | Heat diffusion, ideal gas |
| Relativistic Physics | 🔜 | Gravitational waves |
| Quantum Mechanics | 🔜 | Wave functions |
| Economics | 🔜 | Supply-demand, markets |
| Cellular Automata | 🔜 | Game of Life |

## Contributing

See [docs/adding_simulations.md](docs/adding_simulations.md) for how to add new simulations.

## License

MIT
