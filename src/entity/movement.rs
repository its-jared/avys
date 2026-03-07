use bevy::prelude::*;

use crate::{animation::AnimationTimer, entity::dash::Dashing};

#[derive(Component)]
pub struct MovementStats {
    pub speed: f32, 
    pub stamina_usage: i32, 
    pub direction: Vec3,
}

pub fn handle_movement(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut AnimationTimer, &MovementStats), Without<Dashing>>,
) {
    for (mut transform, mut animation_timer, movement_stats) in q.iter_mut() {
        let delta = time.delta_secs();

        if movement_stats.direction != Vec3::ZERO {
            animation_timer.0.unpause();
            transform.translation += 
                movement_stats.direction * delta * movement_stats.speed;
        } else { animation_timer.0.pause(); }
    }
}