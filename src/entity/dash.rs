use bevy::prelude::*;

use crate::{animation::AnimationTimer, entity::movement::MovementStats};

// speed_modifier is multiplied
// into movement speed found in
// the MovementStats struct.
#[derive(Component)]
pub struct DashStats {
    pub speed_modifier: f32,
    pub dash_timer: Timer,
}

// Handle that is added when an
// entity wants to dash.
#[derive(Component)]
pub struct Dashing;

pub fn handle_dash(
    mut c: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &MovementStats, &mut DashStats, &mut AnimationTimer, &mut Transform), With<Dashing>>,
) {
    for (entity, movement_stats, mut dash_stats, mut animation_timer, mut transform) in q.iter_mut() {
        let delta = time.delta_secs();
        dash_stats.dash_timer.tick(time.delta());

        if dash_stats.dash_timer.just_finished() {
            c.entity(entity).remove::<Dashing>();
        }

        if movement_stats.direction != Vec3::ZERO {
            animation_timer.0.unpause();

            let speed = if !dash_stats.dash_timer.is_finished() {
                movement_stats.speed * dash_stats.speed_modifier
            } else {
                movement_stats.speed
            };

            transform.translation += 
                movement_stats.direction * delta * speed;
        } else { animation_timer.0.pause(); }
    }
}