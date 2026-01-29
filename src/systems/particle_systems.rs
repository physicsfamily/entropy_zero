use bevy::prelude::*;

use crate::components::particle::Particle;
use crate::resources::simulation::{SimulationConfig, SimulationStats};

pub fn update_particles(
    mut query: Query<(&mut Transform, &mut Particle)>,
    config: Res<SimulationConfig>,
    time: Res<Time>,
) {
    if config.paused {
        return;
    }

    let dt = time.delta_seconds() * config.speed_multiplier;
    let bounds = config.bounds;
    let gravity = config.gravity;

    query.par_iter_mut().for_each(|(mut transform, mut particle)| {
        particle.velocity += gravity * dt;
        transform.translation += particle.velocity * dt;

        // Bounce off bounds
        if transform.translation.x.abs() > bounds {
            transform.translation.x = transform.translation.x.signum() * bounds;
            particle.velocity.x *= -0.8;
        }
        if transform.translation.y < -bounds {
            transform.translation.y = -bounds;
            particle.velocity.y *= -0.8;
        }
        if transform.translation.y > bounds {
            transform.translation.y = bounds;
            particle.velocity.y *= -0.8;
        }
        if transform.translation.z.abs() > bounds {
            transform.translation.z = transform.translation.z.signum() * bounds;
            particle.velocity.z *= -0.8;
        }
    });
}

pub fn update_stats(
    query: Query<&Particle>,
    mut stats: ResMut<SimulationStats>,
    time: Res<Time>,
) {
    stats.fps = 1.0 / time.delta_seconds();
    stats.particle_count = query.iter().count();
}
