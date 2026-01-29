//! Control panel generation from parameter definitions.

use bevy::prelude::*;
use bevy_egui::egui;
use ez_core::parameters::{ParameterDef, ParameterValue};
use std::collections::HashMap;

/// Resource holding current parameter values for a simulation.
#[derive(Resource, Default)]
pub struct SimulationParameters {
    pub values: HashMap<&'static str, ParameterValue>,
}

impl SimulationParameters {
    /// Initialize from parameter definitions.
    pub fn from_defs(defs: &[ParameterDef]) -> Self {
        let mut values = HashMap::new();
        for def in defs {
            let value = match def {
                ParameterDef::Float { id, default, .. } => {
                    (*id, ParameterValue::Float(*default))
                }
                ParameterDef::Int { id, default, .. } => {
                    (*id, ParameterValue::Int(*default))
                }
                ParameterDef::Bool { id, default, .. } => {
                    (*id, ParameterValue::Bool(*default))
                }
                ParameterDef::Vec3 { id, default, .. } => {
                    (*id, ParameterValue::Vec3(Vec3::from_array(*default)))
                }
                ParameterDef::Color { id, default, .. } => {
                    (*id, ParameterValue::Color(Color::srgba(
                        default[0],
                        default[1],
                        default[2],
                        default[3],
                    )))
                }
                ParameterDef::Enum { id, default_index, .. } => {
                    (*id, ParameterValue::Enum(*default_index))
                }
            };
            values.insert(value.0, value.1);
        }
        Self { values }
    }

    /// Get a float parameter value.
    pub fn get_float(&self, id: &str) -> Option<f32> {
        self.values.get(id).and_then(|v| v.as_float())
    }

    /// Get a bool parameter value.
    pub fn get_bool(&self, id: &str) -> Option<bool> {
        self.values.get(id).and_then(|v| v.as_bool())
    }
}

/// Render a control panel for the given parameter definitions.
pub fn render_parameter_panel(
    ui: &mut egui::Ui,
    defs: &[ParameterDef],
    params: &mut SimulationParameters,
) {
    for def in defs {
        match def {
            ParameterDef::Float {
                id,
                name,
                min,
                max,
                unit,
                ..
            } => {
                if let Some(ParameterValue::Float(ref mut value)) = params.values.get_mut(id) {
                    let label = if let Some(u) = unit {
                        format!("{} ({})", name, u)
                    } else {
                        name.to_string()
                    };
                    ui.add(egui::Slider::new(value, *min..=*max).text(label));
                }
            }
            ParameterDef::Int {
                id, name, min, max, ..
            } => {
                if let Some(ParameterValue::Int(ref mut value)) = params.values.get_mut(id) {
                    ui.add(egui::Slider::new(value, *min..=*max).text(*name));
                }
            }
            ParameterDef::Bool { id, name, .. } => {
                if let Some(ParameterValue::Bool(ref mut value)) = params.values.get_mut(id) {
                    ui.checkbox(value, *name);
                }
            }
            _ => {
                // Other parameter types will be implemented as needed
            }
        }
    }
}
