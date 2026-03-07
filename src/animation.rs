use bevy::prelude::*;

// Indicies are based off of the
// ID inside of the spritesheet.
#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub paused: usize,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut q: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indicies, mut timer, mut sprite) in q.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas 
        {
            atlas.index = if atlas.index == indicies.last {
                indicies.first
            } else {
                atlas.index + 1
            }
        }

        if timer.0.is_paused() && let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = indicies.paused;
        }
    }
}