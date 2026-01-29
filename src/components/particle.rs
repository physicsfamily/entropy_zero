use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct Particle {
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct ParticleTag;
