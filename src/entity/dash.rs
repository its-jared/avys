use bevy::prelude::*;

use crate::{animation::AnimationTimer, entity::{DASH_STAMINA_USAGE, Stamina, movement::MovementStats}};

// speed_modifier is multiplied
// into movement speed found in
// the MovementStats struct.
#[derive(Component)]
pub struct DashStats {
    pub speed_modifier: f32,
    pub dash_timer: Timer,
    pub dash_direction: Vec3,
}

// Handle that is added when an
// entity wants to dash.
#[derive(Component)]
pub struct Dashing;

pub fn on_dash_start(
    mut q: Query<(&MovementStats, &mut DashStats), Added<Dashing>>,
) {
    for (movement_stats, mut dash_stats) in q.iter_mut() {
        dash_stats.dash_timer.reset();
        dash_stats.dash_direction = if movement_stats.direction == Vec3::ZERO {
            movement_stats.previous_direction
        } else {
            movement_stats.direction
        };
    }
}

pub fn handle_dash(
    mut c: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &MovementStats, &mut DashStats, &mut AnimationTimer, &mut Transform, &mut Stamina), With<Dashing>>,
) {
    for (entity, movement_stats, mut dash_stats, mut animation_timer, mut transform, mut stamina) in q.iter_mut() {
        let delta = time.delta_secs();
        let future_stamina = stamina.value - DASH_STAMINA_USAGE;
        dash_stats.dash_timer.tick(time.delta());

        if dash_stats.dash_timer.just_finished() {
            c.entity(entity).remove::<Dashing>();
            dash_stats.dash_direction = Vec3::ZERO;

            if future_stamina >= 0 { stamina.difference -= DASH_STAMINA_USAGE; }
        }

        if dash_stats.dash_direction != Vec3::ZERO && future_stamina >= 0 {
            animation_timer.0.unpause();
            let speed = movement_stats.running_speed * dash_stats.speed_modifier;

            transform.translation += 
                dash_stats.dash_direction * delta * speed;
        } else { animation_timer.0.pause(); }
    }
}