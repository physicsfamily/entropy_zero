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

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk (for web builds)
cargo install trunk
```

### Development (Hot Reload) 🔥

The recommended way to develop:

```bash
# Using Make
make web

# Or using Just (cargo install just)
just web
```

This starts a hot-reloading dev server:
- **Auto-refresh**: Save your code → browser updates in ~2 seconds
- **F12 Console**: See Rust logs directly in browser DevTools
- **egui Panels**: Tweak parameters live without recompiling

### Desktop (Native)

```bash
make native
# or
cargo run -p entropy_zero_web
```

### All Commands

| Command | Description |
|---------|-------------|
| `make web` | Start hot-reload dev server |
| `make native` | Run native desktop version |
| `make check` | Quick compile check |
| `make test` | Run all tests |
| `make clean` | Clean build artifacts |

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

See [docs/developer_guide.md](docs/developer_guide.md) for a comprehensive guide on creating new simulations, including:

- Step-by-step walkthrough
- Component, Resource, and System patterns
- UI integration with egui
- Testing and debugging tips
- Complete example implementations

## License

MIT
