//! Parameter definitions for automatic UI generation.
//!
//! Simulations define their configurable parameters using these types,
//! and the platform generates appropriate UI controls automatically.

use bevy::prelude::*;

/// Definition of a simulation parameter for UI generation.
#[derive(Debug, Clone)]
pub enum ParameterDef {
    /// Floating-point slider
    Float {
        id: &'static str,
        name: &'static str,
        description: &'static str,
        min: f32,
        max: f32,
        default: f32,
        step: Option<f32>,
        unit: Option<&'static str>,
    },

    /// Integer slider
    Int {
        id: &'static str,
        name: &'static str,
        description: &'static str,
        min: i32,
        max: i32,
        default: i32,
    },

    /// Boolean toggle
    Bool {
        id: &'static str,
        name: &'static str,
        description: &'static str,
        default: bool,
    },

    /// 3D vector input
    Vec3 {
        id: &'static str,
        name: &'static str,
        description: &'static str,
        default: [f32; 3],
        unit: Option<&'static str>,
    },

    /// Color picker
    Color {
        id: &'static str,
        name: &'static str,
        description: &'static str,
        default: [f32; 4], // RGBA
    },

    /// Dropdown selection
    Enum {
        id: &'static str,
        name: &'static str,
        description: &'static str,
        options: &'static [&'static str],
        default_index: usize,
    },
}

impl ParameterDef {
    /// Returns the parameter identifier.
    pub fn id(&self) -> &'static str {
        match self {
            Self::Float { id, .. } => id,
            Self::Int { id, .. } => id,
            Self::Bool { id, .. } => id,
            Self::Vec3 { id, .. } => id,
            Self::Color { id, .. } => id,
            Self::Enum { id, .. } => id,
        }
    }

    /// Returns the display name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Float { name, .. } => name,
            Self::Int { name, .. } => name,
            Self::Bool { name, .. } => name,
            Self::Vec3 { name, .. } => name,
            Self::Color { name, .. } => name,
            Self::Enum { name, .. } => name,
        }
    }
}

/// Runtime parameter value.
#[derive(Debug, Clone, Reflect)]
pub enum ParameterValue {
    Float(f32),
    Int(i32),
    Bool(bool),
    Vec3(Vec3),
    Color(Color),
    Enum(usize),
}

impl ParameterValue {
    pub fn as_float(&self) -> Option<f32> {
        match self {
            Self::Float(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        match self {
            Self::Int(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_vec3(&self) -> Option<Vec3> {
        match self {
            Self::Vec3(v) => Some(*v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_id() {
        let p = ParameterDef::Float {
            id: "gravity",
            name: "Gravity",
            description: "Gravitational acceleration",
            min: 0.0,
            max: 20.0,
            default: 9.8,
            step: Some(0.1),
            unit: Some("m/s²"),
        };
        assert_eq!(p.id(), "gravity");
    }
}
