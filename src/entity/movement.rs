use bevy::prelude::*;

use crate::{animation::AnimationTimer, entity::{Stamina, dash::Dashing}};

#[derive(Component)]
pub struct MovementStats {
    pub walking_speed: f32, 
    pub running_speed: f32,
    pub running_stamina_timer: Timer,
    pub direction: Vec3,
    pub previous_direction: Vec3,
    pub is_running: bool,
}

pub fn handle_movement(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut AnimationTimer, &mut MovementStats, &mut Stamina), Without<Dashing>>,
) {
    for (mut transform, mut animation_timer, mut movement_stats, mut stamina) in q.iter_mut() {
        let delta = time.delta_secs();

        if movement_stats.direction != Vec3::ZERO {
            let degrees = 
                if movement_stats.previous_direction.x == -1.0 && movement_stats.previous_direction.y == -1.0     { 135.0f32.to_radians() }
                else if movement_stats.previous_direction.x == 1.0 && movement_stats.previous_direction.y == 1.0  { 315.0f32.to_radians() }
                else if movement_stats.previous_direction.x == -1.0 && movement_stats.previous_direction.y == 1.0 { 45.0f32.to_radians() }
                else if movement_stats.previous_direction.x == 1.0 && movement_stats.previous_direction.y == -1.0 { 225.0f32.to_radians() }
                
                else if movement_stats.previous_direction.x == 1.0  { -90.0f32.to_radians() } 
                else if movement_stats.previous_direction.x == -1.0 { 90.0f32.to_radians() } 

                else if movement_stats.previous_direction.y == -1.0 { -180f32.to_radians() }

                else { 0.0 };
            
            let speed = 
                if movement_stats.is_running && stamina.value > 0 { 
                    movement_stats.running_stamina_timer.tick(time.delta());

                    if movement_stats.running_stamina_timer.just_finished() {
                        movement_stats.running_stamina_timer.reset();
                        stamina.value -= 1;
                    }

                    movement_stats.running_speed
                }
                else { movement_stats.walking_speed };

            animation_timer.0.unpause();
            transform.translation += 
                movement_stats.direction * delta * speed;
            transform.rotation = Quat::from_rotation_z(degrees);
            movement_stats.previous_direction = movement_stats.direction;
        } else { animation_timer.0.pause(); }
    }
}