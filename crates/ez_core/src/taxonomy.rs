//! Simulation taxonomy based on scientific domain hierarchy.
//!
//! This module provides a rigorous classification system for organizing
//! simulations by their scientific domain.

use bevy::prelude::*;

/// Top-level simulation category classification.
///
/// Simulations are organized into major scientific branches:
/// - Physical Sciences (mechanics, electromagnetism, optics, etc.)
/// - Life Sciences (epidemiology, ecology)
/// - Social Sciences (economics, game theory)
/// - Formal Sciences (cellular automata, chaos theory, fractals)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum SimulationCategory {
    // ══════════════════════════════════════════════════════════════
    // Physical Sciences
    // ══════════════════════════════════════════════════════════════
    ClassicalMechanics(ClassicalMechanicsSubdomain),
    Electromagnetism(ElectromagnetismSubdomain),
    WavePhysics(WavePhysicsSubdomain),
    Optics(OpticsSubdomain),
    Thermodynamics(ThermodynamicsSubdomain),
    RelativisticPhysics(RelativisticSubdomain),
    QuantumMechanics(QuantumSubdomain),

    // ══════════════════════════════════════════════════════════════
    // Life Sciences
    // ══════════════════════════════════════════════════════════════
    Epidemiology,
    Ecology,
    Neuroscience,

    // ══════════════════════════════════════════════════════════════
    // Social Sciences
    // ══════════════════════════════════════════════════════════════
    Economics,
    GameTheory,
    SocialNetworks,

    // ══════════════════════════════════════════════════════════════
    // Formal Sciences
    // ══════════════════════════════════════════════════════════════
    CellularAutomata,
    ChaosTheory,
    Fractals,
}

impl SimulationCategory {
    /// Returns the human-readable name of this category.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::ClassicalMechanics(_) => "Classical Mechanics",
            Self::Electromagnetism(_) => "Electromagnetism",
            Self::WavePhysics(_) => "Wave Physics",
            Self::Optics(_) => "Optics",
            Self::Thermodynamics(_) => "Thermodynamics",
            Self::RelativisticPhysics(_) => "Relativistic Physics",
            Self::QuantumMechanics(_) => "Quantum Mechanics",
            Self::Epidemiology => "Epidemiology",
            Self::Ecology => "Ecology",
            Self::Neuroscience => "Neuroscience",
            Self::Economics => "Economics",
            Self::GameTheory => "Game Theory",
            Self::SocialNetworks => "Social Networks",
            Self::CellularAutomata => "Cellular Automata",
            Self::ChaosTheory => "Chaos Theory",
            Self::Fractals => "Fractals",
        }
    }

    /// Returns the parent science branch.
    pub fn science_branch(&self) -> ScienceBranch {
        match self {
            Self::ClassicalMechanics(_)
            | Self::Electromagnetism(_)
            | Self::WavePhysics(_)
            | Self::Optics(_)
            | Self::Thermodynamics(_)
            | Self::RelativisticPhysics(_)
            | Self::QuantumMechanics(_) => ScienceBranch::Physical,

            Self::Epidemiology | Self::Ecology | Self::Neuroscience => ScienceBranch::Life,

            Self::Economics | Self::GameTheory | Self::SocialNetworks => ScienceBranch::Social,

            Self::CellularAutomata | Self::ChaosTheory | Self::Fractals => ScienceBranch::Formal,
        }
    }
}

/// Top-level science branch classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ScienceBranch {
    Physical,
    Life,
    Social,
    Formal,
}

impl ScienceBranch {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Physical => "Physical Sciences",
            Self::Life => "Life Sciences",
            Self::Social => "Social Sciences",
            Self::Formal => "Formal Sciences",
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Subdomain enums
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ClassicalMechanicsSubdomain {
    /// Motion without considering forces (projectiles, orbits)
    Kinematics,
    /// Forces and their effects (N-body, collisions)
    Dynamics,
    /// Fluid behavior (CFD, SPH)
    FluidDynamics,
    /// Solid object mechanics (engines, robotics)
    RigidBody,
    /// Springs, oscillators
    Elasticity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ElectromagnetismSubdomain {
    /// Static electric fields and charges
    Electrostatics,
    /// Static magnetic fields
    Magnetostatics,
    /// Propagating EM waves
    ElectromagneticWaves,
    /// Circuit analysis (SPICE-like)
    Circuits,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum WavePhysicsSubdomain {
    /// Mechanical waves in media (water, sound)
    MechanicalWaves,
    /// Wave superposition and interference
    Interference,
    /// Resonance and standing wave patterns
    StandingWaves,
    /// Doppler effect and wave source motion
    DopplerEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum OpticsSubdomain {
    /// Ray-based optics (lenses, mirrors)
    GeometricOptics,
    /// Diffraction and interference
    WaveOptics,
    /// Light polarization effects
    Polarization,
    /// Fiber optics and waveguides
    GuidedOptics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ThermodynamicsSubdomain {
    /// Conduction, convection, radiation
    HeatTransfer,
    /// Statistical mechanics and entropy
    StatisticalMechanics,
    /// Phase transitions
    PhaseTransitions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum RelativisticSubdomain {
    /// Time dilation, length contraction
    SpecialRelativity,
    /// Curved spacetime, gravitational waves
    GeneralRelativity,
    /// Black hole physics
    BlackHoles,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum QuantumSubdomain {
    /// Schrödinger equation solutions
    WaveFunctions,
    /// Quantum tunneling
    Tunneling,
    /// Two-level systems
    SpinSystems,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_display_name() {
        let cat = SimulationCategory::ClassicalMechanics(ClassicalMechanicsSubdomain::Dynamics);
        assert_eq!(cat.display_name(), "Classical Mechanics");
    }

    #[test]
    fn test_science_branch() {
        let cat = SimulationCategory::Economics;
        assert_eq!(cat.science_branch(), ScienceBranch::Social);
    }
}
