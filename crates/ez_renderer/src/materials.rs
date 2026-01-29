//! Common materials for simulations.

use bevy::prelude::*;

/// Creates a standard emissive material.
pub fn emissive_material(base_color: Color, emissive: Color) -> StandardMaterial {
    StandardMaterial {
        base_color,
        emissive: emissive.into(),
        ..default()
    }
}

/// Creates a simple colored material.
pub fn simple_material(color: Color) -> StandardMaterial {
    StandardMaterial {
        base_color: color,
        ..default()
    }
}

/// Common color palette for simulations.
pub mod palette {
    use bevy::prelude::*;

    pub const PARTICLE_BLUE: Color = Color::srgb(0.2, 0.7, 1.0);
    pub const PARTICLE_ORANGE: Color = Color::srgb(1.0, 0.5, 0.2);
    pub const PARTICLE_GREEN: Color = Color::srgb(0.2, 1.0, 0.5);
    pub const FIELD_RED: Color = Color::srgb(1.0, 0.3, 0.3);
    pub const FIELD_BLUE: Color = Color::srgb(0.3, 0.3, 1.0);
    pub const GRID_GRAY: Color = Color::srgba(0.5, 0.5, 0.5, 0.3);
}
