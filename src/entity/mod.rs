use bevy::prelude::*;

pub mod dash;
pub mod movement;

#[derive(Component)]
pub struct Entity;

#[derive(Component)]
pub struct Health {
    pub value: i32,
    pub max: i32,
    pub regen_timer: Timer,
}

#[derive(Component)]
pub struct Stamina {
    pub value: i32,
    pub max: i32,
    pub regen_timer: Timer,
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
        if health.value < health.max {
            health.regen_timer.tick(time.delta());

            if health.regen_timer.just_finished() {
                health.regen_timer.reset();
                health.value += 1;
            }
        }
    }
}

pub fn regen_stamina(
    time: Res<Time>,
    mut q: Query<&mut Stamina>
) {
    for mut stamina in q.iter_mut() {
        if stamina.value < stamina.max {
            stamina.regen_timer.tick(time.delta());

            if stamina.regen_timer.just_finished() {
                stamina.regen_timer.reset();
                stamina.value += 1;
            }
        }
    }
}