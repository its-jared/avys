use bevy::prelude::*;

use crate::{animation::*, entity::{Health, Stamina, dash::DashStats, movement::MovementStats}};

pub mod input;

#[derive(Component)]
pub struct Player;

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
        Health {
            value: 25,
            max: 50,
            regen_timer: Timer::from_seconds(2.5, TimerMode::Repeating),
            difference: 0,
        },
        Stamina {
            value: 25,
            max: 25,
            regen_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            difference: 0,
        },

        Transform {
            translation: vec3(0.0, 0.0, 1.0),
            scale: Vec3::splat(3.0),
            ..default()
        },

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
            walking_speed: 150.0,
            running_speed: 300.0,
            running_stamina_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            direction: Vec3::ZERO,
            previous_direction: Vec3::ZERO,
            is_running: false,
        },
        DashStats {
            speed_modifier: 4.0,
            dash_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            dash_direction: Vec3::ZERO,
        },
    ));
}