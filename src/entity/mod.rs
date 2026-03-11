use bevy::prelude::*;

use crate::ui::chat::GameMessageEvent;

pub mod dash;
pub mod movement;

#[derive(Component)]
pub struct Entity;

#[derive(Component)]
pub struct Health {
    pub value: i32,
    pub max: i32,
    pub regen_timer: Timer,
    pub difference: i32,
}

#[derive(Component)]
pub struct Stamina {
    pub value: i32,
    pub max: i32,
    pub regen_timer: Timer,
    pub difference: i32,
}

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    movement::handle_movement,
                    dash::on_dash_start,
                    dash::handle_dash,

                    regen_health,
                    regen_stamina,
            ).chain());
    }
}

pub fn regen_health(
    time: Res<Time>,
    mut q: Query<&mut Health>
) {
    for mut health in q.iter_mut() {
        if health.value < health.max && health.difference == 0 {
            health.regen_timer.tick(time.delta());

            if health.regen_timer.just_finished() {
                health.regen_timer.reset();
                health.value += 1;
            }
        } else {
            health.value += health.difference;
            health.difference = 0;
        }

        if health.value > health.max { health.value = health.max; }
    }
}

pub fn regen_stamina(
    time: Res<Time>,
    mut q: Query<(&mut Stamina, &mut Health)>,
    mut chat: MessageWriter<GameMessageEvent>,
) {
    for (mut stamina, mut health) in q.iter_mut() {
        if stamina.value < stamina.max && stamina.difference == 0 {
            stamina.regen_timer.tick(time.delta());

            if stamina.regen_timer.just_finished() {
                stamina.regen_timer.reset();
                stamina.difference = 1;
            }
        } else {
            if stamina.value + stamina.difference <= 0 {
                health.difference -= 1;
                chat.write(GameMessageEvent {
                    text: "You're out of stamina!".to_string(),
                    color: Color::srgb(1.0, 0.0, 0.0)
                });
            } else {
                stamina.value += stamina.difference;
            }

            stamina.difference = 0;
        }

        if stamina.value > stamina.max { stamina.value = stamina.max; }
    }
}