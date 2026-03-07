use bevy::prelude::*;

use crate::{animation::*, entity::movement::MovementStats};

pub mod input;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Stamina(pub i32);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, input::handle_input);
    }
}

pub fn spawn_player(
    mut c: Commands, 
    a: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = a.load("textures/player_spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 1, last: 2, paused: 0 };

    c.spawn((
        Camera2d::default(),
        Transform::default(),
    ));

    c.spawn((
        Player,
        Health(100),
        Stamina(100),

        Transform::from_scale(Vec3::splat(2.0)),

        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            }
        ),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.35, TimerMode::Repeating)),

        MovementStats {
            speed: 100.0,
            stamina_usage: 1,
            direction: Vec3::ZERO,
        },
    ));
}