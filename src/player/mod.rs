use bevy::{core_pipeline::{bloom::Bloom, tonemapping::{DebandDither, Tonemapping}}, prelude::*};
use bevy_rapier2d::prelude::*;
use crate::{level, rotate_to_cursor::RotateToCursor};

pub mod movement;
pub mod interaction;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCursor;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(interaction::ActiveItem(3))
            .add_systems(Startup, setup)
            .add_systems(Update, movement::handle_movement)
            .add_systems(Update, (
                interaction::handle_interaction,
                interaction::change_selected_block                    
            ).run_if(resource_exists::<level::Level>));
    }
}

fn setup(
    mut c: Commands,
    a: Res<AssetServer>,
) {
    let center = level::level_to_world_pos(IVec2::new(
        level::TILE_SIZE / 2,
        level::TILE_SIZE / 2,
    ), 2.0);
    
    c.spawn((
        Camera2d::default(),
        Camera {
            hdr: true,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Transform::from_translation(center),
        Bloom {
            intensity: 0.001,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        DebandDither::Enabled,
    ));
    c.spawn((
        Player,
        RotateToCursor,

        KinematicCharacterController {
            translation: Some(Vec2::new(center.x, center.y)),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::ball(16.0),

        Sprite {
            image: a.load("textures/player.png"),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        }
    ));

    c.spawn((
        PlayerCursor,

        Transform::from_translation(
            level::level_to_world_pos(
                level::world_to_level_pos(Vec2::new(center.x, center.y)), 
                1.0)),
        Sprite {
            image: a.load("textures/cursor.png"),
            custom_size: Some(Vec2::new(level::TILE_SIZE as f32, level::TILE_SIZE as f32)),
            ..default()
        }
    ));
}