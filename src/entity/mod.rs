use bevy::prelude::*;

use crate::ui::chat::{GameMessageEvent, PlayerDiedEvent};

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

pub const DASH_STAMINA_USAGE: i32 = 5;
pub const RUN_STAMINA_USAGE: i32 = 1;
pub const HEALTH_REGEN_STAMINA_USAGE: i32 = 1;

pub const EXHAUSTION_DAMAGE: i32 = 1;

pub const STAMINA_REGEN: i32 = 1;
pub const HEALTH_REGEN: i32 = 1;

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
    mut q: Query<(&mut Health, &mut Stamina)>,
    mut chat: MessageWriter<PlayerDiedEvent>,
) {
    for (mut health, mut stamina) in q.iter_mut() {
        health.regen_timer.tick(time.delta());

        if health.value < health.max 
            && health.difference == 0
            && health.regen_timer.just_finished() {
                health.regen_timer.reset();
                health.value += HEALTH_REGEN;
                stamina.difference -= HEALTH_REGEN_STAMINA_USAGE;
        } else {
            if health.value <= 0 {
                chat.write(PlayerDiedEvent { player_name: "Player".to_string() });
            }

            health.value += health.difference;
            health.difference = 0;
        }

        if health.value > health.max { health.value = health.max; }
        if health.value < 0 { health.value = 0; }
    }
}

pub fn regen_stamina(
    time: Res<Time>,
    mut q: Query<(&mut Stamina, &mut Health)>,
    mut chat: MessageWriter<GameMessageEvent>,
) {
    for (mut stamina, mut health) in q.iter_mut() {
        stamina.regen_timer.tick(time.delta());

        if stamina.value < stamina.max 
            && stamina.difference == 0
            && stamina.regen_timer.just_finished() {
                stamina.regen_timer.reset();
                stamina.difference = STAMINA_REGEN;
        } else {
            if stamina.value + stamina.difference < 0 {
                health.difference -= EXHAUSTION_DAMAGE;
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
        if stamina.value < 0 { stamina.value = 0; }
    }
}